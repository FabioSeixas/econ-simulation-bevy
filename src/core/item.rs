#[derive(Debug)]
pub struct Item {
    pub id: ItemEnum, // quality: u8,
                      // durability: u8,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum ItemEnum {
    MONEY,
    MEAT,
    WATER,
}

impl ItemEnum {
    pub fn is_food(&self) -> bool {
        match self {
            ItemEnum::MEAT => true,
            _ => false,
        }
    }
}
