use crate::error::Error;

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
