use super::bitboard::FINISH_LINES;
use super::piece;
use std::fmt::{Display, Formatter, Result};
use std::ops::{Index, IndexMut};

// 00000000 00111111 from
// 00001111 11000000 to
// 00110000 00000000 piece
// 01000000 00000000 is_capture
// 10000000 00000000 is_amber_capture

const FROM_MASK: u16 = 0b111111;
const TO_MASK: u16 = 0b111111 << 6;
const PIECE_MASK: u16 = 0b11 << 12;
const CAPTURE_MASK: u16 = 1 << 14;
const AMBER_CAPTURE_MASK: u16 = 1 << 15;
const ACTUAL_MOVE_MASK: u16 = FROM_MASK | TO_MASK | PIECE_MASK;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Action(u16);

impl Action {
    pub const NONE: Self = Self(0);

    pub fn new(from: u16, to: u16, piece: u8, is_capture: bool, is_amber_capture: bool) -> Self {
        Self(
            from | to << 6
                | (piece as u16) << 12
                | (is_capture as u16) << 14
                | (is_amber_capture as u16) << 15,
        )
    }

    #[inline(always)]
    pub fn from(self) -> u16 {
        self.0 & FROM_MASK
    }

    #[inline(always)]
    pub fn to(self) -> u16 {
        (self.0 & TO_MASK) >> 6
    }

    #[inline(always)]
    pub fn piece(self) -> u16 {
        (self.0 & PIECE_MASK) >> 12
    }

    #[inline(always)]
    pub fn is_capture(self) -> bool {
        (self.0 & CAPTURE_MASK) > 0
    }

    #[inline(always)]
    pub fn is_amber_capture(self) -> bool {
        (self.0 & AMBER_CAPTURE_MASK) > 0
    }

    #[inline(always)]
    pub fn is_promotion(self, current_color: usize) -> bool {
        self.piece() as u8 != piece::SEAL && (1 << self.to()) & FINISH_LINES[current_color] > 0
    }

    pub fn serialize(self) -> String {
        (self.0 & ACTUAL_MOVE_MASK).to_string()
    }

    pub fn deserialize(string: String) -> Self {
        Action(string.parse::<u16>().unwrap())
    }

    pub fn to_xml(&self) -> String {
        let from_y = self.from() / 8;
        let from_x = self.from() - from_y * 8;
        let to_y = self.to() / 8;
        let to_x = self.to() - to_y * 8;
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
            "{} {} to {}",
            piece::to_string(piece::PIECES[self.piece() as usize]),
            self.from(),
            self.to(),
        )
    }
}

// 00000001 capture
// 00000110 captured piece
// 00001000 captured piece was stacked
// 00010000 moved piece was stacked

pub const CAPTURED_PIECE_WAS_STACKED: u8 = 0b1;
pub const MOVED_PIECE_WAS_STACKED: u8 = 0b10;

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct UndoInfo(u8, u64);

impl UndoInfo {
    #[inline(always)]
    pub fn set_hash(&mut self, hash: u64) {
        self.1 = hash;
    }

    #[inline(always)]
    pub fn get_hash(&self) -> u64 {
        self.1
    }

    #[inline(always)]
    pub fn set_capture(&mut self, piece: u8, capture_info: u8) {
        self.0 |= 0b1 | (piece as u8) << 1 | capture_info << 3;
    }

    #[inline(always)]
    pub fn get_capture(self) -> Option<(u8, u8)> {
        if self.0 & 0b1 > 0 {
            Some(((self.0 >> 1) & 0b11, (self.0 >> 3) & 0b11))
        } else {
            None
        }
    }

    pub fn set_finish_line_info(&mut self, info: u8) {
        self.0 |= info << 3;
    }

    pub fn get_finish_line_info(self) -> u8 {
        self.0 >> 3 & 0b11
    }
}

pub const MAX_ACTIONS: usize = 200;

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

    #[inline(always)]
    pub fn swap(&mut self, a: usize, b: usize) {
        self.actions.swap(a, b);
    }

    pub fn find_action(&self, action: Action) -> Option<usize> {
        for i in 0..self.size {
            if self.actions[i] == action {
                return Some(i);
            }
        }
        None
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

pub struct ActionListStack {
    pub action_lists: Vec<ActionList>,
}

impl ActionListStack {
    pub fn with_size(size: usize) -> Self {
        Self {
            action_lists: vec![ActionList::default(); size],
        }
    }
}

impl Index<usize> for ActionListStack {
    type Output = ActionList;

    fn index(&self, index: usize) -> &Self::Output {
        if index < self.action_lists.len() {
            &self.action_lists[index]
        } else {
            panic!("Can not extend ActionListStack in non mutable index");
        }
    }
}

impl IndexMut<usize> for ActionListStack {
    fn index_mut(&mut self, index: usize) -> &mut ActionList {
        if index < self.action_lists.len() {
            &mut self.action_lists[index]
        } else {
            self.action_lists
                .append(vec![ActionList::default(); index + 1 - self.action_lists.len()].as_mut());
            self.index_mut(index)
        }
    }
}
