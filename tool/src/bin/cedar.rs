
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
        Command::Setup { force } => {
            if !force && Path::new(&vault).exists() {
                println!("Already setup!");
                return Ok(());
            }

            fs::create_dir_all(&format!("{}/.cedar/lib", home))?;

            // TODO: copy CEF.framework into .cedar/lib
            // TODO: install_name_tool -id "{{CEF}}/Chromium Embedded Framework" "{{CEF}}/Chromium Embedded Framework"
        }

        Command::Run { release } => {
            let app = "cedar-tool";

            let cef = format!("{}/lib/'Chromium Embedded Framework.framework'", vault);
            let pkg = format!("out/{}.app", app);
            let helper = format!("{}/Contents/Frameworks/'cefsimple Helper.app'", pkg);


            sh!("cargo build {}", if release { "--release" } else { "" });


            sh!("mkdir -p {}/Contents/{{Frameworks,MacOS,Resources}}", pkg);

            sh!("cp ../lib/app/mac/Info.plist {}/Contents/.", pkg);
            sh!(
                "cp -a ../lib/app/mac/{{Info.plist,*.icns,English.lproj}} {}/Contents/Resources/.",
                pkg
            );
            sh!("cp ../etc/*.html {}/Contents/Resources/.", pkg);

            sh!("cp -a {} {}/Contents/Frameworks/.", cef, pkg);
            // install_name_tool -id "@rpath/Frameworks/Chromium Embedded Framework.framework/Chromium Embedded Framework" \
            // 					  "{{APP}}/Contents/Frameworks/Chromium Embedded Framework.framework/Chromium Embedded Framework"

            sh!("mkdir -p {}/Contents/MacOS", helper);
            sh!(
                "cp ../lib/app/mac/helper-Info.plist {}/Contents/Info.plist",
                helper
            );

            sh!("cp target/release/examples/{{EXAMPLE}} {{APP}}/Contents/MacOS/cefsimple");
            // sh!("install_name_tool -add_rpath "@executable_path/.." {{APP}}/Contents/MacOS/cefsimple");

            // sh!("cargo build --release --bin helper");

            // sh!("cp target/release/helper '{{HELPER}}/Contents/MacOS/cefsimple Helper'");
            // sh!("install_name_tool -add_rpath "@executable_path/../../../.." "{{HELPER}}/Contents/MacOS/cefsimple Helper"");

            // sh!("./{{APP}}/Contents/MacOS/cefsimple");
        }
    }

    Ok(())
}
