////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

use std::fmt::{Display, Formatter};
use std::io::{Read, Seek, Write};
use std::option::Option::Some;
use std::str;

use binrw::io::Cursor;
use binrw::{binrw, BinRead, BinResult, BinWrite, ReadOptions, WriteOptions};
use debug_print::debug_println as dprintln;
use proptest::char::range;
#[cfg(test)]
use proptest::prelude::*;
#[cfg(test)]
use test_strategy::Arbitrary;

use crate::helpers::{copy_within_slice, U24};

const MAGIC_QFS_ID: u16 = 0x10FB;

#[binrw]
#[brw(little)]
#[derive(PartialOrd, PartialEq, Debug)]
struct CompressionHeader {
    compressed_size: u32,
    #[brw(big, magic = 0x10FB_u16)]
    #[br(map = | x: U24 | * x)]
    #[bw(map = | x: & u32 | U24(* x))]
    uncompressed_size: u32,
}

#[binrw]
#[derive(PartialEq, Debug)]
pub struct CompressedFile {
    header: CompressionHeader,
    //compressed_size includes the length of the header, and the header is always 9 bytes long
    #[br(count = (header.compressed_size - 9))]
    data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CompressionError {
    AlreadyCompressed,
    TooShort(usize),
    CompressionFailed(String),
}

impl Display for CompressionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CompressionError::AlreadyCompressed => write!(f, "Already compressed"),
            CompressionError::TooShort(size) => {
                write!(f, "File is too short; minimum 6, got {}", size)
            }
            CompressionError::CompressionFailed(reason) => {
                write!(f, "Compression failed: {}", reason)
            }
        }
    }
}

// TODO: This implementation is fairly gnarly. Ideally this could be replaced with a declarative
// approach some time in the future.
#[derive(Debug, Copy, Clone, PartialEq)]
struct Control {
    range: ControlRange,
    offset_copy: Option<(usize, usize)>,
    plaintext_copy: Option<usize>,
}

#[cfg(test)]
prop_compose! {
    fn bhav_instruction_mapper()(
        range in any::<ControlRange>(),
        small_string_copy in (0..=3_usize),
        large_string_copy in (4..=7_usize),
        first_offset in (1..=1023_usize),
        first_length in (3..=11_usize),
        second_offset in (1..=16383_usize),
        second_length in (4..=67_usize),
        third_offset in (1..=13_1072_usize),
        third_length in (5..=1028_usize),
    ) -> Control {
        let offset_copy: Option<(usize, usize)> = match range {
            ControlRange::First => Some((first_offset, first_length)),
            ControlRange::Second => Some((second_offset, second_length)),
            ControlRange::Third => Some((third_offset, third_length)),
            ControlRange::Fourth
            | ControlRange::Fifth => None,
        };

        let plaintext_copy: Option<usize> = match range {
                ControlRange::First
                | ControlRange::Second
                | ControlRange::Third
                | ControlRange::Fourth => {
                    if small_string_copy > 0 {
                        Some(small_string_copy)
                    } else {
                        None
                    }
                }
                ControlRange::Fifth => {
                    if large_string_copy > 0 {
                        Some(large_string_copy)
                    } else {
                        None
                    }
                }
            };
        Control {
            range,
            offset_copy,
            plaintext_copy,
        }
    }
}

#[cfg(test)]
impl Arbitrary for Control {
    type Parameters = ();

    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        todo!()
    }

    type Strategy = BoxedStrategy<Self>;
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(test, derive(Arbitrary))]
enum ControlRange {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
}

