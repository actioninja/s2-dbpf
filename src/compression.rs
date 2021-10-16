
const MAGIC_QFS_ID: u16 = 0x10FB;

#[derive(PartialEq, Debug, DekuRead, DekuWrite, Copy, Clone)]
struct CompressionHeader {
    compressed_size: u32,
    compression_id: u16,
    #[deku(bytes = 3)]
    uncompressed_size: u32,
}

#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
struct CompressedFile {
    header: CompressionHeader,
    data: Vec<u8>,
}

fn decompress_dbpf(in_file: &CompressedFile) -> Vec<u8> {
    let mut decompressed = in_file.data.to_vec();

    let mut index = 0;
    while index < decompressed.len() {
    }

    decompressed
}

