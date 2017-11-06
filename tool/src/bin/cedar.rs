
#[macro_use]
extern crate error_chain;
extern crate clap;

extern crate handlebars;

#[macro_use]
extern crate serde_derive;
extern crate serde;

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
        File(handlebars::TemplateFileError);
        Render(handlebars::RenderError);
    }

    errors {
        Missing(e: String) {}
    }
}

enum Command {
    Setup { force: bool, local: Option<String> },
    Run {
        release: bool,
        app: String,
        style: Option<String>,
    },
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
        .subcommand(
            SubCommand::with_name("run")
                .about("TODO")
                .arg(Arg::with_name("release").long("release"))
                .arg(
                    Arg::with_name("app")
                        .long("app")
                        .help("TODO")
                        .value_name("APP")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("style")
                        .long("style")
                        .help("TODO")
                        .value_name("CSS")
                        .takes_value(true),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("setup") {
        return Some(Command::Setup {
            force: matches.is_present("force"),
            local: matches.value_of("local").map(String::from),
        });
    }

    if let Some(matches) = matches.subcommand_matches("run") {
        return Some(Command::Run {
            release: matches.is_present("release"),
            app: matches.value_of("app").map(String::from).unwrap(),
            style: matches.value_of("style").map(String::from),
        });
    }

    None
}

macro_rules! sh {
    ( $( $cmd:tt )* ) => {{
        $crate::execute_with("sh", &format!($( $cmd )*))
    }};
}

fn execute_with(shell: &str, cmd: &String) {
    use std::process::{Command, Stdio};

    let mut command = {
        let mut command = Command::new(shell);
        command.arg("-c").arg(cmd);
        command.stdout(Stdio::inherit()).stderr(Stdio::inherit());
        command
    };

    let mut command = command.spawn().unwrap();
    let status = command.wait().unwrap();
    if !status.success() {
        panic!("Command failed! `{}`", cmd);
    }
}

fn render<M: serde::Serialize>(src: &str, dest: &str, model: &M) -> Result<()> {
    use std::fs::File;
    use std::io::prelude::*;
    use handlebars::Handlebars;

    let mut registry = Handlebars::new();

    registry.register_template_file(src, src)?;

    let mut file = File::create(dest)?;
    file.write_all(&registry.render(src, &model)?.into_bytes())?;

    Ok(())
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

        Command::Run {
            release,
            app,
            style,
        } => {

            #[derive(Serialize)]
            struct Model {
                executable: String,
                bundle: String,
            }

            let model = Model {
                executable: app.clone(),
                bundle: app.clone(),
            };

            if !Path::new("src/bin/helper.rs").exists() {
                bail!(ErrorKind::Missing("src/bin/helper.rs".into()));
            }

            let cef = format!("{}/lib/'Chromium Embedded Framework.framework'", vault);
            let mac = format!("{}/app/mac", vault);

            let pkg = format!("target/cedar/{}.app", app);
            let helper = format!("{}/Contents/Frameworks/{} Helper.app", pkg, app);

            let build = format!("target/{}", if release { "release" } else { "debug" });

            sh!("cargo build {}", if release { "--release" } else { "" });

            sh!("mkdir -p {}/Contents/{{Frameworks,MacOS,Resources}}", pkg);

            sh!(
                "cp -a {}/{{*.icns,English.lproj}} {}/Contents/Resources/.",
                mac,
                pkg
            );
            sh!("cp -a {}/{{app,etc}} {}/Contents/Resources/.", vault, pkg);

            render(
                &format!("{}/Info.plist", mac),
                &format!("{}/Contents/Info.plist", pkg),
                &model,
            )?;

            render(
                &format!("{}/Info.plist", mac),
                &format!("{}/Contents/Resources/Info.plist", pkg),
                &model,
            )?;

            if let Some(style) = style {
                sh!("cp {} {}/Contents/Resources/etc/style.css", style, pkg);
            }

            sh!("cp -a {} {}/Contents/Frameworks/.", cef, pkg);

            let libcef = "'Chromium Embedded Framework.framework/Chromium Embedded Framework'";
            sh!(
                "install_name_tool -id @rpath/Frameworks/{} {}/Contents/Frameworks/{}",
                libcef,
                pkg,
                libcef
            );

            sh!("mkdir -p '{}'/Contents/MacOS", helper);
            sh!("cp {}/{} {}/Contents/MacOS/.", build, app, pkg);
            sh!(
                "install_name_tool -add_rpath '@executable_path/..' {}/Contents/MacOS/{}",
                pkg,
                app
            );

            render(
                &format!("{}/helper-Info.plist", mac),
                &format!("{}/Contents/Info.plist", helper),
                &model,
            )?;

            sh!(
                "cp {}/helper {:?}/Contents/MacOS/'{} Helper'",
                build,
                helper,
                app
            );
            sh!(
                "install_name_tool -add_rpath '@executable_path/../../../..' '{}/Contents/MacOS/{} Helper'",
                helper,
                app
            );

            // sh!("./{}/Contents/MacOS/{}", pkg, app);
            sh!("open {}", pkg);
        }
    }

    Ok(())
}
