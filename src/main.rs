use clap::Parser;
use glob::glob;
use glob::Pattern;

#[derive(Debug, Parser)]
pub(crate) struct CommandLineArguments {
    /// Pass the glob pattern for file paths
    #[clap(short = 'p', long)]
    pub(crate) pattern: Vec<String>,
    
    /// Pass the group(s) of codemods for execution
    #[clap(short = 'g', long)]
    pub(crate) group: Vec<String>,

    /// Pass the limit for the number of files to inspect
    #[clap(short = 'l', long)]
    pub(crate) limit: Option<u64>,

    /// Pass the limit for the number of files to inspect
    #[clap(short = 'o', long)]
    pub(crate) output_directory_path: Option<String>,
}

fn main() {
    let command_line_arguments = CommandLineArguments::parse();

    let pattern = command_line_arguments.pattern;

    let first_pattern = pattern.first().unwrap();
    let other_patterns: Vec<&String> = pattern
        .iter()
        .filter(|p| *p != first_pattern)
        .collect();

    // dbg!(&other_patterns);

    for entry in glob(first_pattern).unwrap() {
        match entry {
            Ok(path) => {
                for other_pattern in &other_patterns {
                    let px = Pattern::new(&other_pattern).unwrap();

                    if px.matches_path(&path) {
                        println!("NO {:?}", path.display())
                    }
                }

                println!("{:?}", path.display())
            },

            // if the path matched but was unreadable,
            // thereby preventing its contents from matching
            Err(e) => println!("{:?}", e),
        }
    }
}
