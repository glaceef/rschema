#![allow(unused_variables)]

use crate::{
    ArrayProp,
    EnumProp,
    Items,
    NumericProp,
    PropType,
    StringProp,
};

pub trait Schematic {
    fn __type(
        min_length: Option<u64>,
        max_length: Option<u64>,
        pattern: Option<String>,
        format: Option<String>,
        minimum: Option<i64>,
        maximum: Option<i64>,
        multiple_of: Option<u64>,
        exclusive_minimum: Option<bool>,
        exclusive_maximum: Option<bool>,
        min_items: Option<u64>,
        max_items: Option<u64>,
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

macro_rules! gen_string_impl {
    ($ty:ty) => {
        impl Schematic for $ty {
            fn __type(
                min_length: Option<u64>,
                max_length: Option<u64>,
                pattern: Option<String>,
                format: Option<String>,
                minimum: Option<i64>,
                maximum: Option<i64>,
                multiple_of: Option<u64>,
                exclusive_minimum: Option<bool>,
                exclusive_maximum: Option<bool>,
                min_items: Option<u64>,
                max_items: Option<u64>,
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

macro_rules! gen_number_impl {
    ($ty:ty) => {
        impl Schematic for $ty {
            fn __type(
                min_length: Option<u64>,
                max_length: Option<u64>,
                pattern: Option<String>,
                format: Option<String>,
                minimum: Option<i64>,
                maximum: Option<i64>,
                multiple_of: Option<u64>,
                exclusive_minimum: Option<bool>,
                exclusive_maximum: Option<bool>,
                min_items: Option<u64>,
                max_items: Option<u64>,
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

gen_string_impl!(&str);
gen_string_impl!(String);

/*
impl Schematic for String {
    fn __type(
        min_length: Option<u64>,
        max_length: Option<u64>,
        pattern: Option<String>,
        format: Option<String>,
        minimum: Option<i64>,
        maximum: Option<i64>,
        multiple_of: Option<u64>,
        exclusive_minimum: Option<bool>,
        exclusive_maximum: Option<bool>,
        min_items: Option<u64>,
        max_items: Option<u64>,
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
*/

gen_number_impl!(i8);
gen_number_impl!(i16);
gen_number_impl!(i32);
gen_number_impl!(i64);
gen_number_impl!(isize);
gen_number_impl!(u8);
gen_number_impl!(u16);
gen_number_impl!(u32);
gen_number_impl!(u64);
gen_number_impl!(usize);

/*
impl Schematic for i32 {
    fn __type(
        min_length: Option<u64>,
        max_length: Option<u64>,
        pattern: Option<String>,
        format: Option<String>,
        minimum: Option<i64>,
        maximum: Option<i64>,
        multiple_of: Option<u64>,
        exclusive_minimum: Option<bool>,
        exclusive_maximum: Option<bool>,
        min_items: Option<u64>,
        max_items: Option<u64>,
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
*/

impl Schematic for bool {
    fn __type(
        min_length: Option<u64>,
        max_length: Option<u64>,
        pattern: Option<String>,
        format: Option<String>,
        minimum: Option<i64>,
        maximum: Option<i64>,
        multiple_of: Option<u64>,
        exclusive_minimum: Option<bool>,
        exclusive_maximum: Option<bool>,
        min_items: Option<u64>,
        max_items: Option<u64>,
    ) -> PropType {
        PropType::Boolean
    }
}

impl<T: Schematic> Schematic for Option<T> {
    fn __type(
        min_length: Option<u64>,
        max_length: Option<u64>,
        pattern: Option<String>,
        format: Option<String>,
        minimum: Option<i64>,
        maximum: Option<i64>,
        multiple_of: Option<u64>,
        exclusive_minimum: Option<bool>,
        exclusive_maximum: Option<bool>,
        min_items: Option<u64>,
        max_items: Option<u64>,
    ) -> PropType {
        PropType::Enum(EnumProp {
            any_of: vec![
                T::__type_no_attr(),
                PropType::Null,
            ],
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
        multiple_of: Option<u64>,
        exclusive_minimum: Option<bool>,
        exclusive_maximum: Option<bool>,
        min_items: Option<u64>,
        max_items: Option<u64>,
    ) -> PropType {
        PropType::Array(ArrayProp {
            items: Box::new(Items::Single(T::__type_no_attr())),
            min_items,
            max_items,
        })
    }
}