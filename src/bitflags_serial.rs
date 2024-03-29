//! A wrapper around the standard bitflags macro that implements serde
//! serialization/deserialization by representing a bit mask as a
//! sequence of constant values defined by the user.

#[doc(hidden)]
#[allow(unused_imports)]
pub use core as _core;
#[doc(hidden)]
#[allow(unused_imports)]
pub use serde as _serde;

#[doc(hidden)]
pub struct _SingleBit<T>(pub T);

impl<'de, T> _serde::Deserialize<'de> for _SingleBit<T>
    where
        T: Default + _serde::de::Visitor<'de, Value = T>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: _serde::Deserializer<'de>
    {
        deserializer
            .deserialize_identifier(T::default())
            .map(Self)
    }
}


#[macro_export]
macro_rules! bitflags_serial {
    (
        $(#[$outer:meta])*
        pub struct $BitFlags:ident: $T:ty {
            $(
                $(#[$inner:ident $($args:tt)*])*
                const $Flag:ident = $value:expr;
            )+
        }
    ) => {
        bitflags! {
            $(#[$outer])*
            pub struct $BitFlags: $T {
                $(
                    $(#[$inner $($args)*])*
                    const $Flag = $value;
                )+
            }
        }

        impl Default for $BitFlags {
            fn default() -> Self {
                Self { bits: 0 }
            }
        }

        impl<'de> $crate::bitflags_serial::_serde::de::Visitor<'de> for $BitFlags {
            type Value = Self;
            fn expecting(&self, formatter: &mut $crate::bitflags_serial::_core::fmt::Formatter) -> $crate::bitflags_serial::_core::fmt::Result {
                write!(formatter, "sequence of bit constants")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: $crate::bitflags_serial::_serde::de::SeqAccess<'de>
            {
                let mut bits = 0;
                while let Some(single) = seq.next_element::<$crate::bitflags_serial::_SingleBit<Self>>()? {
                    bits |= single.0.bits;
                }
                Ok(Self { bits })
            }
            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: $crate::bitflags_serial::_serde::de::Error,
            {
                match $crate::bitflags_serial::_core::str::from_utf8(v).unwrap() {
                    $(
                        stringify!($Flag) => Ok(Self { bits: $value } ),
                    )+
                    other => Err(E::unknown_variant(other, &[]))
                }
            }
        }

        impl<'de> $crate::bitflags_serial::_serde::Deserialize<'de> for $BitFlags {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: $crate::bitflags_serial::_serde::Deserializer<'de>
            {
                deserializer.deserialize_seq(Self { bits: 0 })
            }
        }

        impl $crate::bitflags_serial::_serde::Serialize for $BitFlags {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where S: $crate::bitflags_serial::_serde::Serializer
            {
                use $crate::bitflags_serial::_serde::ser::SerializeSeq;

                $(
                    #[allow(non_snake_case)]
                    struct $Flag;
                    impl $crate::bitflags_serial::_serde::Serialize for $Flag {
                        fn serialize<Ss>(&self, serializer: Ss) -> Result<Ss::Ok, Ss::Error>
                            where Ss: $crate::bitflags_serial::_serde::Serializer
                        {
                            serializer.serialize_unit_variant("", 0, stringify!($Flag))
                        }
                    }
                )+

                // the `__BitFlags` trait is copied from `Debug` implementation of `bitflags!`

                // Unconditionally define a check for every flag, even disabled
                // ones.
                #[allow(non_snake_case)]
                trait __BitFlags {
                    $(
                        #[inline]
                        fn $Flag(&self) -> bool { false }
                    )+
                }

                // Conditionally override the check for just those flags that
                // are not #[cfg]ed away.
                impl __BitFlags for $BitFlags {
                    $(
                        __impl_bitflags! {
                            #[allow(deprecated)]
                            #[inline]
                            $(? #[$attr $($args)*])*
                            fn $Flag(&self) -> bool {
                                self.bits & Self::$Flag.bits == Self::$Flag.bits
                            }
                        }
                    )+
                }

                let mut seq = serializer.serialize_seq(std::option::Option::None)?;
                $(
                    if <Self as __BitFlags>::$Flag(self) {
                        seq.serialize_element(&$Flag)?;
                    }
                )+
                seq.end()
            }
        }
    };
}