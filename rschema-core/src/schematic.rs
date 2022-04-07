#![allow(unused_variables)]

use seq_macro::seq;
use paste::paste;

use crate::{
    AdditionalProperties,
    ArrayProp,
    EnumProp,
    Items,
    NumericProp,
    ObjectProp,
    Properties,
    PropType,
    StringProp,
};

/// A data structure that can provide any schema informations.
/// 
/// It is **deprecated** to implement this manually.
/// 
/// Rschema provides `Schematic` implementations for some Rust primitive and standard library types. All of these can be used to generate schema using Rschema.
/// 
/// ## Implementations of `Schematic` provided by Rschema
/// 
/// - **Primitive types**:
///   - bool
///   - i8, i16, i32, i64, isize
///   - u8, u16, u32, u64, usize
///   - f32, f64
///   - char
///   - str(&str)
/// - **Compound types**:
///   - [T; 0]
///   - tuples up to size 12
/// - **Common standard library types**:
///   - String
///   - Option\<T\>
/// - **Wrapper types**:
///   - Box\<T\>
/// - **Collection types**:
///   - Vec\<T\>
/// 
pub trait Schematic {
    fn __type(
        min_length: Option<u64>,
        max_length: Option<u64>,
        pattern: Option<String>,
        format: Option<String>,
        minimum: Option<i64>,
        maximum: Option<i64>,
        multiple_of: Option<i64>,
        exclusive_minimum: Option<i64>,
        exclusive_maximum: Option<i64>,
        min_items: Option<usize>,
        max_items: Option<usize>,
    ) -> PropType;

