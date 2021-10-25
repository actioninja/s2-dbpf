use binrw::*;
use crate::constants::data_kinds::FormatKind;

#[binrw]
#[derive(Debug, PartialEq)]
struct Dir {
    entries: Vec<DirEntry>
}

#[binrw]
#[derive(Debug, PartialEq)]
struct DirEntry {
    type_id: FormatKind,
    group_id: u32,
    instance_id_lo: u32,
    instance_id_hi: u32,
    decompressed_size: u32,
}
