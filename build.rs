fn download_sources() {
    #[cfg(feature = "download-sources")]
    {
        const VERSION: &str = env!("CARGO_PKG_VERSION");

        use std::fs;
        use std::path::{Path, PathBuf};

        fn copy<U: AsRef<Path>, V: AsRef<Path>>(from: U, to: V) -> Result<(), std::io::Error> {
            let mut stack = Vec::new();
            stack.push(PathBuf::from(from.as_ref()));

            let output_root = PathBuf::from(to.as_ref());
            let input_root = PathBuf::from(from.as_ref()).components().count();

            while let Some(working_path) = stack.pop() {
                // Generate a relative path
                let src: PathBuf = working_path.components().skip(input_root).collect();

                // Create a destination if missing
                let dest = if src.components().count() == 0 {
                    output_root.clone()
                } else {
                    output_root.join(&src)
                };
                if fs::metadata(&dest).is_err() {
                    fs::create_dir_all(&dest)?;
                }

                for entry in fs::read_dir(working_path)? {
                    let entry = entry?;
                    let path = entry.path();
                    if path.is_dir() {
                        stack.push(path);
                    } else {
                        match path.file_name() {
                            Some(filename) => {
                                let dest_path = dest.join(filename);
                                fs::copy(&path, &dest_path)?;
                            }
                            None => {
                                panic!("Cannot read: {:?}", path);
                            }
                        }
                    }
                }
            }

            Ok(())
        }

        let _ = fs::remove_dir_all("sdk");

        let download = std::process::Command::new("git").arg("clone")
                                                        .arg("--depth")
                                                        .arg("1")
                                                        .arg("--branch")
                                                        .arg(VERSION)
                                                        .arg("--single-branch")
                                                        .arg("git@github.com:dart-lang/sdk.git")
                                                        .output()
                                                        .expect("To download sources");

        if !download.status.success() {
            let stderr = String::from_utf8(download.stderr).expect("stderr is not utf-8");
            panic!("Failed to download dart sdk: {}", stderr);
        }

        let source = ["sdk", "runtime", "include"].iter().collect::<PathBuf>();
        let _ = fs::remove_dir_all("dart");
        copy(source, "dart").expect("To copy dart headers");
        let _ = fs::remove_dir_all("sdk");
    }
}

fn generate_lib() {
    #[cfg(feature = "build-bindings")]
    {
        use std::path::PathBuf;

        const PREPEND_LIB: &'static str = "
#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
";

        let out = PathBuf::new().join("src").join("lib.rs");

        let bindings = bindgen::Builder::default().header("dart/dart_native_api.h")
                                                  .header("dart/dart_tools_api.h")
                                                  .raw_line(PREPEND_LIB)
                                                  .parse_callbacks(Box::new(bindgen::CargoCallbacks))
                                                  .generate_comments(false)
                                                  .layout_tests(false)
                                                  .whitelist_type("_*Dart.+")
                                                  .whitelist_function("_*Dart.+")
                                                  .whitelist_var("_*Dart.+")
                                                  .ctypes_prefix("libc")
                                                  .use_core()
                                                  .generate()
                                                  .expect("Unable to generate bindings");

        bindings.write_to_file(out).expect("Couldn't write bindings!");
    }

}

fn main() {
    download_sources();
    generate_lib();
}
