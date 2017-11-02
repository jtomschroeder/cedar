
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
    Setup { force: bool, local: Option<String> },
    Run { release: bool },
}

fn args() -> Option<Command> {
    use clap::{Arg, App, SubCommand};

    let matches = App::new("cedar")
        .version("0.0")
        .author("Tom Schroeder")
        .about("TODO")
        .subcommand(
            SubCommand::with_name("setup")
                .about("TODO")
                .arg(Arg::with_name("force").long("force"))
                .arg(
                    Arg::with_name("local")
                        .long("local")
                        .help("TODO")
                        .value_name("ARCHIVE")
                        .takes_value(true),
                ),
        )
        .subcommand(SubCommand::with_name("run").about("TODO").arg(
            Arg::with_name("release").long("release"),
        ))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("setup") {
        return Some(Command::Setup {
            force: matches.is_present("force"),
            local: matches.value_of("local").map(String::from),
        });
    }

    if let Some(matches) = matches.subcommand_matches("run") {
        return Some(Command::Run { release: matches.is_present("release") });
    }

    None
}

macro_rules! sh {
    ( $( $cmd:tt )* ) => {{
        $crate::execute_with("sh", &format!($( $cmd )*))
    }};
}

fn execute_with(shell: &str, cmd: &String) -> std::process::ExitStatus {
    use std::process::{Command, Stdio};

    let mut command = {
        let mut command = Command::new(shell);
        command.arg("-c").arg(cmd);
        command.stdout(Stdio::inherit()).stderr(Stdio::inherit());
        command
    };

    let mut command = command.spawn().unwrap();
    command.wait().unwrap()
}

fn run() -> Result<()> {
    let command = args().unwrap();

    let home = env::var("HOME").unwrap();
    let vault = format!("{}/.cedar", home);

    match command {
        Command::Setup { force, local } => {
            if !force && Path::new(&vault).exists() {
                println!("Already setup!");
                return Ok(());
            }

            fs::create_dir_all(&vault)?;

            let archive = match local {
                Some(path) => path,
                None => unimplemented!(),
            };

            sh!("tar -xf {} -C {}", archive, vault);

            let cef = format!(
                "{}/lib/'Chromium Embedded Framework.framework/Chromium Embedded Framework'",
                vault
            );
            sh!("install_name_tool -id {} {}", cef, cef);
        }

        Command::Run { release } => {
            let app = "cedar-test";

            let cef = format!("{}/lib/'Chromium Embedded Framework.framework'", vault);
            let mac = format!("{}/etc/mac", vault);

            let pkg = format!("out/{}.app", app);
            let helper = format!("{}/Contents/Frameworks/'{} Helper.app'", pkg, app);

            let build = format!("target/{}", if release { "release" } else { "debug" });

            sh!("cargo build {}", if release { "--release" } else { "" });

            sh!("mkdir -p {}/Contents/{{Frameworks,MacOS,Resources}}", pkg);

            sh!("cp {}/Info.plist {}/Contents/.", mac, pkg);
            sh!(
                "cp -a {}/{{Info.plist,*.icns,English.lproj}} {}/Contents/Resources/.",
                mac,
                pkg
            );
            sh!("cp ../etc/*.html {}/Contents/Resources/.", pkg);

            sh!("cp -a {} {}/Contents/Frameworks/.", cef, pkg);

            let libcef = "'Chromium Embedded Framework.framework/Chromium Embedded Framework'";
            sh!(
                "install_name_tool -id @rpath/Frameworks/{} {}/Contents/Frameworks/{}",
                libcef,
                pkg,
                libcef
            );

            sh!("mkdir -p {}/Contents/MacOS", helper);
            sh!(
                "cp {}/helper-Info.plist {}/Contents/Info.plist",
                mac,
                helper
            );

            sh!("cp {}/{} {}/Contents/MacOS/{}", build, app, pkg, app);
            sh!(
                "install_name_tool -add_rpath '@executable_path/..' {}/Contents/MacOS/cedar-test",
                pkg
            );

            sh!(
                "cp {}/helper {}/Contents/MacOS/'{} Helper'",
                build,
                helper,
                app
            );
            sh!(
                "install_name_tool -add_rpath '@executable_path/../../../..' {}/Contents/MacOS/'cedar-test Helper'",
                helper
            );

            sh!("./{}/Contents/MacOS/{}", pkg, app);
            // sh!("open {}", pkg);
        }
    }

    Ok(())
}
