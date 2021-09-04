use super::piece::Piece;
use std::fmt::{Display, Formatter, Result};
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Debug)]
pub struct Action {
    pub from: u8,
    pub to: u8,
    pub piece: Piece,
}

impl Action {
    pub fn to_xml(&self) -> String {
        let from_y = self.from / 8;
        let from_x = self.from - from_y * 8;
        let to_y = self.to / 8;
        let to_x = self.to - to_y * 8;
        let xml_move = format!(
            "<from x=\"{}\" y=\"{}\"/>\n    <to x=\"{}\" y=\"{}\"/>\n",
            from_x, from_y, to_x, to_y
        );
        format!("  <data class=\"move\">\n    {}  </data>", xml_move)
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Drag {} from {} to {}",
            self.piece.to_char(0),
            self.from,
            self.to
        )
    }
}

pub const MAX_ACTIONS: usize = 300;

#[derive(Clone)]
pub struct ActionList {
    actions: [Action; MAX_ACTIONS],
    pub size: usize,
}

impl ActionList {
    #[inline(always)]
    pub fn push(&mut self, action: Action) {
        self.actions[self.size] = action;
        self.size += 1;
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.size = 0;
    }
}

impl Default for ActionList {
    fn default() -> Self {
        #[allow(clippy::uninit_assumed_init)]
        let actions = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
        Self { actions, size: 0 }
    }
}

impl Display for ActionList {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut ret = String::new();
        for i in 0..self.size {
            if i != 0 {
                ret.push_str(", ");
            }
            ret.push_str(&self[i].to_string());
        }
        write!(f, "{}", ret)
    }
}

impl Index<usize> for ActionList {
    type Output = Action;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        if index < self.size {
            &self.actions[index]
        } else {
            panic!(
                "Index out of bounds for ActionList, given index: {}, size: {}, actions: {:?}",
                index,
                self.size,
                self.actions[0..self.size].to_vec()
            );
        }
    }
}
