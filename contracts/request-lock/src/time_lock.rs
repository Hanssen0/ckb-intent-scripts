use ckb_std::{
    ckb_constants::Source,
    high_level::{load_input_since, QueryIter},
    since::Since,
};

// check since,
// for all inputs the since field must have the exactly same flags with the since
// constraint, and the value of since must greater or equals than the since
// contstaint
pub fn check_since(since: u64) -> bool {
    let since = Since::new(since);

    QueryIter::new(load_input_since, Source::GroupInput).all(|input_since| {
        let input_since = Since::new(input_since);
        input_since >= since
    })
}
