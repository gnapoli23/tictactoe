use std::io;

mod board;
mod player;

use board::{Board, WinningCombinations};
use player::{Player, PlayerSymbol};

fn main() {
    let pl1 = Player::new(PlayerSymbol::Btc);
    let pl2 = Player::new(PlayerSymbol::Usd);

    println!("Player 1 symbol: {}", pl1.get_symbol());
    println!("Player 2 symbol: {}", pl2.get_symbol());

    let mut board = Board::new();
    println!("New Board: {}", board);

    loop {
        // Turn 1
        loop {
            println!("[Player 1] {} is your turn:", pl1.get_symbol());
            let pl1_input = get_input();
            if pl1_input.is_err() {
                println!("{}", &pl1_input.err().unwrap());
                //refresh
                clearscreen::clear().unwrap();
            } else {
                let v = &pl1_input.ok().unwrap();
                match match_input(v, &mut board, &pl1) {
                    Ok(_) => {
                        //refresh
                        clearscreen::clear().unwrap();
                        println!("{}", board);
                        break;
                    }
                    Err(e) => println!("{}", e),
                }
            }
        }

        // Check game
        let winner = check_game(&board);
        if winner.is_some() {
            println!("Congratulations {}, you win!", winner.unwrap().get_symbol());
            break;
        }

        // Turn 2
        loop {
            println!("[Player 2] {} is your turn:", pl2.get_symbol());
            let pl2_input = get_input();
            if pl2_input.is_err() {
                println!("{}", &pl2_input.err().unwrap());
                //refresh
                clearscreen::clear().unwrap();
            } else {
                let v = &pl2_input.ok().unwrap();
                match match_input(v, &mut board, &pl2) {
                    Ok(_) => {
                        //refresh
                        clearscreen::clear().unwrap();
                        println!("{}", board);
                        break;
                    }
                    Err(e) => println!("{}", e),
                };
            }
        }
    }
}

fn check_game(board: &Board) -> Option<&Player> {
    // find if there's a winning combination: 3 cells related to the same player
    let combinations = WinningCombinations::get_all();
    for c in combinations {
        let coords = c.get();
        let attempt = (
            board.cells.get(coords.0).unwrap(),
            board.cells.get(coords.1).unwrap(),
            board.cells.get(coords.2).unwrap(),
        );

        //is there something?
        //if the cells have a reference to the same player, we have a winner
        //if the cells point to different players, nope
        if attempt.0.player.is_some()
            && attempt.1.player.is_some()
            && attempt.2.player.is_some()
            && (attempt.0.player.as_ref().unwrap() == attempt.1.player.as_ref().unwrap())
            && (attempt.0.player.as_ref().unwrap() == attempt.2.player.as_ref().unwrap())
        {
            return attempt.0.player.as_ref();
        }
    }
    Option::None
}

fn match_input(input: &u8, board: &mut Board, player: &Player) -> Result<(), String> {
    // The cells represent the digit keys on keyboard
    //          7   8   9
    //          4   5   6
    //          1   2   3
    let mut cell = match input {
        1u8 => &mut board.cells[6],
        2u8 => &mut board.cells[7],
        3u8 => &mut board.cells[8],
        4u8 => &mut board.cells[3],
        5u8 => &mut board.cells[4],
        6u8 => &mut board.cells[5],
        7u8 => &mut board.cells[0],
        8u8 => &mut board.cells[1],
        9u8 => &mut board.cells[2],
        _ => todo!(),
    };

    if cell.player.is_none() {
        // make the move
        cell.player = Some(player.to_owned());
        Ok(())
    } else {
        Err(String::from("Select an emtpy cell!"))
    }
}

fn get_input() -> Result<u8, String> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let p = match input.trim().parse::<u8>() {
                Ok(v) => v,
                Err(_) => return Err(String::from("Input must be an integer!")),
            };
            if !(1u8..=9u8).contains(&p) {
                Err(String::from("Number must be in range [1-9]!"))
            } else {
                Ok(p)
            }
        }
        Err(_) => Err(String::from("Wrong input!")),
    }
}
