use crate::encoding::{
    EmptyState, ForOverwrite,
};
use crate::encoding::{
    BorrowDecoder, Decoder,
    DistinguishedBorrowDecoder,
    DistinguishedDecoder,
    ValueBorrowDecoder, ValueDecoder,
};

pub struct Unpacked<E>(E);

pub(crate) mod owned {
    use super::*;
    pub(super) fn decode_array_either_repr<T, E>()
    where
        (): ValueDecoder<E, T>,
    {}
}

macro_rules! impl_decoders {
    (
        mode: $mode:ident,
        relaxed: $relaxed:ident::$relaxed_method:ident,
        relaxed_value: $relaxed_value:ident::$relaxed_value_method:ident,
        distinguished: $distinguished:ident::$distinguished_method:ident,
    ) => {

        impl<T, const N: usize, E>
        $relaxed <Unpacked<E>, [T; N]> for ()
        where
            (): $relaxed_value <E, T>
            + EmptyState<E, [T; N]>,
        {
            #[inline]
            fn $relaxed_method(
            ) {
                // time regression
                // specifying the generic parameters to T or E reduces the slowdown
                $mode::decode_array_either_repr::<_, _>()
            }
        }

        impl<T, const N: usize, E>
        $relaxed <Unpacked<E>, Option<[T; N]>> for ()
        where
            (): ForOverwrite<E, [T; N]>,
        {
            #[inline]
            fn $relaxed_method(
            ) {
            }
        }

        impl<T, const N: usize, E>
        $distinguished <Unpacked<E>, Option<[T; N]>> for ()
        where
            (): ForOverwrite<E, [T; N]>
                + $relaxed_value<E, T>,
        {
            #[inline]
            fn $distinguished_method(
            ) {
                // time regression
                $mode::decode_array_either_repr::<_, _>()
            }
        }
    };
}

impl_decoders!(
    mode: owned,
    relaxed: Decoder::decode,
    relaxed_value: ValueDecoder::decode_value,
    distinguished: DistinguishedDecoder::decode_distinguished,
);

// this is a big part of the slowdown, but it is probably interesting on its own
impl_decoders!(
    mode: borrowed,
    relaxed: BorrowDecoder::borrow_decode,
    relaxed_value: ValueBorrowDecoder::borrow_decode_value,
    distinguished: DistinguishedBorrowDecoder::borrow_decode_distinguished,
);
pub(crate) mod borrowed {
    use super::*;
    pub(super) fn decode_array_either_repr<T, E>()
    where
        (): ValueBorrowDecoder<E, T>,
    {}
}
