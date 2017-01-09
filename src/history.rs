//! Edit history

use an_zipper::ZipList;
use an_rope::Rope;

use std::convert;
use std::ops;

#[derive(Debug)]
pub struct Stack(ZipList<Rope>);

impl Stack {
    /// Construct a new history stack
    pub fn new() -> Self {
        Stack(ZipList::new())
    }

    /// Undo the most recent action in the history stack
    #[inline] pub fn undo(&mut self) {
        self.0.move_left();
    }

    /// Redo the most recently undone action in the history stack
    #[inline] pub fn redo(&mut self) {
        // TODO: check if the right side is non-empty?
        self.0.move_right();
    }

    /// Apply a function to the current state of the stack, advancing it
    pub fn edit<F>(&mut self, f: F)
    where F: Fn(&Rope) -> Rope {
        let state = {
            let curr = self.0.peek_left();
            f(curr.unwrap_or( &Rope::new() ))
        };
        self.0.push_left(state);
    }

    /// Return the current state of the history stack
    #[inline] pub fn state(&self) -> Option<&Rope> {
        self.0.peek_left()
    }
}

impl Default for Stack {
    #[inline] fn default() -> Self { Self::new() }
}

impl convert::From<Rope> for Stack {
    fn from(r: Rope) -> Self {
        let mut list = ZipList::new();
        list.push_left(r);
        Stack(list)
    }
}
