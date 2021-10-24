use binrw::*;
use binrw::io::Cursor;
use std::str;
use crate::helpers::{copy_within_slice, U24};

const MAGIC_QFS_ID: u16 = 0x10FB;
const COMPRESSION_HEADER_SIZE: u32 = 9;

#[binrw]
#[brw(little)]
#[derive(PartialOrd, PartialEq, Debug)]
struct CompressionHeader {
    compressed_size: u32,
    #[brw(big, magic = 0x10FBu16)]
    #[br(map = |x: U24| *x)]
    #[bw(map = |x: &u32| U24(*x))]
    uncompressed_size: u32,
}

#[binrw]
#[derive(PartialEq, Debug)]
struct CompressedFile {
    header: CompressionHeader,
    //compressed_size includes the length of the header, and the header is always 9 bytes long
    #[br(count = (header.compressed_size - 9))]
    data: Vec<u8>,
}

impl CompressedFile {
    fn compress(in_vec: &Vec<u8>) -> CompressedFile {
        todo!("Compression not currently implemented")
    }

    fn decompress(&self) -> Vec<u8> {
        let comp_buf = &self.data;
        //Pre-allocate the decompression buffer to avoid having to resize a vec during the process
        let mut decomp_buf: Vec<u8> = vec![0x00; self.header.uncompressed_size as usize];

        let mut comp_pos: usize = 0;
        let mut decomp_pos: usize = 0;

        //TODO: Hopefully in the long run, this will be replaced with some declarative niceness
        while comp_pos < comp_buf.len() {
            let control = &comp_buf[comp_pos];
            println!("[cmp: {}, dcmp: {}] control: {:#X}", comp_pos, decomp_pos, control);

            // Rough explanation of compression scheme:
            // Operates somewhat like LZSS; encoded control bytes are used to determine how to
            // backreference repetitions in the data.
            // However, instead of embedding these control bytes in the stream they prefix blocks of
            // plaintext that get copied to the output, and contain info about how to backref
            // Control flow roughly looks like this:
            // 1. Read a control byte, determine range
            //      control bytes are prefixed with a "header" of sorts that causes their type to
            //      fall into ranges that determine how to read them.
            // 2. Read how much plaintext follows the control, and or offset and length of backref
            // 3. Copy plaintext following control character to output
            // 4. Copy backref and length from within output to output

            //First we have to actually get the needed numbers out of the control
            let control_result: (Option<usize>, Option<(usize, usize)>) = match control {
                // Key for description:
                // Length: Length of the control in bytes
                // Plain Text Range: Possible range of values of plain text
                // Plain Text Magic: Magic number added to the number of plain text to copy
                // Copy Range: Possible range of values of number to copy
                // Copy Magic: Magic number added to the number to copy
                // Offset Range: Possible range of offsets
                // Offset Magic: Magic number added to the offset
                // Layout: Bit layout of the control bytes

                // Key for layout:
                // 0 or 1: header
                // F: oFfset (F to not be confused with 0)
                // N: Number to Copy
                // P: Plaintext
                // -: Nibble Separator
                // --: Byte Separator

                // Numbers are always "smashed" together into as small of a space as possible
                // EX: Getting the offset from "0FFN-NNPP--FFFF-FFFF"
                // 1. mask first byte: (byte0 & 0b0110_0000) = 0FF0-0000
                // 2. shift left by 3: (0FF0-0000 << 3) = 0000-00FF--0000-0000
                // 3. OR with second:  (0000-00FF--0000-0000 | 0000-0000--FFFF-FFFF) = 0000-00FF--FFFF-FFFF
                // Another way to do this would be to first shift right by 5 and so on

                // Length: 2
                // Plain Text Range: 0-3
                // Plain Text Magic: 0
                // Copy Range: 3-11
                // Copy Magic: +3
                // Offset Range: 1-1023
                // Offset Magic: +1
                // Layout: 0FFN-NNPP|FFFF-FFFF
                0x00..=0x7F => {
                    let control_slice = &comp_buf[comp_pos..comp_pos + 2];
                    println!("Full control slice: {:X?}", control_slice);
                    comp_pos += 2;
                    let num_plain_text: usize = (control_slice[0] & 0b0000_0011) as usize;

                    let offset = (((((control_slice[0] & 0b0110_0000) as u16) << 3) | (control_slice[1] as u16)) + 1) as usize;
                    let number_copy_offset = (((control_slice[0] & 0b0001_1100) >> 2) + 3) as usize;

                    (Some(num_plain_text), Some((offset, number_copy_offset)))
                }
                // Length: 3
                // Plain Text Range: 0-3
                // Plain Text Magic: 0
                // Copy Range: 4-67
                // Copy Magic: +4
                // Offset Range: 1-16383
                // Offset Magic: +1
                // Layout: 10NN-NNNN|PPFF-FFFF|FFFF-FFFF
                0x80..=0xBF => {
                    let control_slice = &comp_buf[comp_pos..comp_pos + 3];
                    println!("Full control slice: {:X?}", control_slice);
                    comp_pos += 3;

                    let num_plain_text: usize = ((control_slice[1] & 0b1100_0000) >> 6) as usize;

                    let offset: usize = (((((control_slice[1] & 0b0011_1111) as u16) << 8) | (control_slice[2] as u16)) + 1) as usize;
                    let num_to_copy: usize = ((control_slice[0] & 0b0011_1111) + 4) as usize;

                    (Some(num_plain_text), Some((offset, num_to_copy)))
                }
                // Length: 4
                // Plain Text Range: 0-3
                // Plain Text Magic: 0
                // Copy Range: 5-1028
                // Copy Magic: +5
                // Offset Range: 1-131072
                // Offset Magic: +1
                // Layout: 110F-NNPP|FFFF-FFFF|FFFF-FFFF|NNNN-NNNN
                0xC0..=0xDF => {
                    let control_slice = &comp_buf[comp_pos..comp_pos + 4];
                    println!("Full control slice: {:02X?}", control_slice);
                    comp_pos += 4;

                    let num_plain_text: usize = (control_slice[0] & 0b0000_0011) as usize;

                    let offset: usize = ((((control_slice[0] & 0b0001_0000) as u32) << 12) | ((control_slice[1] as u32) << 8) | (control_slice[2] as u32) + 1) as usize;
                    let num_to_copy: usize = ((((control_slice[0] & 0b0000_1100) as u16) << 6) | (control_slice[3] as u16) + 5) as usize;

                    (Some(num_plain_text), Some((offset, num_to_copy)))
                }
                // Length: 1
                // Plain Text Range: 4-128; limited precision
                // Plain Text Magic: +4
                // Copy Range: 0
                // Copy Magic: 0
                // Offset Range: 0
                // Offset Magic: 0
                // Layout: 111P-PPPP
                // Notes: Magic bit shift happens here for unclear reasons, effectively multiplying
                //        stored number by 4
                0xE0..=0xFB => {
                    let control = comp_buf[comp_pos];
                    comp_pos += 1;
                    //It just is bit shifted, don't ask me why
                    let number_plain_text = (((control & 0b0001_1111) << 2) + 4) as usize;
                    (Some(number_plain_text), None)
                }
                // Length: 1
                // Plain Text Range: 0-3
                // Plain Text Magic: 0
                // Copy Range: 0
                // Copy Magic: 0
                // Offset Range: 0
                // Offset Magic: 0
                // Layout: 1111-11PP
                0xFC..=0xFF => {
                    let control = comp_buf[comp_pos];
                    comp_pos += 1;
                    let number_plain_text = (control & 0b0000_0011) as usize;
                    (Some(number_plain_text), None)
                }
            };

            if let Some(plaintext_copy) = control_result.0 {
                println!("Copying plaintext of length: {}", plaintext_copy);
                let src_slice = &comp_buf[comp_pos..comp_pos + plaintext_copy];
                decomp_buf[decomp_pos..decomp_pos + plaintext_copy].copy_from_slice(src_slice);
                comp_pos += plaintext_copy;
                decomp_pos += plaintext_copy;
            }

            if let Some((offset, length)) = control_result.1 {
                println!("Copying length {} to {}", length, offset);
                let src_pos = decomp_pos - offset;
                println!("decomp index: {}", decomp_pos);
                println!("src pos: {}", src_pos);

                // If the sections do not overlap, we can do ultra fast memory section copy
                if (src_pos + length) < decomp_pos {
                    copy_within_slice(&mut decomp_buf, src_pos, decomp_pos, length);
                }
                // If they do, slower character by character copy is required
                else {
                    for i in 0..length {
                        // These will get optimized out, and enable easier debugging
                        let target = decomp_pos + i;
                        let source = src_pos + i;
                        decomp_buf[target] = decomp_buf[source];
                    }
                }

                decomp_pos += length;

            }
            println!("{:X?}", decomp_buf);
        }

        decomp_buf
    }
}

