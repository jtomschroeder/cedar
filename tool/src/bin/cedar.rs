
#[macro_use]
extern crate error_chain;

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

fn run() -> Result<()> {
    let home = env::var("HOME").unwrap();

    // TODO: disable check with --force
    if Path::new(&format!("{}/.cedar", home)).exists() {
        println!("Already setup!");
        return Ok(());
    }

    fs::create_dir_all(&format!("{}/.cedar/lib", home))?;

    // TODO: copy CEF.framework into .cedar/lib
    // TODO: install_name_tool -id "{{CEF}}/Chromium Embedded Framework" "{{CEF}}/Chromium Embedded Framework"

    Ok(())
}
