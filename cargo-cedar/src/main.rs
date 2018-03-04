#[macro_use]
extern crate structopt;

use std::env;
use std::path::PathBuf;
use std::process::Command;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum CLI {
    #[structopt(name = "build")]
    Build {
        #[structopt(long = "example")] example: String,
        #[structopt(long = "style", parse(from_os_str))] style: PathBuf,
    },
}

fn main() {
    let cli = CLI::from_iter(env::args().skip(1));
    println!("CLI: {:?}", cli);

    match cli {
        CLI::Build { example, style } => {
            let status = Command::new("cargo")
                .args(&[
                    "build",
                    "--target=wasm32-unknown-unknown",
                    "--release",
                    "--example",
                    &example,
                ])
                .status()
                .expect("failed to execute process");

            //            println!("process exited with: {}", status);

            assert!(status.success());
        }
    }
}
