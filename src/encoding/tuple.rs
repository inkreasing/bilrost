//! Every tuple type starting at arity 1 and up to arity 12 implements ValueEncoder for the encoding
//! (E1, ... EN) where its elements are encoded with the corresponding sub-encoding. The
//! representation on the wire is exactly the same as if it were a message type that had fields with
//! tags 0 through arity-minus-1.
//!
//! Note again that these tags start at zero, not at 1 as they do by default when deriving `Message`
//! for a struct. This is for similar reasons; struct fields "feel" more natural when numbered
//! ordinally, but tuple types in rust are *indexed* starting at zero so we prefer to maintain
//! parity with that.
//!
//! The arity-zero unit tuple encodes the same way, but it does so independently of this file.
//! Because it has no fields, it has a privileged lack of ambiguity regarding *how* it will encode,
//! where every other tuple type might appear differently when included in a message depending on
//! which encodings are chosen for each member. For this reason, it is the only tuple type that
//! implements `Message` itself, and it stands as the prototype for a message with no defined
//! fields.

use crate::encoding::{
    EmptyState, ForOverwrite,
};

macro_rules! implement_core_empty_state_rules {
    (
        $encoding:ty
        $(, with generics ($($impl_generics:tt)*))?
    ) => {
        impl<$($($impl_generics)*,)? __T, const __N: usize>
        $crate::encoding::ForOverwrite<$encoding, [__T; __N]> for ()
        where
            (): $crate::encoding::ForOverwrite<$encoding, __T>,
        {
            #[inline]
            fn for_overwrite() -> [__T; __N] {
                ::core::array::from_fn(|_| {
                    <() as $crate::encoding::ForOverwrite<$encoding, __T>>::for_overwrite()
                })
            }
        }

        impl<$($($impl_generics)*,)? __T, const __N: usize>
        $crate::encoding::EmptyState<$encoding, [__T; __N]> for ()
        where
            (): $crate::encoding::EmptyState<$encoding, __T>,
        {
        }
    }
}

macro_rules! impl_tuple {
    (
        ($($numbers:tt),*),
        ($($letters:ident),*),
        ($($encodings:ident),*),
    ) => {
        implement_core_empty_state_rules!(($($encodings,)*), with generics ($($encodings),*));

        impl<$($letters,)* $($encodings,)*> ForOverwrite<($($encodings,)*), ($($letters,)*)> for ()
        where
            $((): ForOverwrite<$encodings, $letters>,)*
        {
            #[inline]
            fn for_overwrite() -> ($($letters,)*) {
                todo!()
            }
        }

        impl<$($letters,)* $($encodings,)*> EmptyState<($($encodings,)*), ($($letters,)*)> for ()
        where
            $((): EmptyState<$encodings, $letters>,)*
        {
        }
    }
}

impl_tuple!(
    (0),           //
    (A),           //
    (Ae),          //
);
impl_tuple!(
    (0, 1),        //
    (A, B),        //
    (Ae, Be),      //
);
impl_tuple!(
    (0, 1, 2),     //
    (A, B, C),     //
    (Ae, Be, Ce),  //
);
impl_tuple!(
    (0, 1, 2, 3),     //
    (A, B, C, D),     //
    (Ae, Be, Ce, De), //
);
impl_tuple!(
    (0, 1, 2, 3, 4),      //
    (A, B, C, D, E),      //
    (Ae, Be, Ce, De, Ee), //
);
impl_tuple!(
    (0, 1, 2, 3, 4, 5),       //
    (A, B, C, D, E, F),       //
    (Ae, Be, Ce, De, Ee, Fe), //
);
impl_tuple!(
    (0, 1, 2, 3, 4, 5, 6),        //
    (A, B, C, D, E, F, G),        //
    (Ae, Be, Ce, De, Ee, Fe, Ge), //
);
impl_tuple!(
    (0, 1, 2, 3, 4, 5, 6, 7),         //
    (A, B, C, D, E, F, G, H),         //
    (Ae, Be, Ce, De, Ee, Fe, Ge, He), //
);
impl_tuple!(
    (0, 1, 2, 3, 4, 5, 6, 7, 8),          //
    (A, B, C, D, E, F, G, H, I),          //
    (Ae, Be, Ce, De, Ee, Fe, Ge, He, Ie), //
);
impl_tuple!(
    (0, 1, 2, 3, 4, 5, 6, 7, 8, 9),           //
    (A, B, C, D, E, F, G, H, I, J),           //
    (Ae, Be, Ce, De, Ee, Fe, Ge, He, Ie, Je), //
);
impl_tuple!(
    (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10),           //
    (A, B, C, D, E, F, G, H, I, J, K),            //
    (Ae, Be, Ce, De, Ee, Fe, Ge, He, Ie, Je, Ke), //
);
impl_tuple!(
    (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11),           //
    (A, B, C, D, E, F, G, H, I, J, K, L),             //
    (Ae, Be, Ce, De, Ee, Fe, Ge, He, Ie, Je, Ke, Le), //
);
impl_tuple!(
    (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12),           //
    (A, B, C, D, E, F, G, H, I, J, K, L, M),             //
    (Ae, Be, Ce, De, Ee, Fe, Ge, He, Ie, Je, Ke, Le, Me), //
);
impl_tuple!(
    (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13),           //
    (A, B, C, D, E, F, G, H, I, J, K, L, M, N),             //
    (Ae, Be, Ce, De, Ee, Fe, Ge, He, Ie, Je, Ke, Le, Me, Ne), //
);

