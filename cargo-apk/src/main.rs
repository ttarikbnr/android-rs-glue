extern crate rustc_serialize;
extern crate term;
extern crate toml;
extern crate clap;

use clap::{App, Arg};
use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;
use std::process::Command;

mod build;
mod config;
mod install;
mod termcmd;

fn main() {
    let matches = App::new( "    Cargo Apk")
                    .about( "    About:  Apk builder subcommand for cargo")
                    .author("    Author: https://github.com/tomaka")
                    .arg(Arg::with_name("release")
                            .long("release")
                            .help("release build"))
                    .arg(Arg::with_name("install")
                            .help("install apk to the device(by using adb)"))
                    .arg(Arg::with_name("bin")
                            .long("bin")
                            .help("binary options")
                            .value_name("TARGET")
                        )
                            // .possible_values("") // TODO
                    .get_matches();

    let current_manifest = current_manifest_path();

    // Fetching the configuration for the build.
    let mut config = config::load(&current_manifest);
    config.release = matches.is_present("release");
    config.target  = matches.value_of("bin").map(|x| x.to_owned());

    if matches.is_present("install"){
        install::install(&current_manifest, &config);
    } else {
        build::build(&current_manifest, &config);
    }
}

/// Returns the path of the `Cargo.toml` that we want to build.
fn current_manifest_path() -> PathBuf {
    let output = Command::new("cargo").arg("locate-project").output().unwrap();

    if !output.status.success() {
        if let Some(code) = output.status.code() {
            exit(code);
        } else {
            exit(-1);
        }
    }

    #[derive(RustcDecodable)]
    struct Data { root: String }
    let stdout = String::from_utf8(output.stdout).unwrap();
    let decoded: Data = rustc_serialize::json::decode(&stdout).unwrap();
    Path::new(&decoded.root).to_owned()
}
