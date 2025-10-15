mod args;
mod config;

use std::io::Seek;

use clap::Parser;
use windows_metadata::{reader::MethodDef, HasAttributes, ParamAttributes, Type};

fn print_type(param: &Type) -> String {
    match param {
        Type::Bool => "bool".to_string(),
        Type::I8 => "int8_t".to_string(),
        Type::U8 => "uint8_t".to_string(),
        Type::I16 => "int16_t".to_string(),
        Type::U16 => "uint16_t".to_string(),
        Type::I32 => "int32_t".to_string(),
        Type::U32 => "uint32_t".to_string(),
        Type::I64 => "int64_t".to_string(),
        Type::U64 => "uint64_t".to_string(),
        Type::F32 => "float".to_string(),
        Type::F64 => "double".to_string(),
        Type::Char => "char".to_string(),
        Type::ISize => "intptr_t".to_string(),
        Type::USize => "size_t".to_string(),
        Type::Name(name) =>name.name.to_string(),
        Type::PtrMut(ty, _) => format!("{} *", print_type(ty)),
        Type::PtrConst(ty, _) => format!("{} const *", print_type(ty)),
        Type::ConstRef(ty) => format!("{} const&", print_type(ty)),
        _ => panic!("Unhandled type: {param:?}"),
    }
}

fn print_param_flags(flags: &ParamAttributes) -> String {
    if flags.contains(ParamAttributes::In) && flags.contains(ParamAttributes::Out) {
        "_Inout_".to_string()
    }
    else if flags.contains(ParamAttributes::In) && flags.contains(ParamAttributes::Optional) {
        "_In_opt_".to_string()
    }
    else if flags.contains(ParamAttributes::Out) && flags.contains(ParamAttributes::Optional) {
        "_Out_opt_".to_string()
    }
    else if flags.contains(ParamAttributes::In) {
        "_In_".to_string()
    }
    else if flags.contains(ParamAttributes::Out) {
        "_Out_".to_string()
    }
    else {
        "".to_string()
    }
}

fn print_function_declaration(func: &MethodDef, funcname_prefix: Option<&str>) -> String {
    let signature = func.signature(&[]);
    let args = std::iter::zip(func.params().skip(1), signature.types).map(|(param, ty)| {
        let constant = param.has_attribute("ConstAttribute");
        format!("{} {}{} {}", print_param_flags(& param.flags()), if constant { "const " } else { "" }, print_type(& ty), param.name())
    }).collect::<Vec<_>>();
    let args = args.join(", ");
    let return_type = print_type(& signature.return_type);

    format!("{} {}{}({})", return_type, funcname_prefix.unwrap_or(""), func.name(), args)
}

fn print_function_pure_virtual_method(func: &MethodDef) -> String {
    let declaration = print_function_declaration(func, None);
    format!("virtual {declaration} = 0;")
}

fn print_overriding_methods(func: &MethodDef) -> String {
    let declaration = print_function_declaration(func, None);
    format!("{declaration} override;")
}

fn print_forwarding_function_definition(func: &MethodDef) -> String {
    let declaration = print_function_declaration(func, Some("Win32Api::"));
    let func_name = func.name();
    let args = func.params().skip(1).map(|param| param.name().to_string()).collect::<Vec<_>>().join(", ");

    format!(r#"{declaration} {{
    return {func_name}({args});
}}"#)
}

