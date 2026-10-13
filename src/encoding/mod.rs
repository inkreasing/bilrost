mod tuple;
mod unpacked;

pub trait EmptyState<E, T>{}

pub trait ForOverwrite<E, T> {
    fn for_overwrite() -> T
    where
        T: Sized;
}
pub(crate) trait Decoder<E, T> {
    fn decode();
}

pub(crate) trait DistinguishedDecoder<E, T> {
    fn decode_distinguished();
}

pub(crate) trait BorrowDecoder<E, T> {
    fn borrow_decode();
}

pub(crate) trait DistinguishedBorrowDecoder<E, T> {
    fn borrow_decode_distinguished();
}

pub(crate) trait ValueDecoder<E, T> {
    fn decode_value();
}

pub(crate) trait ValueBorrowDecoder<E, T> {
    fn borrow_decode_value();
}

// both are a similar time regression
// the where bounds are necessary
use core::ops::Range;

impl<T, Estart, Eend> ValueDecoder<(Estart, Eend), Range<T>> for ()
where
    (): Decoder<Estart, T>
{
    fn decode_value(
    ) {
    }
}


impl<'a, T, Estart, Eend> ValueBorrowDecoder<(Estart, Eend), Range<T>> for ()
where
    (): BorrowDecoder<Estart, T>
{
    fn borrow_decode_value(
    ) {
    }
}