impl Control {
    fn new(offset_copy: Option<(usize, usize)>, plaintext_copy: Option<usize>) -> Self {
        if let Some(plaintext_copy) = plaintext_copy {
            if plaintext_copy > 128 {
                panic!(
                    "plaintext_copy is too large (maximum 128, got {})",
                    plaintext_copy
                );
            } else if plaintext_copy > 3 && offset_copy.is_some() {
                panic!(
                    "Cannot have plaintext_copy higher than 3 with an offset_copy (got {})",
                    plaintext_copy
                );
            }
        }

        if let Some((offset, length)) = offset_copy {
            if offset > 13_1072 {
                panic!("offset is too large (maximum 128, got {})", offset);
            } else if length > 1028 {
                panic!("length is too large (maximum 128, got {})", length);
            }
        }

        let range: ControlRange = if let Some((offset, length)) = offset_copy {
            let plaintext_copy = plaintext_copy.unwrap_or(0);

            if plaintext_copy > 3 {
                panic!(
                    "Cannot have plaintext_copy higher than 3 with an offset_copy (got {})",
                    plaintext_copy
                );
            }

            if offset > 13_1072 || length > 1028 {
                panic!("Invalid offset or number to copy (Maximum offset 131072, got {}) (maximum num to copy 1028, got {})", offset, length);
            } else if offset > 16383 || length > 67 {
                ControlRange::Third
            } else if offset > 1023 || length > 10 {
                ControlRange::Second
            } else {
                ControlRange::First
            }
        } else if let Some(plaintext_copy) = plaintext_copy {
            if plaintext_copy > 128 {
                panic!(
                    "plaintext_copy is too large (maximum 128, got {})",
                    plaintext_copy
                );
            }

            if plaintext_copy > 3 {
                ControlRange::Fourth
            } else {
                ControlRange::Fifth
            }
        } else {
            panic!("Malformed input?")
        };

        Self {
            range,
            offset_copy,
            plaintext_copy,
        }
    }
}

impl BinRead for Control {
    type Args = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        _options: &ReadOptions,
        _args: Self::Args,
    ) -> BinResult<Self> {
        let byte1 = u8::read(reader)?;
        match byte1 {
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
                let byte1: usize = usize::from(byte1);
                let byte2: usize = u8::read(reader)?.into();

                let num_plain_text = byte2 & 0b0000_0011;
                let offset = (((byte1 & 0b0110_0000) << 3) | byte2) + 1;
                let number_copy_offset = ((byte1 & 0b0001_1100) >> 2) + 3;

                Ok(Control {
                    range: ControlRange::First,
                    offset_copy: Some((offset, number_copy_offset)),
                    plaintext_copy: Some(num_plain_text),
                })
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
                let byte1: usize = usize::from(byte1);
                let byte2: usize = u8::read(reader)?.into();
                let byte3: usize = u8::read(reader)?.into();

                let num_plain_text = (byte2 & 0b1100_0000) >> 6;

                let offset = (((byte2 & 0b0011_1111) << 8) | byte3) + 1;

                let num_to_copy = (byte1 & 0b0011_1111) + 4;

                Ok(Control {
                    range: ControlRange::Second,
                    offset_copy: Some((offset, num_to_copy)),
                    plaintext_copy: Some(num_plain_text),
                })
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
                let byte1: usize = usize::from(byte1);
                let byte2: usize = u8::read(reader)?.into();
                let byte3: usize = u8::read(reader)?.into();
                let byte4: usize = u8::read(reader)?.into();

                let num_plain_text = byte1 & 0b0000_0011;

                let offset = (((byte1 & 0b0001_0000) << 12) | (byte2 << 8) | byte3) + 1;

                let num_to_copy = (((byte1 & 0b0000_1100) << 6) | byte4) + 5;

                Ok(Control {
                    range: ControlRange::Third,
                    offset_copy: Some((offset, num_to_copy)),
                    plaintext_copy: Some(num_plain_text),
                })
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
            //        stored number by 4, decreasing precision in the process
            0xE0..=0xFB => {
                let byte1: usize = usize::from(byte1);

                let number_plain_text = ((byte1 & 0b0001_1111) << 2) + 4;

                Ok(Control {
                    range: ControlRange::Fourth,
                    offset_copy: None,
                    plaintext_copy: Some(number_plain_text),
                })
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
                let byte1: usize = usize::from(byte1);

                let number_plain_text = byte1 & 0b0000_0011;

                Ok(Control {
                    range: ControlRange::Fifth,
                    offset_copy: None,
                    plaintext_copy: Some(number_plain_text),
                })
            }
        }
    }
}

