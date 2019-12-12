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

fn check_field_string(field: Option<&String>, friendly_name: &str) {
    if let Some(field) = field {
        if field.contains("?") {
            println!("{}: {}", friendly_name, field);
        }
    } else {
        println!("{}: Not present", friendly_name);
    }
}

fn compare_fields_string(field_1: Option<&String>, field_2: Option<&String>,
    friendly_name_1: &str, friendly_name_2: &str) {
    let not_present = "Not present".to_owned();
    let field_1 = field_1.unwrap_or(&not_present);
    let field_2 = field_2.unwrap_or(&not_present);
    if field_1 != field_2 {
        println!("{} and {} do not match", friendly_name_1, friendly_name_2);
        println!("    {}: {}", friendly_name_1, field_1);
        println!("    {}: {}", friendly_name_2, field_2);
    }
}

fn check_cargo_project(proj_path: &str, _verbose: u64) {
    let cargo_toml_path = Path::new(proj_path).join("Cargo.toml");
    let cargo_toml_data = Manifest::<toml::Value>::from_slice_with_metadata(&read(cargo_toml_path).unwrap()).unwrap();
    let cargo_toml_package = cargo_toml_data.package.as_ref().unwrap();

    // Check Cargo.toml fields
    // TODO: &cargo_toml_package.edition
    check_field_string(Some(&cargo_toml_package.version), "Cargo.toml version field");
    check_field_string(cargo_toml_package.description.as_ref(), "Cargo.toml description field");
    check_field_string(cargo_toml_package.repository.as_ref(), "Cargo.toml repository field");
    check_field_string(cargo_toml_package.license.as_ref(), "Cargo.toml license field");

    // .....................
    let git_config_path = Path::new(proj_path).join(".git").join("config");
    if let Ok(conf) = Ini::load_from_file(git_config_path) {
        if let Some(remote_origin_section) = conf.section(Some("remote \"origin\"".to_owned())) {
            compare_fields_string(
                cargo_toml_package.repository.as_ref(),
                remote_origin_section.get("url"),
                "Cargo.toml repository field",
                ".git/config remote origin URL");
        }
    }

    if let Some(_license_info) = &cargo_toml_package.license {
        // TODO: Check for license files that match cargo_toml_package.license
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
