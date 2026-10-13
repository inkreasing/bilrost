use crate::encoding::{
    EmptyState, ForOverwrite,
};
use crate::encoding::{
    decoding_modes,
    BorrowDecoder, Capped, DecodeContext, Decoder,
    DistinguishedBorrowDecoder,
    Encoder,
    DistinguishedDecoder,
    GeneralPacked, RestrictedDecodeContext,
    ValueBorrowDecoder, ValueDecoder,
};
use crate::{Canonicity, DecodeError};

pub struct Unpacked<E = GeneralPacked>(E);

macro_rules! define_decoders {
    (
        mode: $mode:ident,
        relaxed: $relaxed:ident::$relaxed_method:ident,
        relaxed_value: $relaxed_value:ident::$relaxed_value_method:ident,
        relaxed_field: $relaxed_field:ident::$relaxed_field_method:ident,
        distinguished: $distinguished:ident::$distinguished_method:ident,
    ) => {

        /// Decodes an array value from either unpacked or packed representation. If there are not
        /// exactly the expected number of fields the value is considered to be invalid.
        #[inline]
        pub(super) fn decode_array_either_repr<T, const N: usize, E>(
            _arr: &mut [T; N],
        ) -> Result<(), DecodeError>
        where
            (): $relaxed_value <E, T>,
        {
            Ok(())
        }

        /// Decodes an array value from either packed or unpacked in distinguished mode. If there
        /// are not exactly the expected number of fields the value is considered to be invalid.
        #[inline]
        pub(super) fn decode_distinguished_array_either_repr<T, const N: usize, E>(
            _arr: &mut [T; N],
        ) -> Result<Canonicity, DecodeError>
        where
            (): $relaxed_value <E, T>
        {
            todo!()
        }
    };
}

pub(crate) mod owned {
    use super::*;
    decoding_modes::__invoke!(define_decoders, owned);
}

pub(crate) mod borrowed {
    use super::*;
    decoding_modes::__invoke!(define_decoders, borrowed);
}

impl<T, const N: usize, E> Encoder<Unpacked<E>, [T; N]> for () {}

impl<T, const N: usize, E> Encoder<Unpacked<E>, Option<[T; N]>> for () {}

macro_rules! impl_decoders {
    (
        mode: $mode:ident,
        relaxed: $relaxed:ident::$relaxed_method:ident,
        relaxed_value: $relaxed_value:ident::$relaxed_value_method:ident,
        relaxed_field: $relaxed_field:ident::$relaxed_field_method:ident,
        distinguished: $distinguished:ident::$distinguished_method:ident,
    ) => {

        impl<T, const N: usize, E>
        $relaxed <Unpacked<E>, [T; N]> for ()
        where
            (): $relaxed_value <E, T> + EmptyState<E, [T; N]>,
        {
            #[inline]
            fn $relaxed_method(
                value: &mut [T; N],
            ) -> Result<(), DecodeError> {
                // time regression
                $mode::decode_array_either_repr(value)
            }
        }


        impl<T, const N: usize, E>
        $relaxed <Unpacked<E>, Option<[T; N]>> for ()
        where
            (): $relaxed_value <E, T> + ForOverwrite<E, [T; N]>,
        {
            #[inline]
            fn $relaxed_method(
                _value: &mut Option<[T; N]>,
            ) -> Result<(), DecodeError> {
                Ok(())
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
                value: &mut Option<[T; N]>,
            ) -> Result<Canonicity, DecodeError> {
                // time regression
                $mode::decode_distinguished_array_either_repr(
                    value.get_or_insert_with(<() as ForOverwrite::<E, _>>::for_overwrite),
                )
            }
        }
    };
}

decoding_modes::__invoke!(impl_decoders, owned);
decoding_modes::__invoke!(impl_decoders, borrowed);
