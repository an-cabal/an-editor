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

fn main() {
    let args = clap_app!(an =>
        (version: crate_version!())
        (author: crate_authors!(", "))
        (about: "An text editor.")
            (@arg INPUT: +required "Sets the input file to open.")
            (@arg debug: -d ... "Sets the level of debugging information")
    ).get_matches();
}
