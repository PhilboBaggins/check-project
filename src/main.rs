extern crate cargo_toml;
extern crate toml;
extern crate clap;
extern crate ini;

use std::fs;
use cargo_toml::Manifest;
use std::fs::read;
use clap::{App, Arg};
use std::path::Path;
use ini::Ini;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let five = 5;
        assert!(5 == five);
    }
}

fn check_cargo_project(proj_path: &str, verbose: u64) {
    let cargo_toml_path = Path::new(proj_path).join("Cargo.toml");
    let cargo_toml_data = Manifest::<toml::Value>::from_slice_with_metadata(&read(cargo_toml_path).unwrap()).unwrap();
    let cargo_toml_package = cargo_toml_data.package.as_ref().unwrap();

    let git_config_path = Path::new(proj_path).join(".git").join("config");
    if let Ok(conf) = Ini::load_from_file(git_config_path) {
        let section = conf.section(Some("remote \"origin\"".to_owned())).unwrap();

        if verbose >= 1 {
            println!("URL: {}", section.get("url").unwrap());
        }

        if let Some(repo_path) = &cargo_toml_package.repository {
            println!("repo_path: {}", repo_path);
        }
    }

    // TODO: Add check for CI badge:
    // * https://github.com/sodiumoxide/sodiumoxide/commit/9a9ab1d4347ad15ae545019cd2355cda723938c5
    // * https://doc.rust-lang.org/cargo/reference/manifest.html
}

fn main() {
    let matches = App::new("check-project")
        .version("1.0")
        .about("???????????????????????????????????????")
        .author("Phil B.")
        .arg(Arg::with_name("path")
            .help("Path to project")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .multiple(true)
            .help("Enable verbose output"))
        .get_matches();

    let proj_path = matches.value_of("path").unwrap();
    let verbose = matches.occurrences_of("verbose");

    // TODO: Check for .git dir, .gitignore file, etc

    let projects_top_level_contents = fs::read_dir(proj_path).unwrap_or_else(|error| {
        eprintln!("{}", error.to_string());
        ::std::process::exit(1);
    });
    for path in projects_top_level_contents {
        // TODO: Consider files and directories differently
        // TODO: Get rid of some of these unwrap() calls

        if let Some(file_name) = path.unwrap().path().file_name() {
            match file_name.to_str() {
                Some("Cargo.toml") => check_cargo_project(proj_path, verbose),
                Some(file_name) if verbose >= 2 => {
                    println!("Looking for project files ... ignoring {}", file_name)
                },
                _ => {
                    // ????
                }
            }
        }
    }
}
