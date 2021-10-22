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
    fn compress(in_vec: &Vec<u8>) -> CompressedFile {
        todo!("Todo!")
    }

    fn decompress(&self) -> Vec<u8> {
        let compressed = &self.data;
        let mut decompressed: Vec<u8> = vec![0x00; self.header.uncompressed_size as usize];

        let mut index: usize = 0;
        let mut decomp_index: usize = 0;
        println!("compressed length: {}", compressed.len());
        while index < compressed.len() {
            let control = &compressed[index];
            println!("Control character: {:#X}", control);

            let control_result: (Option<usize>, Option<(usize, usize)>) = match control {
                0x00..=0x7F => {
                    let control_slice = &compressed[index..index + 2];
                    index += 2;
                    let num_plain_text: usize = (control_slice[0] & 0b0000_0011) as usize;

                    let offset: usize = (((((control_slice[0] & 0b0110_0000) as u16) << 3) | (control_slice[1] as u16)) + 1) as usize;
                    let number_copy_offset = (((control_slice[0] & 0b0001_1100) >> 2) + 3) as usize;

                    (Some(num_plain_text), Some((offset, number_copy_offset)))
                }
                0x80..=0xBF => {
                    let control_slice = &compressed[index..index + 3];
                    index += 3;

                    let num_plain_text: usize = ((control_slice[1] & 0b1100_0000) >> 6) as usize;

                    let offset: usize = ((((control_slice[1] & 0b0011_1111) as u16) << 8) | (control_slice[2] as u16)) as usize;
                    let num_to_copy: usize = (control_slice[0] & 0b0011_1111) as usize;

                    (Some(num_plain_text), Some((offset, num_to_copy)))
                }
                0xC0..=0xDF => {
                    let control_slice = &compressed[index..index + 4];
                    index += 4;

                    let num_plain_text: usize = (control_slice[0] & 0b0000_0011) as usize;

                    let offset: usize = ((((control_slice[0] & 0b0001_0000) as u32) << 12) | ((control_slice[1] as u32) << 8) | (control_slice[2] as u32) + 1) as usize;
                    let num_to_copy: usize = ((((control_slice[0] & 0b0000_1100) as u16) << 6) | (control_slice[3] as u16) + 5) as usize;

                    (Some(num_plain_text), Some((offset, num_to_copy)))
                }
                0xE0..=0xFB => {
                    let control = compressed[index];
                    index += 1;
                    //It just is bit shifted, don't ask me why
                    let number_plain_text = (((control & 0b0001_1111) << 2) + 4) as usize;
                    (Some(number_plain_text), None)
                }
                0xFC..=0xFF => {
                    let control = compressed[index];
                    index += 1;
                    let number_plain_text = (control & 0b0000_0011) as usize;
                    (Some(number_plain_text), None)
                }
            };

            if let Some(plaintext_copy) = control_result.0 {
                println!("Copying plaintext of length: {}", plaintext_copy);
                let src_slice = &compressed[index..index + plaintext_copy];
                decompressed[decomp_index..decomp_index + plaintext_copy].copy_from_slice(src_slice);
                index += plaintext_copy;
                decomp_index += plaintext_copy;
            }

            if let Some((src_pos, length)) = control_result.1 {
                println!("Copying length {} to {}", length, src_pos);
                println!("decomp index: {}", decomp_index);
                let dest_pos = decomp_index - src_pos;
                println!("dest pos: {}", dest_pos);

                //If the sections do not overlap, we can do ultra fast
                //memory section copy
                if (dest_pos + length) < src_pos {
                    let (dst, src) = decompressed.split_at_mut(src_pos);

                    dst[dest_pos..dest_pos + length].copy_from_slice(&src[..length]);
                }
                //If they do, slower character by character copy is required
                else {
                    for i in 0..length {
                        decompressed[dest_pos + i as usize] = decompressed[src_pos + i as usize];
                    }
                }

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
    use crate::compression::{CompressedFile, vec_to_compressed};

    #[test]
    fn is_compressed_works() {
        let data: Vec<u8> = vec![0x30, 0x00, 0x00, 0x00, 0x10, 0xFB, 0x00, 0x01, 0x40, 0xE3, 0x53, 0x65, 0x6D, 0x69, 0x2D, 0x67, 0x6C, 0x6F, 0x62, 0x61, 0x6C, 0x20, 0x66, 0x69, 0x6C, 0x65, 0xAB, 0x40, 0x00, 0x00, 0xE1, 0x0D, 0x50, 0x65, 0x72, 0x73, 0x6F, 0x6E, 0x47, 0x08, 0x41, 0xC2, 0x00, 0x00, 0xEC, 0x73, 0xA3, 0xFC];
        let compressed = vec_to_compressed(&data).unwrap();
        println!("{:X?}", compressed);
        let decompressed = compressed.decompress();
    }

    #[test]
    fn compression_then_decompression_gives_same_vec() {
        let test_vec = b"This is a test vector with a repeated phrase! This is a test vector with a repeated phrase!".to_vec();

        let compressed = CompressedFile::compress(&test_vec);
        let decompressed = compressed.decompress();

        assert_eq!(test_vec, decompressed)

    }
}
