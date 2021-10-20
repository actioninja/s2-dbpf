use deku::prelude::*;

const MAGIC_QFS_ID: u16 = 0x10FB;
const COMPRESSION_HEADER_SIZE: u32 = 9;

#[derive(PartialEq, Debug, Copy, Clone, DekuRead, DekuWrite)]
#[deku(endian="little")]
struct CompressionHeader {
    #[deku(bytes = 4)]
    compressed_size: u32,
    #[deku(assert_eq = "0xFB10")]
    compression_id: u16,
    #[deku(endian = "big", bytes = 3)]
    uncompressed_size: u32,
}

#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
struct CompressedFile {
    header: CompressionHeader,
    #[deku(count = "header.compressed_size - 9")]
    data: Vec<u8>,
}

impl CompressedFile {
    fn decompress(&self) -> Vec<u8> {
        let compressed = &self.data;
        let mut decompressed: Vec<u8> = vec![0x00; self.header.uncompressed_size as usize];

        let mut index = 0;
        let mut decomp_index: usize = 0;
        let mut control: u8 = 0;
        println!("compressed length: {}", compressed.len());
        while index < compressed.len() {
            control = compressed[index];
            println!("Control character: 0x{:X?} at pos {:?}", control, index);
            index += 1;

            let control_result: (Option<usize>, Option<(usize, usize)>) = match control {
                0x00..=0x7F => {
                    let control_2 = compressed[index];
                    index += 1;
                    let num_plain_text: usize = (control_2 & 0b0000_0011) as usize;

                    let num_to_copy

                    let offset: usize = ((((control & 0x60) as u32) << 3) + (control_2 as u32) + 1) as usize;
                    let number_copy_offset = (((control & 0x1C) >> 2) + 3) as usize;
                    let start_index: usize = decomp_index - offset;
                    decompressed[decomp_index..decomp_index + number_copy_offset].copy_from_slice(&compressed[start_index..start_index + number_copy_offset]);

                    (Some(num_plain_text), Some((0, 0)))
                }
                0x80..=0xBF => {
                    (None, None)
                }
                0xC0..=0xDF => {
                    (None, None)
                }
                0xE0..=0xFB => {
                    //It just is bit shifted, don't ask me why
                    let number_plain_text = ((control & 0b0001_1111) << 2) as usize;
                    (Some(number_plain_text), None)
                }
                0xFC..=0xFF => {
                    let number_plain_text = (control & 0b0000_0011) as usize;
                    (Some(number_plain_text), None)
                }
                _ => panic!("Invalid control code found!")
            };

            if let Some(plaintext_copy) = control_result.0 {
                println!("Copying: {}", plaintext_copy);
                let src_slice = &compressed[index..index + plaintext_copy];
                decompressed[decomp_index..decomp_index + plaintext_copy].copy_from_slice(src_slice);
                index += number_plain_text;
                decomp_index += number_plain_text;
            }

            if let Some((src_pos, length)) = control_result.1 {
                let dest_pos = decomp_index - src_pos;

                let src_slice = &decompressed[src_pos..src_pos + length];
                decompressed[dest_pos..dest_pos + length].copy_from_slice(src_slice);

                decomp_index += length;

            }
        }


        decompressed
    }
}


fn vec_to_compressed(in_vec: &Vec<u8>) -> Option<CompressedFile> {
    if &in_vec[4..=5] == MAGIC_QFS_ID.to_be_bytes() {
        let (_rest, mut val) = CompressedFile::from_bytes((in_vec.as_ref(), 0)).unwrap();
        return Some(val);
    }

    None
}

mod tests {
    use crate::compression::vec_to_compressed;

    #[test]
    fn is_compressed_works() {
        let data: Vec<u8> = vec![0x30, 0x00, 0x00, 0x00, 0x10, 0xFB, 0x00, 0x01, 0x40, 0xE3, 0x53, 0x65, 0x6D, 0x69, 0x2D, 0x67, 0x6C, 0x6F, 0x62, 0x61, 0x6C, 0x20, 0x66, 0x69, 0x6C, 0x65, 0xAB, 0x40, 0x00, 0x00, 0xE1, 0x0D, 0x50, 0x65, 0x72, 0x73, 0x6F, 0x6E, 0x47, 0x08, 0x41, 0xC2, 0x00, 0x00, 0xEC, 0x73, 0xA3, 0xFC];
        let compressed = vec_to_compressed(&data).unwrap();
        println!("{:X?}", compressed);
        let decompressed = compressed.decompress();
    }
}