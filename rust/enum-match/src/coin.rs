#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    pub fn value_in_cents(self) -> u8 {
        match self {
            Coin::Penny   => 1,
            Coin::Nickel  => 5,
            Coin::Dime    => 10,
            Coin::Quarter => 25,
        }
    }
}


