use std::fs::File;
use std::io::Write;
use std::str::FromStr;
use std::{collections::hash_map::DefaultHasher, hash::Hasher, path::PathBuf};

use clap::Parser;
use command_line_arguments::CommandLineArguments;
use glob::Pattern;
use json::object;

use polyglot_piranha::{
    execute_piranha, models::language::PiranhaLanguage,
    models::piranha_arguments::PiranhaArgumentsBuilder,
};

mod command_line_arguments;

pub fn build_byte_hash(bytes: &[u8]) -> u64 {
    let mut hasher = DefaultHasher::new();
    hasher.write(bytes);
    return hasher.finish();
}

pub fn build_output_path(
    output_directory_path: &String,
    new_page_path: &String,
    extension: &str,
) -> Option<String> {
    let hash = build_byte_hash(new_page_path.as_bytes());

    let file_name = format!("{:x}.{}", hash, extension);

    let output_path_buf: PathBuf = [output_directory_path, &file_name].iter().collect();

    output_path_buf.to_str().map(|s| s.to_string())
}

fn main() {
    let command_line_arguments = CommandLineArguments::try_parse();
    let command_line_arguments = command_line_arguments.unwrap();

    let patterns = command_line_arguments
        .patterns
        .iter()
        .map(|p| Pattern::new(p).unwrap())
        .collect();

    let antipatterns: Vec<Pattern> = command_line_arguments
        .antipatterns
        .iter()
        .map(|p| Pattern::new(p).unwrap())
        .collect();

    let piranha_arguments = PiranhaArgumentsBuilder::default()
        .path_to_codebase(command_line_arguments.input_directory_path)
        .path_to_configurations(command_line_arguments.configuration_directory_path)
        .include(patterns)
        .exclude(antipatterns)
        .language(PiranhaLanguage::from_str("ts").unwrap())
        .dry_run(true)
        .build();

    let summaries = execute_piranha(&piranha_arguments);

    for summary in summaries {
        let path = summary.path();
        let path_buf: PathBuf = path.into();

        let extension = path_buf.extension().unwrap_or_default().to_str().unwrap();

        let content = summary.content();

        let output_path = build_output_path(
            &command_line_arguments.output_directory_path,
            path,
            extension,
        );

        if output_path.is_none() {
            let error_message = object! {
                message: "Could not build the output path",
                path: path.clone(),
            };

            eprintln!("{}", json::stringify(error_message));

            continue;
        }

        let output_path = output_path.unwrap();

        let file = File::create(&output_path);

        if file.is_err() {
            let error_message = object! {
                message: file.unwrap_err().to_string(),
                output_path: output_path,
            };

            eprintln!("{}", json::stringify(error_message));

            continue;
        }

        let mut file = file.unwrap();

        let result = file.write_all(content.as_bytes());

        if result.is_err() {
            let error_message = object! {
                message: result.unwrap_err().to_string(),
                output_path: output_path,
            };

            eprintln!("{}", json::stringify(error_message));

            continue;
        }

        let rewrite_message = object! {
            k: 3,
            i: path.to_owned(),
            o: output_path,
            c: ""
        };

        println!("{}", json::stringify(rewrite_message));
    }

    let finish_message = object! {
        k: 2,
    };

    println!("{}", json::stringify(finish_message));
}