impl BinWrite for Control {
    type Args = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        options: &WriteOptions,
        _args: Self::Args,
    ) -> BinResult<()> {
        // Nothing to write
        if self.offset_copy.is_none() && self.plaintext_copy.is_none() {
            // Maybe make this an error?
            return Ok(());
        }

        let plaintext_copy = self.plaintext_copy.unwrap_or(0);
        let (offset, length) = self.offset_copy.unwrap_or((0, 0));
        return match self.range {
            ControlRange::First => {
                let magic_adjust_offset = offset - 1;
                let magic_adjust_length = length - 3;

                let byte1 = (((magic_adjust_offset & 0b0000_0011_0000_0000) >> 3)
                    | (magic_adjust_length << 2)
                    | (plaintext_copy & 0b0000_0011)) as u8;

                let byte2 = (magic_adjust_offset & 0b1111_1111) as u8;

                byte1.write_options(writer, options, ())?;
                byte2.write_options(writer, options, ())?;
                Ok(())
            }
            ControlRange::Second => {
                let magic_adjust_offset = offset - 1;
                let magic_adjust_length = length - 5;

                let byte1 = (0b1000_0000 | (magic_adjust_length & 0b0011_1111)) as u8;

                let byte2 = ((plaintext_copy << 6)
                    | ((magic_adjust_offset & 0b0011_1111_0000_0000) >> 8))
                    as u8;

                let byte3 = (magic_adjust_offset & 0b1111_1111) as u8;

                byte1.write_options(writer, options, ())?;
                byte2.write_options(writer, options, ())?;
                byte3.write_options(writer, options, ())?;
                Ok(())
            }
            ControlRange::Third => {
                let magic_adjust_offset = offset - 1;
                let magic_adjust_length = length - 5;

                let byte1 = (0b1100_0000
                    | ((magic_adjust_offset >> 12) & 0b0001_0000)
                    | ((magic_adjust_length >> 6) & 0b0000_1100)
                    | (plaintext_copy & 0b0000_0011)) as u8;

                let byte2 = ((magic_adjust_offset >> 8) & 0b1111_1111) as u8;

                let byte3 = (magic_adjust_offset & 0b1111_1111) as u8;

                let byte4 = (magic_adjust_length & 0b1111_1111) as u8;

                byte1.write_options(writer, options, ())?;
                byte2.write_options(writer, options, ())?;
                byte3.write_options(writer, options, ())?;
                byte4.write_options(writer, options, ())?;
                Ok(())
            }
            ControlRange::Fourth => {
                let byte = (0b1110_0000 | ((plaintext_copy - 4) >> 2)) as u8;
                byte.write_options(writer, options, ())?;
                Ok(())
            }
            ControlRange::Fifth => {
                let byte = (0b1111_1100 | plaintext_copy) as u8;
                byte.write_options(writer, options, ())?;
                Ok(())
            }
        };
    }
}

impl CompressedFile {
    pub fn compress(in_vec: &[u8]) -> Result<CompressedFile, CompressionError> {
        const MAX_OFFSET: usize = 0x0002_0000;
        const MAX_COPY: usize = 0x0404;
        const QFS_MAXITER: usize = 0x80;

        if in_vec.len() < 6 {
            return Err(CompressionError::TooShort(in_vec.len()));
        }

        if in_vec[4..=5] == MAGIC_QFS_ID.to_be_bytes() {
            return Err(CompressionError::AlreadyCompressed);
        }

        let uncompressed_len = in_vec.len();

        let mut compressed = Vec::with_capacity(uncompressed_len);

        Ok(CompressedFile {
            header: CompressionHeader {
                compressed_size: compressed.len() as u32,
                uncompressed_size: uncompressed_len as u32,
            },
            data: compressed,
        })
    }

