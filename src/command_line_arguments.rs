use clap::Parser;

#[derive(Debug, Parser)]
pub(crate) struct CommandLineArguments {
    /// Pass the input directory path
    #[clap(short = 'i', long)]
    pub(crate) input_directory_path: String,

    /// Pass the configuration directory path
    #[clap(short = 'c', long)]
    pub(crate) configuration_directory_path: String,

    /// Pass the output directory path
    #[clap(short = 'o', long)]
    pub(crate) output_directory_path: String,

    /// Pass the glob pattern for file paths
    #[clap(short = 'p', long)]
    pub(crate) patterns: Vec<String>, // "**/pages/**/*.{ts,tsx}"

    /// Pass the glob antipattern for file paths
    #[clap(short = 'a', long)]
    pub(crate) antipatterns: Vec<String>,

    // Pass the language
    #[clap(short = 'l', long)]
    pub(crate) language: String,
}
