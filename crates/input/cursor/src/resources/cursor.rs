use bevy::prelude::*;

#[derive(Resource, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cursor {
    prev: Option<UVec2>,
    curr: Option<UVec2>,
}

impl Cursor {
    pub(crate) fn reset(&mut self) {
        self.prev = None;
        self.curr = None;
    }

    pub(crate) fn set_curr(&mut self, coord: Option<UVec2>) {
        self.prev = self.curr;
        self.curr = coord;
    }

    pub fn prev(&self) -> Option<UVec2> {
        self.prev
    }

    pub fn curr(&self) -> Option<UVec2> {
        self.curr
    }
}
