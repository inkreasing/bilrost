use crate::encoding::{
    EmptyState, ForOverwrite,
};
use crate::encoding::{
    Decoder,
    DistinguishedDecoder,
    ValueDecoder,
};

pub struct Unpacked<E>(E);

pub(super) fn decode_array_either_repr<T, E>()
where
    (): ValueDecoder<E, T>,
{}

impl<T, const N: usize, E>
Decoder <Unpacked<E>, [T; N]> for ()
where
    (): ValueDecoder<E, T>
    + EmptyState<E, [T; N]>,
{
    fn decode(
    ) {
        // time regression
        // specifying the generic parameters to T or E reduces the slowdown
        decode_array_either_repr::<_, _>()
    }
}

impl<T, const N: usize, E>
Decoder <Unpacked<E>, Option<[T; N]>> for ()
where
    (): ForOverwrite<E, [T; N]>,
{
    fn decode(
    ) {}
}

impl<T, const N: usize, E>
DistinguishedDecoder <Unpacked<E>, Option<[T; N]>> for ()
where
    (): ForOverwrite<E, [T; N]>
        + ValueDecoder<E, T>,
{
    fn decode_distinguished(
    ) {
        // time regression
        // specifying the generic parameters to T or E reduces the slowdown
        decode_array_either_repr::<_, _>()
    }
}
