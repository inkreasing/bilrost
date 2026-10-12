use crate::{DecodeError};
use bytes::Buf;

pub(crate) mod decoding_modes;
mod encoding_traits;
mod range_as_tuple;
mod tuple;
mod unpacked;

pub(crate) use encoding_traits::{
    BorrowDecoder, Decoder, DistinguishedBorrowDecoder, DistinguishedDecoder, Encoder,
};
pub(crate) use encoding_traits::{
    ValueBorrowDecoder, ValueDecoder,
    // ValueEncoder,
};

pub(crate) struct Packed<E = GeneralPacked>(E);
pub(crate) struct Varint;

const PREFER_PACKED: u8 = 1;

pub struct GeneralGeneric<const P: u8>;
pub type GeneralPacked = GeneralGeneric<PREFER_PACKED>;
pub trait EmptyState<E, T: ?Sized>{}

pub trait ForOverwrite<E, T: ?Sized> {
    fn for_overwrite() -> T
    where
        T: Sized;
}

pub trait Collection {
    type Item;
}
/// Decodes a LEB128-bijective-encoded variable length integer from the buffer.
#[inline(always)]
pub fn decode_varint<B: Buf + ?Sized>(_buf: &mut B) -> Result<u64, DecodeError> {
    todo!()
}

pub type DecodeContext = ();

pub type RestrictedDecodeContext = ();

pub struct Capped<'a, B: 'a + Buf + ?Sized> {
    buf: &'a mut B,
}

pub type Canonicity = ();
