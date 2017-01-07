use an_rope::Rope;
use std::path;

pub struct Buffer {
    /// The full text of the buffer
    pub text: Rope
  , /// Path to write the text to on saves, if this buffer is into an open file
    pub file: Option<path::PathBuf>
    // TODO: do we want to store a cursor position on the buffer?
    // TODO: put edit history here unless using persistent ropes?
}
