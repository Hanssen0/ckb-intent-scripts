use ckb_std::{ckb_constants::Source, high_level::load_script};

use crate::{
    error::Error,
    time_lock::check_since,
    types::ScriptLocation,
    utils::{check_owner_lock_32_bytes, check_owner_type_32_bytes},
};

fn check_script_exists(hash: &[u8], location: ScriptLocation) -> bool {
    match location {
        ScriptLocation::InputLock => check_owner_lock_32_bytes(hash),
        ScriptLocation::InputType => check_owner_type_32_bytes(hash, Source::Input),
        ScriptLocation::OutputType => check_owner_type_32_bytes(hash, Source::Output),
    }
}

pub fn main() -> Result<(), Error> {
    let script = load_script()?;
    let args = script.args().raw_data();
    if args.len() < 74 {
        return Err(Error::Encoding);
    }

    let receiver_script_hash = &args[0..32];
    let receiver_script_location: ScriptLocation = args[32].try_into()?;
    let sender_script_hash = &args[33..65];
    let sender_script_location: ScriptLocation = args[65].try_into()?;
    let since = &args[66..74];

    if check_script_exists(receiver_script_hash, receiver_script_location) {
        return Ok(());
    }

    if !check_since(u64::from_le_bytes(since.try_into().unwrap())) {
        return Err(Error::IncorrectSince);
    }

    if check_script_exists(sender_script_hash, sender_script_location) {
        return Ok(());
    }

    Err(Error::CheckFailed)
}
