use core::panic;
use std::process::exit;

use clap::{command, Arg, Command};
use repositories::store::{self, AppStore, Store};

mod repositories;
mod data;

trait OutputMapper {
    fn print_output(&self, output_json: bool);
}

impl OutputMapper for Result<(), String> {
    fn print_output(&self, _output_json: bool) {
        match self {
            Ok(_) => {
                println!("Success!");
                exit(0);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                exit(1);
            }
        }
    }
}

fn main() {
    let notes = Command::new("notes")
        .about("Patches the release notes for a certain platform.")
        .arg(
            Arg::new("locale")
                .help("The locale of the release notes")
                .required(true)
                .short('l')
                .long("locale"),
        )
        .arg(
            Arg::new("message")
                .help("The message to be patched")
                .required(true)
                .short('m')
                .long("message"),
        )
        .arg(
            Arg::new("version-name")
                .help("The version name to be patched")
                .required(true)
                .short('n')
                .long("version-name"),
        );

    let version = Command::new("version")
        .about("Creates a new version on the specified store.")
        .arg(
            Arg::new("version-name")
                .help("The version name to be created")
                .required(true)
                .short('n')
                .long("version-name"),
        )
        .arg(
            Arg::new("fail")
                .help("Fails the command if the version already exists.")
                .action(clap::ArgAction::SetTrue)
                .short('f'),
        );

    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(notes)
        .subcommand(version)
        .arg(
            Arg::new("platform")
                .help("The platform to patch the release notes")
                .required(true)
                .short('p')
                .long("platform")
                .value_parser(["android", "ios"]),
        )
        .arg(
            Arg::new("ios-app-id")
                .help("The App Store Connect App ID")
                .long("ios-app-id")
                .required_if_eq("platform", "ios"),
        )
        .arg(
            Arg::new("issuer-id")
                .help("The issuer ID, UUID of the App Store Connect API key")
                .long("issuer-id")
                .required_if_eq("platform", "ios"),
        )
        .arg(
            Arg::new("key-path")
                .help("The path to the service account key file. Must be named AuthKey_{KEY_ID}.p8")
                .short('k')
                .long("key-path")
                .required_if_eq("platform", "ios"),
        )
        .arg(
            Arg::new("package-name")
                .help("The package name of the app")
                .long("package-name")
                .required_if_eq("platform", "android"),
        )
        .arg(
            Arg::new("google-key-path")
                .help("The path to the service account key file")
                .short('g')
                .long("google-key-path")
                .required_if_eq("platform", "android"),
        )
        .arg(
            Arg::new("track")
                .help("The track to update")
                .long("track")
                .required_if_eq("platform", "android")
                .value_parser(["internal", "alpha", "beta", "production"]),
        )
        .arg(
            Arg::new("machine")
                .long("machine")
                .help("Outputs in machine readable format (json)")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let platform = matches.get_one::<String>("platform").unwrap().as_str();
    let machine_output = matches.get_flag("machine");

    if machine_output {
        println!("Note: Machine output is not yet implemented.");
        exit(1);
    }

    let ios_store: Option<AppStore> = if platform == "ios" {
        Some(AppStore::new(
            matches.get_one::<String>("key-path").unwrap().to_string(),
            matches.get_one::<String>("issuer-id").unwrap().to_string(),
            matches.get_one::<String>("ios-app-id").unwrap().to_string(),
        ))
    } else {
        None
    };

    let android_store: Option<store::GooglePlay> = if platform == "android" {
        Some(store::GooglePlay::new(
            matches
                .get_one::<String>("google-key-path")
                .unwrap()
                .to_string(),
            matches
                .get_one::<String>("package-name")
                .unwrap()
                .to_string(),
            matches.get_one::<String>("track").unwrap().to_string(),
        ))
    } else {
        None
    };

    match matches.subcommand() {
        Some(("notes", matches)) => {
            let version_name = matches.get_one::<String>("version-name").unwrap();
            let locale = matches.get_one::<String>("locale").unwrap();
            let notes = matches.get_one::<String>("message").unwrap();

            if let Some(mut store) = ios_store {
                store
                    .set_changelog(locale, version_name, notes)
                    .print_output(machine_output);
            } else if let Some(mut store) = android_store {
                store
                    .set_changelog(locale, version_name, notes)
                    .print_output(machine_output);
            }
        }

        Some(("version", matches)) => {
            let version_name = matches.get_one::<String>("version-name").unwrap();
            let fail = matches.get_flag("fail");

            if fail {
                panic!("Flag fail is not yet implemented.")
            }

            if let Some(mut store) = ios_store {
                store
                    .create_version(version_name)
                    .print_output(machine_output);
            } else if let Some(mut store) = android_store {
                store
                    .create_version(version_name)
                    .print_output(machine_output);
            }
        }

        _ => {
            panic!("This should not happen!")
        }
    }
}
