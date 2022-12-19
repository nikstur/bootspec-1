use std::fmt;
use std::convert::TryInto;

use std::error::Error;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

pub mod generation;
pub mod v1;

#[doc(hidden)]
pub type Result<T, E = Box<dyn Error + Send + Sync + 'static>> = core::result::Result<T, E>;

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
/// A wrapper type describing the name of a NixOS specialisation.
pub struct SpecialisationName(pub String);

impl fmt::Display for SpecialisationName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq)]
/// A wrapper type describing the root directory of a NixOS system configuration.
pub struct SystemConfigurationRoot(pub PathBuf);

// !!! IMPORTANT: KEEP `BootJson`, `SCHEMA_VERSION`, and `JSON_FILENAME` IN SYNC !!!
/// The current bootspec schema.
pub type BootJson<Extension> = v1::GenerationV1<Extension>;
/// The current bootspec schema version.
pub const SCHEMA_VERSION: u64 = v1::SCHEMA_VERSION;
/// The current bootspec schema filename.
pub const JSON_FILENAME: &str = v1::JSON_FILENAME;

// Enable conversions from Generation into the current Bootspec schema.
impl<Extension: Default + fmt::Debug> TryInto<BootJson<Extension>> for generation::Generation<Extension> {
    type Error = Box<dyn Error + Send + Sync + 'static>;

    // Rust(-Analyzer) do not guess that Generation is not an exhaustive enum.
    #[allow(unreachable_patterns)]
    fn try_into(self) -> Result<BootJson<Extension>> {
        match self {
            generation::Generation::V1(v1) => Ok(v1),
            _ => Err(format!("Unsupported Bootspec generation: {:?}", self).into())
        }
    }
}
