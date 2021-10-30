////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

#[macro_expert]
macro_rules! test_parsing {
    ($data:expr, $types:expr, $intype:ident, $name:literal) => {
        #[test]
        fn parses_$name() {
            let pre = $data;

            let actual: $intype = Cursor::new(test_record).read_le().unwrap();
            let expected = #types;

            assert_eq!(actual, expected);
        }

        #[test]
        fn writes_$name() {
            let pre = $types;
            let mut cursor = Cursor::new(vec![]);
            cursor.write_le(&pre).unwrap();

            let expected = $data;

            assert_eq!(&cursor.into_inner()[..], expected);
        }

        #[proptest]
        fn $name_symmetrical(x: $intype) {
            let mut cursor = Cursor::new(vec![]);
            cursor.write_le(&x).unwrap();

            let output = cursor.get_ref();

            cursor.set_position(0);

            let out: $intype = cursor.read_le().unwrap();
            prop_assert_eq!(out, x)
        }
    };
}
