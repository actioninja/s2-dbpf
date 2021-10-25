////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

use std::io::{Read, Write, Seek};
use std::ops::Deref;
use binrw::*;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct U24(pub u32);

impl BinRead for U24 {
    type Args = ();

    fn read_options<R: Read + Seek>(reader: &mut R, _options: &ReadOptions, _args: Self::Args) -> BinResult<Self> {
        let mut buf: [u8; 3] = [0; 3];
        reader.read_exact(&mut buf)?;

        Ok(U24(
            (buf[0] as u32) << 16 |
            (buf[1] as u32) << 8 |
            (buf[2] as u32)
        ))
    }
}

impl BinWrite for U24 {
    type Args = ();

    fn write_options<W: Write + Seek>(&self, writer: &mut W, _options: &WriteOptions, _args: Self::Args) -> BinResult<()> {
        let value = self.0;
        writer.write_all(&[
            (value >> 16) as u8,
            (value >> 8) as u8,
            value as u8,
        ])?;

        Ok(())
    }
}




impl Deref for U24 {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn copy_within_slice<T: Copy>(v: &mut [T], from: usize, to: usize, len: usize) {
    if from > to {
        let (dst, src) = v.split_at_mut(from);
        dst[to..to + len].copy_from_slice(&src[..len]);
    } else {

        let (src, dst) = v.split_at_mut(to);
        dst[..len].copy_from_slice(&src[from..from + len]);
    }
}