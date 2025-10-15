#[derive(clap::Parser)]
pub struct Args {
    /// Win32 metadata file.
    /// 
    /// You can download the "Win32Metadata" nuget package from [here](https://www.nuget.org/packages/Microsoft.Windows.SDK.Win32Metadata/65.0.8-preview),
    /// rename it to `.zip` and extract the winmd metadata file from it.
    /// 
    /// If no metadata file is provided, the tool will try to download and extract the file from a nuget package.
    #[clap(long)]
    pub win32_metadata: Option<std::path::PathBuf>,
    #[clap(long, default_value = ".")]
    pub include_dir: std::path::PathBuf,
    #[clap(long, default_value = ".")]
    pub source_dir: std::path::PathBuf,
    #[clap(short, long)]
    pub force: bool,
    /// API config file to build a Win32 mock for.
    pub api: std::path::PathBuf,
}