impl TryFrom<&Vec<u8>> for CompressedFile {
    type Error = &'static str;

    fn try_from(in_vec: &Vec<u8>) -> Result<Self, Self::Error> {
        if &in_vec[4..=5] == MAGIC_QFS_ID.to_be_bytes() {
            let mut reader = Cursor::new(in_vec);
            let val = CompressedFile::read(&mut reader).unwrap();
            Ok(val)
        } else {
            return Err("Data is not compressed; use \"CompressedFile::compress\"");
        }

    }

}

mod tests {
    use crate::compression::{CompressedFile};



    #[test]
    fn decompression_yields_correct_data() {
        let data: Vec<u8> = vec![0x30, 0x00, 0x00, 0x00, 0x10, 0xFB, 0x00, 0x01, 0x40, 0xE3, 0x53, 0x65, 0x6D, 0x69, 0x2D, 0x67, 0x6C, 0x6F, 0x62, 0x61, 0x6C, 0x20, 0x66, 0x69, 0x6C, 0x65, 0xAB, 0x40, 0x00, 0x00, 0xE1, 0x0D, 0x50, 0x65, 0x72, 0x73, 0x6F, 0x6E, 0x47, 0x08, 0x41, 0xC2, 0x00, 0x00, 0xEC, 0x73, 0xA3, 0xFC];
        let compressed = CompressedFile::try_from(&data).unwrap();
        let decompressed = compressed.decompress();
        let expected: Vec<u8> = vec![ 0x53, 0x65, 0x6D, 0x69, 0x2D, 0x67, 0x6C, 0x6F, 0x62, 0x61, 0x6C, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0D, 0x50, 0x65, 0x72, 0x73, 0x6F, 0x6E, 0x47, 0x6C, 0x6F, 0x62, 0x61, 0x6C, 0x73, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3 ];
        assert_eq!(decompressed, expected)
    }

    #[test]
    fn compression_is_symmetrical() {
        let test_vec = b"This is a test vector with a repeated phrase! This is a test vector with a repeated phrase!".to_vec();

        let compressed = CompressedFile::compress(&test_vec);
        let decompressed = compressed.decompress();

        assert_eq!(test_vec, decompressed)

    }
}
