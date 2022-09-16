////////////////////////////////////////////////////////////////////////////////
// This Source Code Form is subject to the terms of the Mozilla Public         /
// License, v. 2.0. If a copy of the MPL was not distributed with this         /
// file, You can obtain one at https://mozilla.org/MPL/2.0/.                   /
////////////////////////////////////////////////////////////////////////////////

use binrw::error::Error as BinError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("File is not in Sims 2 DBPF Format")]
    NotSims2Format,
    #[error("Failed to read file")]
    BinRWError(#[from] BinError),
}

pub type DbpfResult<T> = Result<T, Error>;
