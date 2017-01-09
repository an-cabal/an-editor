//! Edit history

use an_zipper::ZipList;
use an_rope::Rope;

use std::convert;

#[derive(Debug)]
pub struct Stack(ZipList<Rope>);

impl Stack {
    /// Construct a new history stack
    pub fn new() -> Self {
        Stack(ZipList::new())
    }

    pub fn undo(&mut self) {
        self.0.move_left();
    }

    pub fn redo(&mut self) {
        self.0.move_right();
    }

    pub fn edit<F>(&mut self, f: F)
    where F: Fn(&Rope) -> Rope {
        let state = {
            let curr = self.0.peek_left();
            f(curr.unwrap_or( &Rope::new() ))
        };
        self.0.push_left(state);
    }
}

impl convert::From<Rope> for Stack {
    fn from(r: Rope) -> Self {
        let mut list = ZipList::new();
        list.push_left(r);
        Stack(list)
    }
}
