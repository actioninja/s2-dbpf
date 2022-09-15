////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

use binrw::{binrw, BinRead};
use s2_dbpf_derive::DbpfKindsDerive;

#[derive(Copy, Clone)]
pub struct ParserArgs;

#[binrw]
#[derive(Copy, Clone)]
#[br(import_raw(args: ParserArgs))]
pub struct Size(u32);

#[binrw]
#[derive(Copy, Clone)]
#[br(import_raw(args: ParserArgs))]
pub struct Unimplemented(u8);

#[binrw]
#[derive(Copy, Clone)]
#[br(import_raw(args: ParserArgs))]
pub struct Entry1AssocType(u8);

#[binrw]
#[derive(Copy, Clone)]
#[br(import_raw(args: ParserArgs))]
pub struct Entry2AssocType(u8);

#[derive(DbpfKindsDerive)]
#[repr(u32)]
pub enum Test {
    #[dbpf(short_name = "ENTR", kind_type = "Entry1AssocType")]
    Entry1 = 1230_2301u32,
    #[dbpf(short_name = "ENTE")]
    Entry2 = 1233_2301u32,
    #[dbpf(short_name = "UIMP", kind_type = "Unimplemented")]
    Unimplemented = 0xFFFF_FFFFu32,
}

#[test]
fn main() {
    let new = Test::Entry1;
}
