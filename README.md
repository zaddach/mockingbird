# mockingbird

This is a proof of concept on how to use the [Win32 metadata](https://github.com/microsoft/win32metadata) project to generate code. The Win32 metadata
project ingests Win32 api headers and provides type information in the winmd format. Winmd is basically a CLR (Common Language Runtime) executable without code, just type metadata.

Other projections exist for programming languages such as Rust. This project aims to build a C++ API from the metadata (Though the templating engine should be able to produce other languages than C++).

An example is given for automatically producing mocks for the Win32 API.

## Running
```
cargo run -- ./examples/mocking/mocking.yaml
```

This will generate files `IWin32Api.h`, `Win32Api.h`, `MockWin32Api.h`, `Win32Api.cpp`, `MockWin32Api.cpp` in the current directory.