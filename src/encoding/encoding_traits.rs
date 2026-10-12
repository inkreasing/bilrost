use crate::encoding::{
    Capped, DecodeContext, RestrictedDecodeContext,
};
use crate::{Canonicity, DecodeError};
use bytes::Buf;

/// The core trait for encoding bilrost data.
pub(crate) trait Encoder<E, T: ?Sized> {
}

/// The core trait for decoding bilrost data. Data must always be copied from the buffer.
pub(crate) trait Decoder<E, T> {
    fn decode<B: Buf + ?Sized>(
        value: &mut T,
        buf: Capped<B>,
    ) -> Result<(), DecodeError>;
}

pub(crate) trait DistinguishedDecoder<E, T> {
    /// Decodes a field for the value, returning a value indicating how canonical the encoding was.
    fn decode_distinguished<B: Buf + ?Sized>(
        value: &mut T,
        buf: Capped<B>,
    ) -> Result<Canonicity, DecodeError>;
}

/// Decoding trait that allows decoding borrowed data.
pub(crate) trait BorrowDecoder<'a, E, T> {
    fn borrow_decode(
        value: &mut T,
        buf: Capped<&'a [u8]>,
    ) -> Result<(), DecodeError>;
}

/// Decoding trait that allows distinguished decoding of borrowed data.
pub(crate) trait DistinguishedBorrowDecoder<'a, E, T> {
    /// Decodes a field for the value, returning a value indicating how canonical the encoding was.
    fn borrow_decode_distinguished(
        value: &mut T,
        buf: Capped<&'a [u8]>,
    ) -> Result<Canonicity, DecodeError>;
}

pub(crate) trait ValueDecoder<E, T> {
    fn decode_value<B: Buf + ?Sized>(
        value: &mut T,
        buf: Capped<B>,
    ) -> Result<(), DecodeError>;
}

pub(crate) trait ValueBorrowDecoder<'a, E, T> {
    fn borrow_decode_value(
        value: &mut T,
        buf: Capped<&'a [u8]>,
    ) -> Result<(), DecodeError>;
}
