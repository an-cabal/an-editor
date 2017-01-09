use an_rope::Rope;

use history;

use std::convert;
use std::default::Default;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::io;
use std::io::{Read, BufReader};
use std::ops;


#[derive(Debug)]
pub struct Buffer {
    /// The full text of the buffer
    pub text: history::Stack
  , /// Path to write the text to on saves, if this buffer is into an open file
    pub file: Option<PathBuf> // TODO: support having a session file?
    // TODO: do we want to store a cursor position on the buffer?
}

impl Buffer {

    pub fn new() -> Self {
        Buffer { text: history::Stack::new()
               , file: None }
    }

    pub fn from_file<P>(path: P) -> io::Result<Buffer>
    where P: AsRef<Path> {
        let file = File::open(path.as_ref())?;
        let mut text = String::new();

        BufReader::new(file).read_to_string(&mut text)?;

        Ok(Buffer { text: history::Stack::from(Rope::from(text))
                  , file: Some(path.as_ref().to_owned()) })

    }



}


impl Default for Buffer {
    #[inline] fn default() -> Self { Self::new() }
}

impl<T> convert::From<T> for Buffer
where Rope: convert::From<T> {

    fn from(text: T) -> Self {
        Buffer { text: history::Stack::from(Rope::from(text))
               , file: None }
    }

}

#[cfg(feature = "unstable")]
impl<P> convert::TryFrom<P> for Buffer
where P: AsRef<Path> {
    type Err = io::Error;

    #[inline]
    fn try_from(path: P) -> io::Result<Self> { Self::from_file(path) }
}

impl ops::Deref for Buffer {
    type Target = history::Stack;
    #[inline] fn deref(&self) -> &history::Stack { &self.text }
}
