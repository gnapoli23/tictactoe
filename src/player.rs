#[derive(Clone, PartialEq)]
pub enum PlayerSymbol {
    Usd,
    Btc,
}

impl std::fmt::Display for PlayerSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            PlayerSymbol::Btc => write!(
                f,
                "{}",
                std::str::from_utf8(&[0xE2u8, 0x82u8, 0xBFu8]).unwrap()
            ),
            PlayerSymbol::Usd => write!(f, "{}", std::str::from_utf8(&[0x24u8]).unwrap()),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Player {
    pub id: u8,
    pub symbol: PlayerSymbol,
}

impl Player {
    pub fn new(symbol: PlayerSymbol) -> Self {
        Self {
            id: rand::random(),
            symbol,
        }
    }

    pub fn get_symbol(&self) -> &PlayerSymbol {
        &self.symbol
    }
}
