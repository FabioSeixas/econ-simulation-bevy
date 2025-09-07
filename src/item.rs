#[derive(Debug)]
pub struct Item {
    pub id: u8,
    // quality: u8,
    // durability: u8,
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum ItemEnum {
    MONEY,
    MEAT,
    WATER
}
