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

mod compression_keys {
    use std::intrinsics::offset;

    trait CompressionKey {
        fn length(&self) -> usize;

        fn plaintext_to_copy(&self) -> Option<usize> {
            None
        }

        fn offset_copy(&self) -> Option<(usize, usize)> {
            None
        }
    }

    //TODO: Make a spicy proc macro to cut down on the boilerplate here
    // Ideally instead of having the giant trait definitions below each type, it could
    // just have an extra attribute macro

    //0x00-0x7F
    #[deku_derive(DekuRead, DekuWrite)]
    #[derive(Debug, PartialEq)]
    struct KeyRange1 {
        #[deku(bits = 1, assert_eq="0")]
        header: u8,
        #[deku(bits = 2)]
        first_chunk_offset: u8,
        //In the spec to add 3
        #[deku(bits = 3, map = "|field: u8| -> Result<_, DekuError> { Ok(field + 3) }")]
        num_to_copy: u8,
        #[deku(bits = 2)]
        plaintext_to_copy: u8,
        #[deku(bits = 8)]
        second_chunk_offset: u8,
        #[deku(skip, default = "")]
        offset: u16
    }

    impl CompressionKey for KeyRange1 {
        fn length(&self) -> usize { 2 }
    }

    //0x80-0xBF
    #[derive(Debug, PartialEq, DekuRead, DekuWrite)]
    struct KeyRange2 {
        #[deku(bits = 2, assert_eq = "0b01")]
        header: u8,
        #[deku(bits = 6)]
        num_to_copy: u8,
        #[deku(bits = 2)]
        plaintext_to_copy: u8,
        #[deku(bits = 14)]
        offset: u16,
    }

    impl CompressionKey for KeyRange2 {
        fn length(&self) -> usize {
            3
        }

        fn plaintext_to_copy(&self) -> Option<usize> {
            Some(self.plaintext_to_copy as usize)
        }

        fn offset_copy(&self) -> Option<(usize, usize)> {
            Some((self.offset as usize, self.num_to_copy as usize))
        }
    }

    //0xC0-0xDF
    #[derive(Debug, PartialEq, DekuRead, DekuWrite)]
    struct KeyRange3 {
        #[deku(bits = 3, assert_eq = "0b110")]
        header: u8,
        #[deku(bits = 1, update = "(self.offset >> 16) as u8")]
        offset_first_chunk: u8,
        #[deku(bits = 2, update = "(self.num_to_copy >> 8) as u8")]
        num_to_copy_first_chunk: u8,
        #[deku(bits = 2)]
        plaintext_to_copy: u8,
        #[deku(bits = 16, update = "self.offset as u16")]
        offset_second_chunk: u16,
        #[deku(bits = 8, update = "self.num_to_copy as u8")]
        num_to_copy_second_chunk: u8,
        offset: u32,
        num_to_copy: u16,
    }

    impl CompressionKey for KeyRange3 {
        fn length(&self) -> usize {
            4
        }

        fn plaintext_to_copy(&self) -> Option<usize> {
            Some(self.plaintext_to_copy as usize)
        }

        fn offset_copy(&self) -> Option<(usize, usize)> {
            Some((self.offset as usize, self.num_to_copy as usize))
        }
    }


    //0xE0-0xFC
    #[derive(Debug, PartialEq, DekuRead, DekuWrite)]
    struct KeyRange4 {
        #[deku(bits = 3, assert_eq="0b00000_111")]
        header: u8,
        //This guys gets punched over to the left two bits, don't ask me why it's in the spec
        #[deku(bits = 5, map = "|field: u8| -> Result<_, DekuError> { Ok(field << 2) }")]
        plaintext_to_copy: u8,
    }

    impl CompressionKey for KeyRange4 {
        fn length(&self) -> usize { 1 }

        fn plaintext_to_copy(&self) -> Option<usize> {
            Some(self.plaintext_to_copy as usize)
        }
    }

    // 0xFD-0xFF
    #[derive(Debug, PartialEq, DekuRead, DekuWrite)]
    struct KeyRange5 {
        #[deku(bits = 6, assert_eq = "0b00_111111")]
        header: u8,
        #[deku(bits = 2)]
        plaintext_to_copy: u8,
    }

    impl CompressionKey for KeyRange5 {
        fn length(&self) -> usize {
            1
        }

        fn plaintext_to_copy(&self) -> Option<usize> {
            Some(self.plaintext_to_copy as usize)
        }
    }

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