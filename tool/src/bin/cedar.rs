
#[macro_use]
extern crate error_chain;
extern crate clap;

use std::fs;
use std::env;
use std::path::Path;

fn main() {
    run().unwrap()
}

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    links {}

    foreign_links {
        Fmt(::std::fmt::Error);
        Io(::std::io::Error) #[cfg(unix)];
    }

    errors {}
}

enum Command {
    Setup { force: bool },
    Run { release: bool },
}

fn args() -> Option<Command> {
    use clap::{Arg, App, SubCommand};

    let matches = App::new("cedar")
        .version("0.0")
        .author("Tom Schroeder")
        .about("TODO")
        .subcommand(SubCommand::with_name("setup").about("TODO").arg(
            Arg::with_name("force").long("force"),
        ))
        .subcommand(SubCommand::with_name("run").about("TODO").arg(
            Arg::with_name("release").long("release"),
        ))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("setup") {
        return Some(Command::Setup { force: matches.is_present("force") });
    }

    if let Some(matches) = matches.subcommand_matches("run") {
        return Some(Command::Run { release: matches.is_present("release") });
    }

    None
}

fn run() -> Result<()> {
    let command = args().unwrap();

    let home = env::var("HOME").unwrap();

    match command {
        Command::Setup { force } => {
            if !force && Path::new(&format!("{}/.cedar", home)).exists() {
                println!("Already setup!");
                return Ok(());
            }

            fs::create_dir_all(&format!("{}/.cedar/lib", home))?;

            // TODO: copy CEF.framework into .cedar/lib
            // TODO: install_name_tool -id "{{CEF}}/Chromium Embedded Framework" "{{CEF}}/Chromium Embedded Framework"
        }

        Command::Run { release } => {
            // TODO: package .app
        }
    }

    Ok(())
}
