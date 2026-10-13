#![no_std]
#![forbid(unsafe_op_in_unsafe_fn)]
#![allow(unused)]

pub(crate) mod encoding;

use crate::encoding::{Canonicity};
pub type DecodeError = ();

