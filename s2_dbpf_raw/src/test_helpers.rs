////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

#![allow(unused_imports)]

macro_rules! test_parsing {
    ($data:expr, $types:expr, $intype:ident, $name:ident) => {
        paste::paste! {
            #[test]
            fn [<$name _parse>]() {
                let pre = $data;

                let actual: $intype = Cursor::new(pre).read_le().unwrap();
                let expected = $types;

                assert_eq!(actual, expected);
            }

            #[test]
            fn [<$name _write>]() {
                let pre = $types;
                let mut cursor = Cursor::new(vec![]);
                cursor.write_le(&pre).unwrap();

                let expected = $data;

                assert_eq!(&cursor.into_inner()[..], expected);
            }

            #[test_strategy::proptest]
            fn [<$name _symmetrical>](x: $intype) {
                let mut cursor = Cursor::new(vec![]);
                cursor.write_le(&x)?;

                let _output = cursor.get_ref();

                cursor.set_position(0);

                let out: $intype = cursor.read_le()?;
                prop_assert_eq!(out, x)
            }
        }
    };

    ($data:expr, $types:expr, $intype:ident, $name:ident, $args:expr) => {
        paste::paste! {
            #[test]
            fn [<$name _parse>]() {
                let pre = $data;

                let actual: $intype = Cursor::new(pre).read_le_args($args).unwrap();
                let expected = $types;

                assert_eq!(actual, expected);
            }

            #[test]
            fn [<$name _write>]() {
                let pre = $types;
                let mut cursor = Cursor::new(vec![]);
                cursor.write_le_args(&pre, $args).unwrap();

                let expected = $data;

                assert_eq!(&cursor.into_inner()[..], expected);
            }

            #[proptest]
            fn [<$name _symmetrical>](x: $intype) {
                let mut cursor = Cursor::new(vec![]);
                cursor.write_le_args(&x, $args)?;

                let _output = cursor.get_ref();

                cursor.set_position(0);

                let out: $intype = cursor.read_le_args($args)?;
                prop_assert_eq!(out, x)
            }
        }
    };
}
pub(crate) use test_parsing;

//TODO: learn macros better
macro_rules! test_parsing_bhav_ins {
    ($data:expr, $types:expr, $intype:ident, $name:ident, $args:expr) => {
        paste::paste! {
            #[test]
            fn [<$name _parse>]() {
                let pre = $data;

                let actual: $intype = Cursor::new(pre).read_le_args($args).unwrap();
                let expected = $types;

                assert_eq!(actual, expected);
            }

            #[test]
            fn [<$name _write>]() {
                let pre = $types;
                let mut cursor = Cursor::new(vec![]);
                cursor.write_le_args(&pre, $args).unwrap();

                let expected = $data;

                assert_eq!(&cursor.into_inner()[..], expected);
            }

            #[proptest]
            fn [<$name _symmetrical>](#[strategy(any_with::<$intype>($args))] x: $intype) {
                let mut cursor = Cursor::new(vec![]);
                cursor.write_le_args(&x, $args)?;

                let _output = cursor.get_ref();

                cursor.set_position(0);

                let out: $intype = cursor.read_le_args($args)?;
                prop_assert_eq!(out, x)
            }
        }
    };
}
pub(crate) use test_parsing_bhav_ins;
