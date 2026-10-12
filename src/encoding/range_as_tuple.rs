use crate::encoding::{
    BorrowDecoder, Capped, DecodeContext, Decoder,
    Encoder, ValueBorrowDecoder, ValueDecoder,
};
use crate::DecodeError;
use bytes::Buf;
use core::ops::Range;


// both are a similar time regression
// the where bounds are necessary

impl<T, Estart, Eend> ValueDecoder<(Estart, Eend), Range<T>> for ()
where
    (): Decoder<Estart, T>
{
    fn decode_value<B: Buf + ?Sized>(
        _value: &mut Range<T>,
        _buf: Capped<B>,
    ) -> Result<(), DecodeError> {
        Ok(())
    }
}


impl<'a, T, Estart, Eend> ValueBorrowDecoder<'a, (Estart, Eend), Range<T>> for ()
where
    (): BorrowDecoder<'a, Estart, T>
{
    fn borrow_decode_value(
        _value: &mut Range<T>,
        _buf: Capped<&'a [u8]>,
    ) -> Result<(), DecodeError> {
        Ok(())
    }
}

