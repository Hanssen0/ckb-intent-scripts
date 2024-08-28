#[derive(Default, Debug)]
pub struct IntentData {
    pub owner: [u8; 32],
    pub singers: Vec<[u8; 32]>,
    pub targets: Vec<Vec<u8>>,
    pub calldata: Vec<u8>,
}

#[test]
fn test_intent_data() {
    let a = IntentData {
        ..Default::default()
    };

    println!("{:?}", a);

    println!("size: {}", std::mem::size_of::<IntentData>())
}
