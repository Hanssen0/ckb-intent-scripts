use crate::error::Error;
use crate::generated;

use alloc::vec::Vec;

#[derive(Default, Debug)]
pub enum ScriptLocation {
    #[default]
    InputLock,
    InputType,
    OutputType,
}

impl TryFrom<u8> for ScriptLocation {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ScriptLocation::InputLock),
            1 => Ok(ScriptLocation::InputType),
            2 => Ok(ScriptLocation::OutputType),
            _ => Err(Error::Encoding),
        }
    }
}

pub fn verify_intent_data(value: generated::intent::IntentDataReader) -> Result<(), Error> {
    value
        .creators()
        .iter()
        .map(|signer| {
            let _: ScriptLocation = Into::<u8>::into(signer.location()).try_into()?;
            Ok(())
        })
        .collect::<Result<Vec<_>, Error>>()?;

    Ok(())
}
