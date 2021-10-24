mod constants;
mod types;
mod dbpf;
mod compression;
mod helpers;

use binrw::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

#[binrw]
#[derive(Debug, PartialEq)]
#[brw(little, magic = b"DBPF")]
struct DbpfHeader {
    major_version: u32,
    minor_version: u32,
    major_user_version: u32,
    minor_user_version: u32,
    flags: u32,
    date_created: u32,
    date_modified: u32,
    index_major_version: u32,
    index_entry_count: u32,
    first_index_entry_offset: u32,
    index_size: u32,
    hole_entry_count: u32,
    hole_offset: u32,
    hole_size: u32,
    #[brw(pad_after = 32)]
    index_minor_version: u32,
}

struct DbpfIndexEntry {
    file_type: DbpfFileType,
    group_id: u32,
    instance_id_lo: u32,
    instance_id_hi: u32,
    payload: Vec<u8>,
}

enum DbpfFileType {
    Bhav
}