    fn __type_no_attr() -> PropType {
        Self::__type(
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
    }
}

macro_rules! impl_for_str {
    ($ty:ty) => {
        impl Schematic for $ty {
            fn __type(
                min_length: Option<u64>,
                max_length: Option<u64>,
                pattern: Option<String>,
                format: Option<String>,
                minimum: Option<i64>,
                maximum: Option<i64>,
                multiple_of: Option<i64>,
                exclusive_minimum: Option<i64>,
                exclusive_maximum: Option<i64>,
                min_items: Option<usize>,
                max_items: Option<usize>,
            ) -> PropType {
                PropType::String(StringProp {
                    min_length,
                    max_length,
                    pattern,
                    format,
                    enm: vec![],
                })
            }
        }
    };
}

impl_for_str!(&str);
impl_for_str!(String);

macro_rules! impl_for_num {
    ($ty:ty) => {
        impl Schematic for $ty {
            fn __type(
                min_length: Option<u64>,
                max_length: Option<u64>,
                pattern: Option<String>,
                format: Option<String>,
                minimum: Option<i64>,
                maximum: Option<i64>,
                multiple_of: Option<i64>,
                exclusive_minimum: Option<i64>,
                exclusive_maximum: Option<i64>,
                min_items: Option<usize>,
                max_items: Option<usize>,
            ) -> PropType {
                PropType::Number(NumericProp {
                    minimum,
                    maximum,
                    multiple_of,
                    exclusive_minimum,
                    exclusive_maximum,
                })
            }
        }
    };
}

impl_for_num!(i8);
impl_for_num!(i16);
impl_for_num!(i32);
impl_for_num!(i64);
impl_for_num!(isize);
impl_for_num!(u8);
impl_for_num!(u16);
impl_for_num!(u32);
impl_for_num!(u64);
impl_for_num!(usize);
impl_for_num!(f32);
impl_for_num!(f64);

impl Schematic for char {
    fn __type(
        min_length: Option<u64>,
        max_length: Option<u64>,
        pattern: Option<String>,
        format: Option<String>,
        minimum: Option<i64>,
        maximum: Option<i64>,
        multiple_of: Option<i64>,
        exclusive_minimum: Option<i64>,
        exclusive_maximum: Option<i64>,
        min_items: Option<usize>,
        max_items: Option<usize>,
    ) -> PropType {
        PropType::String(StringProp {
            min_length: Some(1),
            max_length: Some(1),
            pattern,
            format,
            enm: vec![],
        })
    }
}

impl Schematic for bool {
    fn __type(
        min_length: Option<u64>,
        max_length: Option<u64>,
        pattern: Option<String>,
        format: Option<String>,
        minimum: Option<i64>,
        maximum: Option<i64>,
        multiple_of: Option<i64>,
        exclusive_minimum: Option<i64>,
        exclusive_maximum: Option<i64>,
        min_items: Option<usize>,
        max_items: Option<usize>,
    ) -> PropType {
        PropType::Boolean
    }
}

impl Schematic for () {
    fn __type(
        min_length: Option<u64>,
        max_length: Option<u64>,
        pattern: Option<String>,
        format: Option<String>,
        minimum: Option<i64>,
        maximum: Option<i64>,
        multiple_of: Option<i64>,
        exclusive_minimum: Option<i64>,
        exclusive_maximum: Option<i64>,
        min_items: Option<usize>,
        max_items: Option<usize>,
    ) -> PropType {
        PropType::Null
    }
}

macro_rules! impls {
    // $t: Type parameter
    // $c: Comma
    ( $( $t:tt $c:tt )* ) => {
        impl<$($t:Schematic $c)*> Schematic for ($($t $c)*) {
            fn __type(
                min_length: Option<u64>,
                max_length: Option<u64>,
                pattern: Option<String>,
                format: Option<String>,
                minimum: Option<i64>,
                maximum: Option<i64>,
                multiple_of: Option<i64>,
                exclusive_minimum: Option<i64>,
                exclusive_maximum: Option<i64>,
                min_items: Option<usize>,
                max_items: Option<usize>,
            ) -> PropType {
                PropType::Array(ArrayProp {
                    items: Box::new(Items::Tuple(vec![
                        $(
                            <$t as Schematic>::__type_no_attr(),
                        )*
                    ])),
                    min_items,
                    max_items,
                })
            }
        }
    }
}

macro_rules! impls_tuple {
    ($n:expr) => {
        seq!(N in 0..$n {
            paste! {
                impls!( #( [<T~N>], )* );
            }
        });
    };
}

macro_rules! impls_tuple_for {
    ($n:expr) => {
        seq!(N in 1..=$n {
            impls_tuple!(N);
        });
    }
}

// Implementation for tuples with up to 12 members according to `Debug`.
// See https://qiita.com/9laceef/items/e24f9541ef3924112f6b for these macros.
impls_tuple_for!(12);

impl<T: Schematic> Schematic for Option<T> {
    fn __type(
        min_length: Option<u64>,
        max_length: Option<u64>,
        pattern: Option<String>,
        format: Option<String>,
        minimum: Option<i64>,
        maximum: Option<i64>,
        multiple_of: Option<i64>,
        exclusive_minimum: Option<i64>,
        exclusive_maximum: Option<i64>,
        min_items: Option<usize>,
        max_items: Option<usize>,
    ) -> PropType {
        PropType::Enum(EnumProp {
            any_of: vec![
                T::__type_no_attr(),
                PropType::Null,
            ],
        })
    }
}

impl<T: Schematic, const N: usize> Schematic for [T; N] {
    fn __type(
        min_length: Option<u64>,
        max_length: Option<u64>,
        pattern: Option<String>,
        format: Option<String>,
        minimum: Option<i64>,
        maximum: Option<i64>,
        multiple_of: Option<i64>,
        exclusive_minimum: Option<i64>,
        exclusive_maximum: Option<i64>,
        min_items: Option<usize>,
        max_items: Option<usize>,
    ) -> PropType {
        PropType::Array(ArrayProp {
            items: Box::new(Items::Single(T::__type_no_attr())),
            min_items: Some(N),
            max_items: Some(N),
        })
    }
}

impl<T: Schematic> Schematic for Vec<T> {
    fn __type(
        min_length: Option<u64>,
        max_length: Option<u64>,
        pattern: Option<String>,
        format: Option<String>,
        minimum: Option<i64>,
        maximum: Option<i64>,
        multiple_of: Option<i64>,
        exclusive_minimum: Option<i64>,
        exclusive_maximum: Option<i64>,
        min_items: Option<usize>,
        max_items: Option<usize>,
    ) -> PropType {
        PropType::Array(ArrayProp {
            items: Box::new(Items::Single(T::__type_no_attr())),
            min_items,
            max_items,
        })
    }
}

impl<T: Schematic> Schematic for Box<T> {
    fn __type(
        min_length: Option<u64>,
        max_length: Option<u64>,
        pattern: Option<String>,
        format: Option<String>,
        minimum: Option<i64>,
        maximum: Option<i64>,
        multiple_of: Option<i64>,
        exclusive_minimum: Option<i64>,
        exclusive_maximum: Option<i64>,
        min_items: Option<usize>,
        max_items: Option<usize>,
    ) -> PropType {
        T::__type(
            min_length,
            max_length,
            pattern,
            format,
            minimum,
            maximum,
            multiple_of,
            exclusive_minimum,
            exclusive_maximum,
            min_items,
            max_items,
        )
    }
}

impl<V: Schematic, S> Schematic for std::collections::HashMap<String, V, S> {
    fn __type(
        min_length: Option<u64>,
        max_length: Option<u64>,
        pattern: Option<String>,
        format: Option<String>,
        minimum: Option<i64>,
        maximum: Option<i64>,
        multiple_of: Option<i64>,
        exclusive_minimum: Option<i64>,
        exclusive_maximum: Option<i64>,
        min_items: Option<usize>,
        max_items: Option<usize>,
    ) -> PropType {
        PropType::Object(ObjectProp {
            properties: Properties::new(),
            required: vec![],
            additional_properties: Box::new(
                AdditionalProperties::Complex(V::__type_no_attr())
            ),
        })
    }
}
