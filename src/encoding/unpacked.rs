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
        distinguished_value: $distinguished_value:ident::$distinguished_value_method:ident,
        distinguished_field: $distinguished_field:ident::$distinguished_field_method:ident,
        buf_ty: $buf_ty:ty,
        impl_buf_ty: $impl_buf_ty:ty,
        $(buf_generic: ($($buf_generic:tt)*),)?
        $(lifetime: $lifetime:lifetime,)?
    ) => {

        /// Decodes an array value from either unpacked or packed representation. If there are not
        /// exactly the expected number of fields the value is considered to be invalid.
        #[inline]
        pub(super) fn decode_array_either_repr<$($lifetime,)? T, const N: usize, E>(
            _arr: &mut [T; N],
            _buf: Capped<$impl_buf_ty>,
        ) -> Result<(), DecodeError>
        where
            (): $relaxed_value <$($lifetime,)? E, T>,
        {
            Ok(())
        }

        /// Decodes an array value from either packed or unpacked in distinguished mode. If there
        /// are not exactly the expected number of fields the value is considered to be invalid.
        #[inline]
        pub(super) fn decode_distinguished_array_either_repr<$($lifetime,)? T, const N: usize, E>(
            _arr: &mut [T; N],
            _buf: Capped<$impl_buf_ty>,
        ) -> Result<Canonicity, DecodeError>
        where
            (): $relaxed_value <$($lifetime,)? E, T>
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
        distinguished_value: $distinguished_value:ident::$distinguished_value_method:ident,
        distinguished_field: $distinguished_field:ident::$distinguished_field_method:ident,
        buf_ty: $buf_ty:ty,
        impl_buf_ty: $impl_buf_ty:ty,
        $(buf_generic: ($($buf_generic:tt)*),)?
        $(lifetime: $lifetime:lifetime,)?
    ) => {

        impl<$($lifetime,)? T, const N: usize, E>
        $relaxed <$($lifetime,)? Unpacked<E>, [T; N]> for ()
        where
            (): $relaxed_value <$($lifetime,)? E, T> + EmptyState<E, [T; N]>,
        {
            #[inline]
            fn $relaxed_method $($($buf_generic)*)? (
                value: &mut [T; N],
                buf: Capped<$buf_ty>,
            ) -> Result<(), DecodeError> {
                // time regression
                $mode::decode_array_either_repr(value, buf)
            }
        }


        impl<$($lifetime,)? T, const N: usize, E>
        $relaxed <$($lifetime,)? Unpacked<E>, Option<[T; N]>> for ()
        where
            (): $relaxed_value <$($lifetime,)? E, T> + ForOverwrite<E, [T; N]>,
        {
            #[inline]
            fn $relaxed_method $($($buf_generic)*)? (
                _value: &mut Option<[T; N]>,
                _buf: Capped<$buf_ty>,
            ) -> Result<(), DecodeError> {
                Ok(())
            }
        }

        impl<$($lifetime,)? T, const N: usize, E>
        $distinguished <$($lifetime,)? Unpacked<E>, Option<[T; N]>> for ()
        where
            (): ForOverwrite<E, [T; N]>
                + $relaxed_value<$($lifetime,)? E, T>,
        {
            #[inline]
            fn $distinguished_method $($($buf_generic)*)? (
                value: &mut Option<[T; N]>,
                buf: Capped<$buf_ty>,
            ) -> Result<Canonicity, DecodeError> {
                // time regression
                $mode::decode_distinguished_array_either_repr(
                    value.get_or_insert_with(<() as ForOverwrite::<E, _>>::for_overwrite),
                    buf,
                )
            }
        }
    };
}

decoding_modes::__invoke!(impl_decoders, owned);
decoding_modes::__invoke!(impl_decoders, borrowed);
