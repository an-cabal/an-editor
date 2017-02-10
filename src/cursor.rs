
use std::convert;
use termion::cursor::Goto;

#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub line: u16
  , pub col: u16
}

impl convert::Into<Goto> for Position {
    #[inline] fn into(self) -> Goto {
        Goto(self.line, self.col)
    }
}
