//! An editor

#![cfg_attr( feature = "unstable", feature(try_from))]
#![cfg_attr( feature = "clippy", feature(plugin) )]
#![cfg_attr( feature = "clippy", plugin(clippy) )]

extern crate an_rope;
extern crate an_zipper;
#[macro_use]
extern crate clap;

pub mod buffer;
pub mod history;

fn file_exists(path: String) -> Result<(), String> {
    if std::fs::metadata(path.clone()).is_ok() {
        Ok(())
    } else {
        Err(format!("File `{}` doesn't exist", path))
    }
}

fn main() {

    let args = clap_app!(an =>
        (version: crate_version!())
        (author: crate_authors!(", "))
        (about: "An text editor.")
            (@arg INPUT: +required "Sets the input file to open.")
            (@arg debug: -d ... "Sets the level of debugging information")
            (@arg config: -c --config <conf> #{1, 2} {file_exists}
                "Sets a custom config file")
    ).get_matches();

    if let Some(cfg) = args.value_of("config") {
        // load custom config file, otherwise load ~/.anrc
        unimplemented!()
    }
}
