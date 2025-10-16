mod args;
mod config;
mod constants;

mod wrappers;

use std::{io::Seek, path::Path};

use clap::Parser;

use crate::{constants::TYPE_VARIANTS, wrappers::MethodDefWrapper};


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
    let conf_dir = args.api.parent().unwrap();

    let mut tera = tera::Tera::default();
    let mut files = Vec::new();

    for include_path in & config.includes {
        let path = conf_dir.join(include_path);
        tera.add_template_file(& path, Some(include_path)).unwrap();
    }

    for template in & config.templates {
        let path = conf_dir.join(template);
        tera.add_template_file(& path, Some(template)).unwrap();
        files.push(template);
    }

    let mut functions = Vec::new();
    let namespaces = config.api.iter().map(|(namespace, functions)| (index.expect(namespace, "Apis"), functions.clone())).collect::<Vec<_>>();
    for (namespace, function_names) in & namespaces {
        let namespace_functions = namespace.methods().filter(|func| function_names.iter().any(|name| name == func.name()));
        functions.extend(namespace_functions);
    }
    let functions = functions.into_iter().map(MethodDefWrapper).collect::<Vec<_>>();

    let mut context = tera::Context::new();
    context.insert("functions", &functions);
    context.insert("type_aliases", &config.type_aliases);

    let type_map = TYPE_VARIANTS.iter().cloned().collect::<std::collections::HashMap<_, _>>();
    context.insert("TYPES", &type_map);

    let cur_dir = std::env::current_dir().unwrap();
    let output_dir = config.output_dir.as_ref().unwrap_or(& cur_dir);

    for file in & files {
        let output = tera.render(file, &context).unwrap();
        let output_file_path = output_dir.join(Path::new(file).file_name().unwrap());
        std::fs::write(& output_file_path, output).unwrap();
    }
}