fn print_mock_function_definition(func: &MethodDef) -> String {
    let func_name = func.name();
    let return_type = print_type(& func.signature(&[]).return_type);
    let mut arg_types = Vec::new();
    for (param, ty) in std::iter::zip(func.params().skip(1), func.signature(&[]).types) {
        arg_types.push(format!("{}{}", if param.has_attribute("ConstAttribute") { "const " } else { "" }, print_type(& ty)));
    }
    let arg_types = arg_types.join(", ");
    format!(r#"MOCK_METHOD({return_type}, {func_name}, ({arg_types}), (override));"#)
}

fn get_metadata(path: & Option<std::path::PathBuf>) -> windows_metadata::reader::Index {
    const URL : &str = "https://globalcdn.nuget.org/packages/microsoft.windows.sdk.win32metadata.65.0.8-preview.nupkg?packageVersion=65.0.8-preview";
    const METADATA_FILE: &str = "Windows.Win32.winmd";

    if let Some(path) = path {
        return windows_metadata::reader::Index::read(path).unwrap();
    }

    if std::fs::exists(METADATA_FILE).unwrap() {
        return windows_metadata::reader::Index::read(METADATA_FILE).unwrap();
    }

    let mut response = reqwest::blocking::get(URL).unwrap();
    let mut temp_zip = tempfile::NamedTempFile::new().unwrap();
    response.copy_to(&mut temp_zip).unwrap();
    temp_zip.seek(std::io::SeekFrom::Start(0)).unwrap();
    let mut zip = zip::ZipArchive::new(& temp_zip).unwrap();
    let mut metadata_file = std::fs::File::create(METADATA_FILE).unwrap(); 
    std::io::copy(&mut zip.by_path(METADATA_FILE).unwrap(), &mut metadata_file).unwrap();
    windows_metadata::reader::Index::read(METADATA_FILE).unwrap()
}


fn main() {
    let args = args::Args::parse();

    let index = get_metadata(& args.win32_metadata);
    let config = config::Config::from_file(& args.api).unwrap();

    let mut pure_virtual_methods = Vec::new();
    let mut override_methods = Vec::new();
    let mut impl_methods = Vec::new();
    let mut mock_methods = Vec::new();
    
    for (namespace, functions) in &config.api {
        let namespace = index.expect(namespace, "Apis");

        for func in namespace.methods() {
            if functions.iter().any(|func_name| func.name() == func_name) {
                pure_virtual_methods.push(print_function_pure_virtual_method(&func));
                override_methods.push(print_overriding_methods(&func));
                impl_methods.push(print_forwarding_function_definition(&func));
                mock_methods.push(print_mock_function_definition(&func));
            }

        }
    }
    
    let pure_virtual_methods = pure_virtual_methods.join("\n    ");
    let iheader = format!(r#"// This file has been automatically generated and should not be edited manually
#pragma once
#include <cstdint>
#include <cstddef>
#include <windows.h>

class IWin32Api {{
public:
    {pure_virtual_methods}
}};
"#);

    let override_methods = override_methods.join("\n    ");
    let header = format!(r#"// This file has been automatically generated and should not be edited manually
#pragma once

#include "IWin32Api.h"

class Win32Api : public IWin32Api {{
public:
    Win32Api();
    virtual ~Win32Api();

    {override_methods}
}};
"#);

    let impl_methods = impl_methods.join("\n\n");
    let source = format!(r#"// This file has been automatically generated and should not be edited manually

{impl_methods}
"#);

    let mock_methods = mock_methods.join("\n    ");
    let mock = format!(r#"// This file has been automatically generated and should not be edited manually
#pragma once

#include "IWin32Api.h"

class MockWin32Api : public IWin32Api {{
public:
    MockWin32Api();
    virtual ~MockWin32Api();

    {mock_methods}
}};
"#);

    let mock_source = r#"// This file has been automatically generated and should not be edited manually
#include "MockWin32Api.h"

MockWin32Api::MockWin32Api() {
}

MockWin32Api::~MockWin32Api() {
}
"#;



    std::fs::write(args.include_dir.join("IWin32Api.h"), iheader).unwrap();
    std::fs::write(args.include_dir.join("Win32Api.h"), header).unwrap();
    std::fs::write(args.source_dir.join("Win32Api.cpp"), source).unwrap();
    std::fs::write(args.include_dir.join("MockWin32Api.h"), mock).unwrap();
    std::fs::write(args.source_dir.join("MockWin32Api.cpp"), mock_source).unwrap();
}
