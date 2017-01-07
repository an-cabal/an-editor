use an_rope::Rope;

use std::convert;
use std::default::Default;
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::io;
use std::io::{Read, Write, BufReader, BufWriter};


#[derive(Debug)]
pub struct Buffer<W: Write> {
    /// The full text of the buffer
    pub text: Rope
  , /// Path to write the text to on saves, if this buffer is into an open file
    pub file: Option<W> // TODO: should this be the writer, rather than the path?
    // TODO: do we want to store a cursor position on the buffer?
    // TODO: put edit history here unless using persistent ropes?
}

impl<W: Write> Buffer<W> {

    pub fn new() -> Self {
        Buffer { text: Rope::new()
               , file: None }
    }

    pub fn from_file<P>(path: P) -> io::Result<Buffer<BufWriter<File>>>
    where P: AsRef<Path> {
        let file = OpenOptions::new()
                    .read(true)
                    .open(path.as_ref())?;
        let mut text = String::new();

        BufReader::new(file).read_to_string(&mut text)?;

        let file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(path.as_ref())?;

        Ok(Buffer { text: Rope::from(text)
                  , file: Some(BufWriter::new(file)) })

    }

}


impl<W: Write> Default for Buffer<W> {
    #[inline]
    fn default() -> Self { Self::new() }
}

impl<T, W> convert::From<T> for Buffer<W>
where Rope: convert::From<T>
    , W: Write {

    fn from(text: T) -> Self {
        Buffer { text: Rope::from(text)
               , file: None }
    }

}

#[cfg(feature = "unstable")]
impl<P> convert::TryFrom<P> for Buffer<BufWriter<File>>
where P: AsRef<Path> {
    type Err = io::Error;

    #[inline]
    fn try_from(path: P) -> Result<Self, Self::Err> { Self::from_file(path) }
}
