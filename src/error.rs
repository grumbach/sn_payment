// Copyright 2021 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under The General Public License (GPL), version 3.
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. Please review the Licences for the specific language governing
// permissions and limitations relating to use of the SAFE Network Software.
use thiserror::Error;

/// Specialisation of `std::Result`.
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[allow(clippy::large_enum_variant)]
#[derive(Error, Debug)]
// #[non_exhaustive]
/// error variants.
pub enum Error {
    #[error("memo too long")]
    MemoTooLong,

    #[error("insufficient payment")]
    InsufficientPayment,

    /// dbc error
    #[error("dbc error: {0}")]
    Dbc(#[from] sn_dbc::Error),
    // /// serialisation error.
    // #[error("serialisation error: {0}")]
    // Serialisation(#[from] serde_json::Error),

    // /// serialisation error.
    // #[error("serialisation error: {0}")]
    // Deserialisation(#[from] serde_json::de::Error),

    // #[error("Infallible.  Can never fail")]
    // Infallible(#[from] std::convert::Infallible),
}
