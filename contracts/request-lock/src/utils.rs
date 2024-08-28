use ckb_std::{
    ckb_constants::Source,
    high_level::{load_cell_lock_hash, load_cell_type_hash, QueryIter},
};

pub fn check_owner_lock_32_bytes(owner_lock_hash: &[u8]) -> bool {
    QueryIter::new(load_cell_lock_hash, Source::Input)
        .any(|cell_lock_hash| owner_lock_hash[..] == cell_lock_hash[0..32])
}

pub fn check_owner_type_32_bytes(owner_input_type_hash: &[u8], source: Source) -> bool {
    QueryIter::new(load_cell_type_hash, source).any(|cell_type_hash| {
        if let Some(cell_type_hash) = cell_type_hash {
            owner_input_type_hash[..] == cell_type_hash[0..32]
        } else {
            false
        }
    })
}
