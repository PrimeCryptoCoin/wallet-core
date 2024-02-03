// Copyright © 2017-2023 Trust Wallet.
//
// This file is part of Trust. The full Trust copyright notice, including
// terms governing use, modification, and redistribution, is contained in the
// file LICENSE at the root of the source code distribution tree.

use std::fmt;

/// Represents a sign of an integer.
pub enum Sign {
    Positive,
    Negative,
}

impl Sign {
    /// Returns whether the sign is positive.
    #[inline]
    pub fn is_positive(&self) -> bool {
        matches!(self, Sign::Positive)
    }

    /// Returns whether the sign is negative.
    #[inline]
    pub fn is_negative(&self) -> bool {
        !self.is_positive()
    }

    /// Returns the sign character.
    #[inline]
    pub fn as_char(&self) -> char {
        match self {
            Sign::Positive => '+',
            Sign::Negative => '-',
        }
    }
}

impl fmt::Display for Sign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self, f.sign_plus()) {
            (Self::Positive, false) => Ok(()),
            _ => write!(f, "{}", self.as_char()),
        }
    }
}
