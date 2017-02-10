//! An editor

#![cfg_attr( feature = "unstable", feature(try_from))]
#![cfg_attr( feature = "clippy", feature(plugin) )]
#![cfg_attr( feature = "clippy", plugin(clippy) )]

extern crate an_rope;
extern crate an_zipper;
#[macro_use]
extern crate clap;
extern crate termion;

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
    use termion::color;
    use termion::raw::IntoRawMode;
    use std::io::{Read, Write, stdout, stdin};

    let args = clap_app!(an =>
        (version: crate_version!())
        (author: crate_authors!(", "))
        (about: "An text editor.")
            // FIXME: validate that the path is valid (& if it exists, we have
            //        permissions?)
            (@arg debug: -d ... "Sets the level of debugging information")
            (@arg config: -c --config +takes_value {file_exists}
                "Sets a custom config file")
            (@arg INPUT: "Sets the input file to open.")
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

    // Initialize 'em all.
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let stdin = stdin();
    let stdin = stdin.lock();

    write!( stdout, "{}{}{}"
          , termion::clear::All
          , termion::style::Reset
          , termion::cursor::Goto(1,1)
          ).unwrap();
    stdout.flush().unwrap();
    loop {
    for (n, line) in buffer.text.state().unwrap().lines().enumerate() {
        write!( stdout, "{}{}{}{:<5}{}{}{}"
              , termion::cursor::Goto(1, n as u16)
              // FIXME: get colors from config file
              , color::Fg(color::LightWhite)
              , color::Bg(color::LightBlack)
              , n
              , color::Fg(color::White)
              , color::Bg(color::Black)
              , line ).unwrap();
    }

    stdout.flush().unwrap();
}

}
