use super::player;

pub struct Cell {
    pub x_pos: u8,
    pub y_pos: u8,
    pub player: Option<player::Player>,
}

impl Cell {
    pub fn new(x_pos: u8, y_pos: u8, player: Option<player::Player>) -> Self {
        Self {
            x_pos,
            y_pos,
            player,
        }
    }

    pub fn show(&self) -> String {
        match &self.player {
            Some(p) => p.get_symbol().to_string(),
            None => String::from("-"),
        }
    }
}

pub struct Board {
    pub cells: Vec<Cell>,
    pub players: (player::Player, player::Player),
}

impl Board {
    pub fn new() -> Self {
        let cells = {
            let mut cells = Vec::new();
            for i in 0..4 {
                for j in 0..4 {
                    let cell = Cell::new(i as u8, j as u8, Option::None);
                    cells.push(cell);
                }
            }
            cells
        };
        let players = {
            (
                player::Player::new(player::PlayerSymbol::Btc),
                player::Player::new(player::PlayerSymbol::Usd),
            )
        };
        Self { cells, players }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let show = format!(
            r#"
            ...................
            :  {}  :  {}  :  {}  :
            ...................
            :  {}  :  {}  :  {}  :
            ...................
            :  {}  :  {}  :  {}  :
            ...................
            "#,
            self.cells[0].show(),
            self.cells[1].show(),
            self.cells[2].show(),
            self.cells[3].show(),
            self.cells[4].show(),
            self.cells[5].show(),
            self.cells[6].show(),
            self.cells[7].show(),
            self.cells[8].show(),
        );
        write!(f, "{}", &show)
    }
}

#[derive(Debug)]
pub enum WinningCombinations {
    BottomHorizontal,
    CenterHorizontal,
    TopHorizontal,
    LeftVertical,
    CenterVertical,
    RightVertical,
    LeftToRightDiagonal,
    RightToLeftDiagonal,
}

impl WinningCombinations {
    pub fn get(&self) -> (usize, usize, usize) {
        match &self {
            WinningCombinations::BottomHorizontal => (6, 7, 8),
            WinningCombinations::CenterHorizontal => (3, 4, 5),
            WinningCombinations::TopHorizontal => (0, 1, 2),
            WinningCombinations::LeftVertical => (0, 3, 6),
            WinningCombinations::CenterVertical => (1, 4, 7),
            WinningCombinations::RightVertical => (2, 5, 8),
            WinningCombinations::LeftToRightDiagonal => (6, 4, 2),
            WinningCombinations::RightToLeftDiagonal => (8, 4, 0),
        }
    }

    pub fn get_all() -> Vec<WinningCombinations> {
        vec![
            WinningCombinations::BottomHorizontal,
            WinningCombinations::CenterHorizontal,
            WinningCombinations::TopHorizontal,
            WinningCombinations::LeftVertical,
            WinningCombinations::CenterVertical,
            WinningCombinations::RightVertical,
            WinningCombinations::LeftToRightDiagonal,
            WinningCombinations::RightToLeftDiagonal,
        ]
    }
}
