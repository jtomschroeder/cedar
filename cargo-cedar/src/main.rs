#[macro_use]
extern crate structopt;
extern crate sass_rs as sass;

use std::env;
use std::path::PathBuf;
use std::process::Command;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum CLI {
    #[structopt(name = "run")]
    Run {
        #[structopt(long = "example")]
        example: Option<String>,

        #[structopt(long = "style", parse(from_os_str))]
        style: Option<PathBuf>,
    },
}

fn main() {
    let cli = CLI::from_iter(env::args().skip(1));
    println!("CLI: {:?}", cli);

    match cli {
        CLI::Run { example, style } => {
            let mut command = Command::new("cargo");
            command.args(&["run", "--release"]);

            if let Some(style) = style {
                let css = sass::compile_file(style, sass::Options::default()).unwrap();
                command.env("CEDAR_STYLING", css);
                println!("HERE");
            }

            let status = command.status().expect("Failed to execute `cargo run`");

            assert!(status.success());
        }
    }
}
