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

/// Tests whether or not a file actually exists
fn file_exists(path: String) -> Result<(), String> {
    if std::fs::metadata(path.clone()).is_ok() {
        Ok(())
    } else {
        Err(format!("File `{}` doesn't exist", path))
    }
}

fn main() {
    use self::buffer::Buffer;

    let args = clap_app!(an =>
        (version: crate_version!())
        (author: crate_authors!(", "))
        (about: "An text editor.")
            // FIXME: validate that the path is valid (& if it exists, we have
            //        permissions?)
            (@arg INPUT: "Sets the input file to open.")
            (@arg debug: -d ... "Sets the level of debugging information")
            (@arg config: -c --config <conf> {file_exists}
                "Sets a custom config file")
    ).get_matches();

    if let Some(cfg) = args.value_of("config") {
        // FIXME: unimplemented
        unimplemented!();
    } else if let Ok(_) = file_exists(String::from("~/.anrc")) {
        // FIXME: do we want "~/.anrc" to be the name of the config file?
        // FIXME: unimplemented
        unimplemented!();
    }

    // Create a text buffer
    let buffer = if let Some(path) = args.value_of("INPUT") {
        // either by opening the specified input file if one was given,
        Buffer::from_file(path).expect("Could not open file!")
    } else {
        // or just making an empty file
        Buffer::new()
    };
}
