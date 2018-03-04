extern crate hammer;
#[macro_use]
extern crate structopt;

use std::env;
use std::path::PathBuf;
use std::process::Command;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum CLI {
    #[structopt(name = "run")]
    Run {
        #[structopt(long = "example")] example: String,
        #[structopt(long = "style", parse(from_os_str))] style: PathBuf,
    },
}

fn main() {
    let cli = CLI::from_iter(env::args().skip(1));
    println!("CLI: {:?}", cli);

    match cli {
        CLI::Run { example, style } => {
            let status = Command::new("cargo")
                .args(&[
                    "build",
                    "--target=wasm32-unknown-unknown",
                    "--release",
                    "--example",
                    &example,
                ])
                .status()
                .expect("Failed to execute `cargo build`");

            // println!("process exited with: {}", status);

            assert!(status.success());

            // target/wasm32-unknown-unknown/release/examples
            // target/cedar/release/examples/${example}/*

            let directory = format!("target/cedar/release/examples/{}", example);

            std::fs::create_dir_all(&directory).unwrap();

            std::fs::copy(
                &format!(
                    "target/wasm32-unknown-unknown/release/examples/{}.wasm",
                    example
                ),
                &format!("{}/code.wasm", directory),
            ).unwrap();

            std::fs::copy("lib/wasm/app.js", &format!("{}/app.js", directory)).unwrap();
            std::fs::copy("lib/wasm/index.html", &format!("{}/index.html", directory)).unwrap();
            std::fs::copy("lib/wasm/style.css", &format!("{}/style.css", directory)).unwrap();

            println!("Serving {} @ localhost:8000", example);
            hammer::serve(directory, "localhost:8000");
        }
    }
}
