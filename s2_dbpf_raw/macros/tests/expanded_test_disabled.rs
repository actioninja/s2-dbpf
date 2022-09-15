////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

/*
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use s2_dbpf_derive::DbpfKindsDerive;
pub struct Size(u32);
pub struct Unimplemented(u8);
pub struct Entry1AssocType(u8);
pub struct Entry2AssocType(u8);
pub enum Test {
    //#[dbpf(short_name = "ENTR", id = 1230_2301u32, kind_type = "Entry1AssocType")]
    Entry1,
    //#[dbpf(short_name = "ENTE", id = 1233_2301u32, kind_type = "Entry2AssocType")]
    Entry2,
}
impl Test {
    pub const fn from_id(from: u32) -> std::option::Option<Self> {
        match from {
            1230_2301u32 => ::std::option::Option::Some(Self::Entry1),
            1233_2301u32 => ::std::option::Option::Some(Self::Entry2),
            _ => ::std::option::Option::None,
        }
    }
    pub const fn id(&self) -> u32 {
        match self {
            Self::Entry1 => 1230_2301u32,
            Self::Entry2 => 1233_2301u32,
        }
    }
    pub const fn short_name(&self) -> &'static str {
        match self {
            Self::Entry1 => "ENTR",
            Self::Entry2 => "ENTE",
        }
    }
}
pub enum TestKind {
    Entry1(Entry1AssocType),
    Entry2(Entry2AssocType),
}
impl ::std::convert::From<Entry1AssocType> for TestKind {
    fn from(item: Entry1AssocType) -> Self {
        Self::Entry1(item)
    }
}
impl ::std::convert::TryFrom<TestKind> for Entry1AssocType {
    type Error = &'static str;
    fn try_from(item: TestKind) -> ::std::result::Result<Self, Self::Error> {
        match item {
            TestKind::Entry1(inner) => ::std::result::Result::Ok(inner),
            _ => ::std::result::Result::Err("Incorrect variant found in the enum"),
        }
    }
}
impl ::std::convert::From<Entry2AssocType> for TestKind {
    fn from(item: Entry2AssocType) -> Self {
        Self::Entry2(item)
    }
}
impl ::std::convert::TryFrom<TestKind> for Entry2AssocType {
    type Error = &'static str;
    fn try_from(item: TestKind) -> ::std::result::Result<Self, Self::Error> {
        match item {
            TestKind::Entry2(inner) => ::std::result::Result::Ok(inner),
            _ => ::std::result::Result::Err("Incorrect variant found in the enum"),
        }
    }
}
impl TestKind {
    pub fn parse<R: ::std::io::Read + ::std::io::Seek>(
        reader: &mut R,
        id: Test,
        options: &::binrw::ReadOptions,
        size: Size,
    ) -> ::binrw::BinResult<Self> {
        match id {
            Test::Entry1 => Ok(Test::read_options(reader, options, ())?.into()),
            Test::Entry2 => Ok(Test::read_options(reader, options, ())?.into()),
            _ => ::std::result::Result::Ok(
                Unimplemented::read_options(reader, options, (size.0 as usize,))?.into(),
            ),
        }
    }
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker]
pub const main: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("main"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(main())),
};
#[allow(dead_code)]
fn main() {
    let new = Test::Entry1;
}
#[rustc_main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&main])
}
 */
