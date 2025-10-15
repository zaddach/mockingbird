# mockingbird

This is a proof of concept on how to use the [Win32 metadata](https://github.com/microsoft/win32metadata) project to generate mocks for the Win32 API.

## Running
```
cargo run -- .\examples\config.yaml
```

This will generate files `IWin32Api.h`, `Win32Api.h`, `MockWin32Api.h`, `Win32Api.cpp`, `MockWin32Api.cpp` in the current directory.