    /// Decompresses the data in the `CompressedFile`, yielding a Vec<u8>
    /// This shouldn't be able to fail, data is already checked to be compressed
    #[must_use]
    pub fn decompress(&self) -> Vec<u8> {
        let comp_buf = &self.data;
        //Pre-allocate the decompression buffer to avoid having to resize a vec during the process
        let mut decomp_buf: Vec<u8> = vec![0x00; self.header.uncompressed_size as usize];

        let mut comp_pos: usize = 0;
        let mut decomp_pos: usize = 0;

        //TODO: Hopefully in the long run, this will be replaced with some declarative niceness
        while comp_pos < comp_buf.len() {
            let control = &comp_buf[comp_pos];
            dprintln!(
                "[cmp: {}, dcmp: {}] control: {:#X}",
                comp_pos,
                decomp_pos,
                control
            );

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
                // Copy Range: 3-10
                // Copy Magic: +3
                // Offset Range: 1-1023
                // Offset Magic: +1
                // Layout: 0FFN-NNPP|FFFF-FFFF
                0x00..=0x7F => {
                    let control_slice = &comp_buf[comp_pos..comp_pos + 2];
                    dprintln!("Full control slice: {:X?}", control_slice);
                    comp_pos += 2;
                    let num_plain_text: usize = (control_slice[0] & 0b0000_0011) as usize;

                    let offset = (((u16::from(control_slice[0] & 0b0110_0000) << 3)
                        | u16::from(control_slice[1]))
                        + 1) as usize;
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
                    dprintln!("Full control slice: {:X?}", control_slice);
                    comp_pos += 3;

                    let num_plain_text: usize = ((control_slice[1] & 0b1100_0000) >> 6) as usize;

                    let offset: usize = (((u16::from(control_slice[1] & 0b0011_1111) << 8)
                        | u16::from(control_slice[2]))
                        + 1) as usize;
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
                    dprintln!("Full control slice: {:02X?}", control_slice);
                    comp_pos += 4;

                    let num_plain_text: usize = (control_slice[0] & 0b0000_0011) as usize;

                    let offset: usize = ((u32::from(control_slice[0] & 0b0001_0000) << 12)
                        | (u32::from(control_slice[1]) << 8)
                        | (u32::from(control_slice[2]) + 1))
                        as usize;
                    let num_to_copy: usize = ((u16::from(control_slice[0] & 0b0000_1100) << 6)
                        | (u16::from(control_slice[3]) + 5))
                        as usize;

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
                dprintln!("Copying plaintext of length: {}", plaintext_copy);
                let src_slice = &comp_buf[comp_pos..comp_pos + plaintext_copy];
                decomp_buf[decomp_pos..decomp_pos + plaintext_copy].copy_from_slice(src_slice);
                comp_pos += plaintext_copy;
                decomp_pos += plaintext_copy;
            }

            if let Some((offset, length)) = control_result.1 {
                dprintln!("Copying length {} to {}", length, offset);
                let src_pos = decomp_pos - offset;
                dprintln!("decomp index: {}", decomp_pos);
                dprintln!("src pos: {}", src_pos);

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
            dprintln!("{:X?}", decomp_buf);
        }

        decomp_buf
    }
}

impl TryFrom<&Vec<u8>> for CompressedFile {
    type Error = &'static str;

    /// Rapid already compressed data to `CompressedData` conversion.
    ///
    /// # Errors
    /// returns `Err("Data is not compressed; use \"CompressedFile::compress\")`
    /// if the data isn't compressed, slower `compress()` call is needed
    fn try_from(in_vec: &Vec<u8>) -> Result<Self, Self::Error> {
        if in_vec[4..=5] == MAGIC_QFS_ID.to_be_bytes() {
            let mut reader = Cursor::new(in_vec);
            let val = CompressedFile::read(&mut reader).unwrap();
            Ok(val)
        } else {
            Err("Data is not compressed; use \"CompressedFile::compress\"")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helpers::test_parsing;

    use super::*;

    #[test]
    fn decompression_yields_correct_data() {
        let data: Vec<u8> = vec![
            0x30, 0x00, 0x00, 0x00, 0x10, 0xFB, 0x00, 0x01, 0x40, 0xE3, 0x53, 0x65, 0x6D, 0x69,
            0x2D, 0x67, 0x6C, 0x6F, 0x62, 0x61, 0x6C, 0x20, 0x66, 0x69, 0x6C, 0x65, 0xAB, 0x40,
            0x00, 0x00, 0xE1, 0x0D, 0x50, 0x65, 0x72, 0x73, 0x6F, 0x6E, 0x47, 0x08, 0x41, 0xC2,
            0x00, 0x00, 0xEC, 0x73, 0xA3, 0xFC,
        ];
        let compressed = CompressedFile::try_from(&data).unwrap();
        let decompressed = compressed.decompress();
        let expected: Vec<u8> = vec![
            0x53, 0x65, 0x6D, 0x69, 0x2D, 0x67, 0x6C, 0x6F, 0x62, 0x61, 0x6C, 0x20, 0x66, 0x69,
            0x6C, 0x65, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0D, 0x50, 0x65, 0x72, 0x73, 0x6F,
            0x6E, 0x47, 0x6C, 0x6F, 0x62, 0x61, 0x6C, 0x73, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3,
            0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3,
            0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3,
            0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3,
            0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3,
            0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3,
            0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3,
            0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3,
            0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3,
            0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3,
            0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3,
            0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3,
            0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3,
            0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3,
            0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3,
            0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3,
            0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3,
            0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3, 0xA3,
        ];
        assert_eq!(decompressed, expected);
    }

    test_parsing!([], Control {}, Control, control);

    //#[test]
    fn compression_is_symmetrical() {
        let test_vec = b"This is a test vector with a repeated phrase! This is a test vector with a repeated phrase!".to_vec();

        let compressed = CompressedFile::compress(&test_vec);
        let decompressed = compressed.unwrap().decompress();

        assert_eq!(test_vec, decompressed);
    }
}
