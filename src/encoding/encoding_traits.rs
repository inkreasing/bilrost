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
    fn decode(
        value: &mut T,
    ) -> Result<(), DecodeError>;
}

pub(crate) trait DistinguishedDecoder<E, T> {
    /// Decodes a field for the value, returning a value indicating how canonical the encoding was.
    fn decode_distinguished(
        value: &mut T,
    ) -> Result<Canonicity, DecodeError>;
}

/// Decoding trait that allows decoding borrowed data.
pub(crate) trait BorrowDecoder<E, T> {
    fn borrow_decode(
        value: &mut T,
    ) -> Result<(), DecodeError>;
}

/// Decoding trait that allows distinguished decoding of borrowed data.
pub(crate) trait DistinguishedBorrowDecoder<E, T> {
    /// Decodes a field for the value, returning a value indicating how canonical the encoding was.
    fn borrow_decode_distinguished(
        value: &mut T,
    ) -> Result<Canonicity, DecodeError>;
}

pub(crate) trait ValueDecoder<E, T> {
    fn decode_value(
        value: &mut T,
    ) -> Result<(), DecodeError>;
}

pub(crate) trait ValueBorrowDecoder<E, T> {
    fn borrow_decode_value(
        value: &mut T,
    ) -> Result<(), DecodeError>;
}
