use serde_derive::{Deserialize, Serialize};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

#[derive(Debug, Copy, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub enum Flags {
    Carry = (1 << 0),
    Zero = (1 << 1),
    InterruptDisable = (1 << 2),
    DecimalMode = (1 << 3),
    Break = (1 << 4),
    Unused = (1 << 5),
    Overflow = (1 << 6),
    Negative = (1 << 7),
}

macro_rules! set_flag {
    ($field:expr, $flag:ident) => {
        set_flag!($field, $flag, true)
    };
    ($field:expr, $flag:ident, $cond:expr) => {
        match $cond {
            true => $field |= Flags::$flag,
            false => $field &= !Flags::$flag,
        }
    };
}
pub(super) use set_flag;
macro_rules! unset_flag {
    ($field:expr, $flag:ident) => {
        set_flag!($field, $flag, false)
    };
}
pub(super) use unset_flag;

impl Flags {
    pub fn is_set(self, val: u8) -> bool {
        self & val == self as u8
    }
}

impl BitAnd<u8> for Flags {
    type Output = u8;

    fn bitand(self, rhs: u8) -> Self::Output {
        self as u8 & rhs
    }
}

impl BitAnd<Flags> for u8 {
    type Output = u8;

    fn bitand(self, rhs: Flags) -> Self::Output {
        self & rhs as u8
    }
}

impl BitAndAssign<Flags> for u8 {
    fn bitand_assign(&mut self, rhs: Flags) {
        *self = *self & rhs
    }
}

impl BitOr<u8> for Flags {
    type Output = u8;

    fn bitor(self, rhs: u8) -> Self::Output {
        self as u8 | rhs
    }
}

impl BitOr<Flags> for u8 {
    type Output = u8;

    fn bitor(self, rhs: Flags) -> Self::Output {
        rhs as u8 | self
    }
}

impl BitOr<Flags> for Flags {
    type Output = u8;

    fn bitor(self, rhs: Flags) -> Self::Output {
        self as u8 | rhs as u8
    }
}

impl BitOrAssign<Flags> for u8 {
    fn bitor_assign(&mut self, rhs: Flags) {
        *self = *self | rhs;
    }
}

impl BitXor<Flags> for u8 {
    type Output = u8;

    fn bitxor(self, rhs: Flags) -> Self::Output {
        self ^ rhs as u8
    }
}

impl BitXor<Flags> for Flags {
    type Output = u8;

    fn bitxor(self, rhs: Flags) -> Self::Output {
        self as u8 ^ rhs as u8
    }
}

impl BitXor<u8> for Flags {
    type Output = u8;

    fn bitxor(self, rhs: u8) -> Self::Output {
        self as u8 ^ rhs
    }
}

impl BitXorAssign<Flags> for u8 {
    fn bitxor_assign(&mut self, rhs: Flags) {
        *self = *self ^ rhs;
    }
}

impl Not for Flags {
    type Output = u8;

    fn not(self) -> Self::Output {
        !(self as u8)
    }
}

impl PartialEq<u8> for Flags {
    fn eq(&self, other: &u8) -> bool {
        *self as u8 == *other
    }
}

impl PartialEq<Flags> for u8 {
    fn eq(&self, other: &Flags) -> bool {
        *self == *other as u8
    }
}
