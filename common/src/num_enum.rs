use std::fmt::{Debug, Display, Formatter};

pub trait Repr: 'static + Debug + Display {}
impl Repr for u8 {}
impl Repr for u16 {}
impl Repr for u32 {}
impl Repr for u64 {}
impl Repr for i8 {}
impl Repr for i16 {}
impl Repr for i32 {}
impl Repr for i64 {}

#[derive(Debug)]
pub struct NumEnumFromError {
    type_name: &'static str,
    value: Box<dyn Repr>,
}

impl Display for NumEnumFromError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Num enum '{}' does not contain a variant for value {}",
            self.type_name, self.value
        )
    }
}

impl NumEnumFromError {
    pub fn new<T: NumEnum>(value: T::Repr) -> Self {
        Self {
            type_name: std::any::type_name::<T>(),
            value: Box::new(value),
        }
    }
}

pub trait NumEnum {
    type Repr: Repr;
}

#[macro_export]
macro_rules! num_enum {
    (
        #[repr($repr:ty)]
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $($variant:ident $(= $value:literal)? ),*
        }
    ) => {
        $(#[$meta])*
        #[repr($repr)]
        $vis enum $name {
            $($variant $( = $value)? ),*
        }

        impl $crate::num_enum::NumEnum for $name {
            type Repr = $repr;
        }

        impl $name {
            const COUNT: usize = [ $(Self::$variant),* ].len();
            pub const VALUES: [$name;Self::COUNT] = [ $(Self::$variant),* ];
            pub const NUM_VALUES: [$repr;Self::COUNT] = [ $(Self::$variant as $repr),* ];
        }

        impl From<$name> for $repr {
            fn from(val: $name) -> $repr {
                val as $repr
            }
        }

        impl TryFrom<$repr> for $name {
            type Error = $crate::num_enum::NumEnumFromError;
            #[allow(non_upper_case_globals)]
            fn try_from(val: $repr) -> Result<$name, Self::Error> {
                $(
                    const $variant: $repr = $name::$variant as $repr;
                )*
                match val {
                    $(
                        $variant => Ok(Self::$variant),
                    )*
                    v => Err(Self::Error::new::<Self>(v)),
                }
            }
        }

        impl PartialEq<$repr> for $name {
            fn eq(&self, other: & $repr) -> bool {
                *self as $repr == *other
            }
        }
    }
}
