#![no_std]
#![forbid(unsafe_op_in_unsafe_fn)]
#![allow(unused)]

/// Re-export of the bytes crate for use within derived code.
use bytes;

pub(crate) mod encoding;

use crate::encoding::{Canonicity};
pub type DecodeError = ();

