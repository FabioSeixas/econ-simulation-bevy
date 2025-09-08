#[derive(Debug)]
pub struct Item {
    pub id: ItemEnum
    // quality: u8,
    // durability: u8,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum ItemEnum {
    MONEY,
    MEAT,
    WATER
}
