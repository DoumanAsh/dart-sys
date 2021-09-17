# dart-sys

[![Crates.io](https://img.shields.io/crates/v/dart-sdk-sys.svg)](https://crates.io/crates/dart-sdk-sys)
[![Documentation](https://docs.rs/dart-sdk-sys/badge.svg)](https://docs.rs/crate/dart-sdk-sys/)
[![Build](https://github.com/DoumanAsh/dart-sys/workflows/Rust/badge.svg)](https://github.com/DoumanAsh/dart-sys/actions?query=workflow%3ARust)

Bindings to dart FFI.

Crate version corresponds to Dart SDK [release](https://github.com/dart-lang/sdk/releases)

## Use cases

### General requirements

- Build your rust code as `cdylib` lib

### Flutter application

- When building for mobile device, you need to build with correct target (i.e. for `android-arm64` your rust target must be `aarch64-linux-android`)

- Place rust library into `android/app/src/main/jniLibs` accordingly to your target (i.e. for `android-arm64` you need to place it inside `arm64-v8a`)

- Build flutter application;

- Refer to `Dart application` for next steps. Flutter embeds your library inside APK so you can refer to it by just library full name.

### Dart application

- Dart FFI provides API to load C shared libraries: `ffi.DynamicLibrary.open(<path to shared library>)`;

- Once library successfully loaded, returned object can be used to lookup function pointers.

Given following rust function:

```rust
#[no_mangle]
pub unsafe extern "C" fn handle(rd: *const c_char) -> i8 {
    //Do something
    return 0;
}
```

You can access its pointer in following way

```dart
import 'dart:ffi' as ffi;
// External package https://pub.dev/packages/ffi
import 'package:ffi/ffi.dart' as ffiUtils;

typedef NativeFunctionT = ffi.Int8 Function(ffi.Pointer<ffiUtils.Utf8>);
typedef DartFunctionT = int Function(ffi.Pointer<ffiUtils.Utf8>);

final d = ffi.DynamicLibrary.open("my_shared_lib_name.so");
final DartFunctionT sendDataToRust = d.lookupFunction<RustRxNativeFunc, RustRxDartFunc>("rx_handler");

/// Use function to send string data which internally converts it to C compatible char buffer.
void sendNative(DartFunctionT ptr, String d) {
    final data = d.toNativeUtf8();
    sendDataToRust(data);
    ffiUtils.calloc.free(data);
}

```

## How-to update to new SDK version

1. Update `version` in `Cargo.toml` to be equal to desired version of SDK
2. Run `cargo build --features download-sources,build-bindings`
3. Optionally run `rustfmt src/lib.rs` to make it pretty

4. Commit and publish
