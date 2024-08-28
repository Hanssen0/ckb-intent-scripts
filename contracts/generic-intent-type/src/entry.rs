use alloc::vec::Vec;
use ckb_std::{
    ckb_constants::Source,
    ckb_types::{
        packed,
        prelude::{Entity, Unpack},
    },
    high_level::{load_script, load_witness_args},
};

use crate::{
    error::Error,
    generated,
    intent::{verify_intent_data, ScriptLocation},
    utils::{check_owner_lock_32_bytes, check_owner_type_32_bytes, has_cell},
};

pub fn main() -> Result<(), Error> {
    let script = load_script()?;
    let args = script.args().raw_data();
    if args.len() < 32 {
        return Err(Error::Encoding);
    }

    let intent_data_hash = &args[0..32];

    if has_cell(0, Source::GroupInput) {
        check_input(intent_data_hash)?;
    }

    if has_cell(0, Source::GroupOutput) {
        check_output(intent_data_hash)?;
    }

    Ok(())
}

pub fn check_input(expected_intent_data_hash: &[u8]) -> Result<(), Error> {
    let witness_args = load_witness_args(0, Source::GroupInput)?;
    let data: Vec<u8> = match witness_args.input_type().to_opt() {
        Some(data) => data.unpack(),
        None => todo!(),
    };

    let intent_data_hash = ckb_hash::blake2b_256(data);
    if intent_data_hash != expected_intent_data_hash {
        return Err(Error::IntentDataUnmatched);
    }

    Ok(())
}

pub fn check_output(expected_intent_data_hash: &[u8]) -> Result<(), Error> {
    let witness_args = load_witness_args(0, Source::GroupInput)?;
    let data: Vec<u8> = match witness_args.input_type().to_opt() {
        Some(data) => data.unpack(),
        None => todo!(),
    };

    if ckb_hash::blake2b_256(&data) != expected_intent_data_hash {
        return Err(Error::IntentDataUnmatched);
    }

    let intent_data = generated::intent::IntentData::from_slice(&data)?;
    verify_intent_data(intent_data.as_reader())?;

    if intent_data.creators().into_iter().all(|signer| {
        let signer_script_hash: packed::Byte32 = signer.script_hash();
        let signer_script_location: ScriptLocation =
            match Into::<u8>::into(signer.location()).try_into() {
                Ok(loc) => loc,
                Err(_) => return false,
            };

        let is_signer_exist = match signer_script_location {
            ScriptLocation::InputLock => check_owner_lock_32_bytes(signer_script_hash.as_slice()),
            ScriptLocation::InputType => {
                check_owner_type_32_bytes(signer_script_hash.as_slice(), Source::Input)
            }
            ScriptLocation::OutputType => {
                check_owner_type_32_bytes(signer_script_hash.as_slice(), Source::Output)
            }
        };

        is_signer_exist
    }) {
        return Ok(());
    }

    Err(Error::CheckFailed)
}
