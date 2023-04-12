use rand::Rng;
use std::io;
mod text;
mod types;
use text::{get_colors, ARROW_TEXT, WUMPUS_TEXT};
use types::{ArrowTarget, BoardPiece};

const BOARD_SIZE: usize = 5;

fn main() {
    let debug_board = true;
    let mut board = init_board();
    let mut hero_arrows: u8 = 2;

    let (arrow_row, arrow_col) = random_row_col();
    place_piece(
        &mut board,
        BoardPiece::Arrow,
        arrow_row,
        arrow_col,
        &mut hero_arrows,
    )
    .unwrap();

    let (arrow_row, arrow_col) = random_row_col();
    place_piece(
        &mut board,
        BoardPiece::Arrow,
        arrow_row,
        arrow_col,
        &mut hero_arrows,
    )
    .unwrap();

    let (mut hero_row, mut hero_col) = random_row_col();
    place_piece(
        &mut board,
        BoardPiece::Hero,
        hero_row,
        hero_col,
        &mut hero_arrows,
    )
    .unwrap();
    if debug_board == true {
        println!("Hero at row {}, column {}", hero_row, hero_col);
    }

    let (mut wumpus_row, mut wumpus_col) = random_row_col();
    while hero_col == wumpus_col && hero_row == wumpus_row {
        let (row, col) = random_row_col();
        wumpus_row = row;
        wumpus_col = col;
    }
    place_piece(
        &mut board,
        BoardPiece::Wumpus,
        wumpus_row,
        wumpus_col,
        &mut hero_arrows,
    )
    .unwrap();

    let (mut bat_row, mut bat_col) = random_row_col();
    while (hero_col == bat_col && hero_row == bat_row)
        || (wumpus_col == bat_col && wumpus_row == bat_row)
    {
        let (row, col) = random_row_col();
        bat_row = row;
        bat_col = col;
    }
    place_piece(
        &mut board,
        BoardPiece::Bats,
        bat_row,
        bat_col,
        &mut hero_arrows,
    )
    .unwrap();

    let (mut hole_row, mut hole_col) = random_row_col();
    while (hero_col == hole_col && hero_row == hole_row)
        || (wumpus_col == hole_col && wumpus_row == hole_row)
        || (bat_col == hole_col && bat_row == hole_row)
    {
        let (row, col) = random_row_col();
        hole_row = row;
        hole_col = col;
    }
    place_piece(
        &mut board,
        BoardPiece::Hole,
        hole_row,
        hole_col,
        &mut hero_arrows,
    )
    .unwrap();
    if debug_board == true {
        print_board(&board);
    }

    check_surroundings(hero_row, hero_col, &board);
    println!("Move or shoot right, left, down, or up.");
    loop {
        let mut action = String::new();
        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read line");

        match action
            .trim()
            .to_lowercase()
            .split(" ")
            .collect::<Vec<&str>>()
            .as_slice()
        {
            ["q"] => {
                break;
            }
            ["move", "right"] => {
                if hero_col != 4 {
                    place_piece(
                        &mut board,
                        BoardPiece::Empty,
                        hero_row,
                        hero_col,
                        &mut hero_arrows,
                    )
                    .unwrap();
                    hero_col = hero_col + 1;
                    let res = place_piece(
                        &mut board,
                        BoardPiece::Hero,
                        hero_row,
                        hero_col,
                        &mut hero_arrows,
                    );
                    match res {
                        Ok(_) => {}
                        Err(err) => {
                            println!("{}", err);
                            break;
                        }
                    }

                    if debug_board == true {
                        print_board(&board);
                    }
                } else {
                    println!("You reached the end of the cave");
                }
                check_surroundings(hero_row, hero_col, &board);
            }
            ["move", "left"] => {
                if hero_col != 0 {
                    place_piece(
                        &mut board,
                        BoardPiece::Empty,
                        hero_row,
                        hero_col,
                        &mut hero_arrows,
                    )
                    .unwrap();
                    hero_col = hero_col - 1;
                    let res = place_piece(
                        &mut board,
                        BoardPiece::Hero,
                        hero_row,
                        hero_col,
                        &mut hero_arrows,
                    );
                    match res {
                        Ok(_) => {}
                        Err(err) => {
                            println!("{}", err);
                            break;
                        }
                    }

                    if debug_board == true {
                        print_board(&board);
                    }
                } else {
                    println!("You reached the end of the cave");
                }
                check_surroundings(hero_row, hero_col, &board);
            }
            ["move", "down"] => {
                if hero_row != 4 {
                    place_piece(
                        &mut board,
                        BoardPiece::Empty,
                        hero_row,
                        hero_col,
                        &mut hero_arrows,
                    )
                    .unwrap();
                    hero_row = hero_row + 1;
                    let res = place_piece(
                        &mut board,
                        BoardPiece::Hero,
                        hero_row,
                        hero_col,
                        &mut hero_arrows,
                    );
                    match res {
                        Ok(_) => {}
                        Err(err) => {
                            println!("{}", err);
                            break;
                        }
                    }

                    if debug_board == true {
                        print_board(&board);
                    }
                } else {
                    println!("You reached the end of the cave");
                }
                check_surroundings(hero_row, hero_col, &board);
            }
            ["move", "up"] => {
                if hero_row != 0 {
                    place_piece(
                        &mut board,
                        BoardPiece::Empty,
                        hero_row,
                        hero_col,
                        &mut hero_arrows,
                    )
                    .unwrap();
                    hero_row = hero_row - 1;
                    let res = place_piece(
                        &mut board,
                        BoardPiece::Hero,
                        hero_row,
                        hero_col,
                        &mut hero_arrows,
                    );
                    match res {
                        Ok(_) => {}
                        Err(err) => {
                            println!("{}", err);
                            break;
                        }
                    }
                    if debug_board == true {
                        print_board(&board);
                    }
                } else {
                    println!("You reached the end of the cave");
                }
                check_surroundings(hero_row, hero_col, &board);
            }
            ["shoot", "right"] => {
                let arrow_col = hero_col + 1;
                let arrow_row = hero_row;
                let target = shoot_bow(&mut hero_arrows, arrow_row, arrow_col, &mut board);

                match target {
                    ArrowTarget::Wumpus => break,
                    ArrowTarget::Bats => {
                        place_piece(
                            &mut board,
                            BoardPiece::Empty,
                            arrow_row,
                            arrow_col,
                            &mut hero_arrows,
                        )
                        .unwrap();
                    }
                    ArrowTarget::None => continue,
                };
            }
            ["shoot", "left"] => {
                let arrow_col = if hero_col > 0 { hero_col - 1 } else { 0 };
                let arrow_row = hero_row;
                let target = shoot_bow(&mut hero_arrows, arrow_row, arrow_col, &mut board);

                match target {
                    ArrowTarget::Wumpus => break,
                    ArrowTarget::Bats => {
                        place_piece(
                            &mut board,
                            BoardPiece::Empty,
                            arrow_row,
                            arrow_col,
                            &mut hero_arrows,
                        )
                        .unwrap();
                    }
                    ArrowTarget::None => continue,
                };
            }
            ["shoot", "down"] => {
                let arrow_col = hero_col;
                let arrow_row = hero_row + 1;
                let target = shoot_bow(&mut hero_arrows, arrow_row, arrow_col, &mut board);

                match target {
                    ArrowTarget::Wumpus => break,
                    ArrowTarget::Bats => {
                        place_piece(
                            &mut board,
                            BoardPiece::Empty,
                            arrow_row,
                            arrow_col,
                            &mut hero_arrows,
                        )
                        .unwrap();
                    }
                    ArrowTarget::None => continue,
                };
            }

            ["shoot", "up"] => {
                let arrow_col = hero_col;
                let arrow_row = if hero_row > 0 { hero_row - 1 } else { 0 };
                let target = shoot_bow(&mut hero_arrows, arrow_row, arrow_col, &mut board);
                match target {
                    ArrowTarget::Wumpus => break,
                    ArrowTarget::Bats => {
                        place_piece(
                            &mut board,
                            BoardPiece::Empty,
                            arrow_row,
                            arrow_col,
                            &mut hero_arrows,
                        )
                        .unwrap();
                    }
                    ArrowTarget::None => continue,
                };
            }
            _ => {
                println!("Invalid input. Please enter 'move' or 'shoot' followed by 'right', 'left', 'down', or 'up'");
            }
        }
    }
}

fn random_row_col() -> (usize, usize) {
    let mut rng = rand::thread_rng();
    (rng.gen_range(0..BOARD_SIZE), rng.gen_range(0..BOARD_SIZE))
}

fn init_board() -> Vec<Vec<BoardPiece>> {
    vec![vec![BoardPiece::Empty; BOARD_SIZE]; BOARD_SIZE]
}
fn place_piece(
    board: &mut Vec<Vec<BoardPiece>>,
    piece: BoardPiece,
    row: usize,
    col: usize,
    hero_arrows: &mut u8,
) -> Result<(), String> {
    let colors = get_colors();
    let red = colors.get("red").unwrap();
    let reset_color = colors.get("reset").unwrap();
    let piece_in_current_position = &mut board[row][col];
    if piece == BoardPiece::Hero {
        match piece_in_current_position {
            BoardPiece::Arrow => {
                let random_text_index = rand::thread_rng().gen_range(0..=ARROW_TEXT.len() - 1);
                let arrow_text = ARROW_TEXT.get(random_text_index);
                println!("{}", arrow_text.unwrap());
                *hero_arrows += 1;
                println!("You have {} arrows!", hero_arrows);
            }
            BoardPiece::Wumpus => {
                let random_text_index = rand::thread_rng().gen_range(0..=WUMPUS_TEXT.len() - 1);
                let wumpus_text = WUMPUS_TEXT.get(random_text_index).unwrap();
                let mut error_text = String::new();
                error_text.push_str(red);
                error_text.push_str(wumpus_text);
                error_text.push_str(reset_color);
                return Err(error_text);
            }
            BoardPiece::Bats => {
                return Err("You're swarmed by vampiric bats, their fangs piercing your skin as they drain your blood and leave you weak and lifeless.".to_owned());
            }
            BoardPiece::Hole => {
                return Err("You slip and fall into an endless chasm, the darkness and emptiness swallowing you whole.".to_owned());
            }
            _ => {}
        }
    }
    *piece_in_current_position = piece;
    Ok(())
}
fn print_board(board: &Vec<Vec<BoardPiece>>) {
    println!();
    for row in board {
        let row_str = row
            .iter()
            .map(|piece| format!("{:#?} ", piece))
            .collect::<String>();
        println!("{}", row_str);
    }
    println!();
}
fn check_surroundings(hero_row: usize, hero_col: usize, board: &Vec<Vec<BoardPiece>>) {
    let colors = get_colors();
    let adjacents = vec![
        board
            .get(hero_row)
            .and_then(|r| r.get(hero_col + 1))
            .cloned(),
        board
            .get(hero_row)
            .and_then(|r| r.get(hero_col.wrapping_sub(1)))
            .cloned(),
        board
            .get(hero_row.wrapping_sub(1))
            .and_then(|r| r.get(hero_col))
            .cloned(),
        board
            .get(hero_row + 1)
            .and_then(|r| r.get(hero_col))
            .cloned(),
    ];

    for adjacent in adjacents {
        let mut info = String::new();
        match adjacent {
            Some(BoardPiece::Wumpus) => info = String::from("You smell Wumpus"),
            Some(BoardPiece::Hole) => info = String::from("You feel a breeze"),
            Some(BoardPiece::Bats) => info = String::from("You hear a flapping"),
            _ => (),
        }

        if !info.is_empty() {
            println!(
                "{}{}{}",
                colors.get("yellow").unwrap(),
                info,
                colors.get("reset").unwrap()
            );
        }
    }
}
fn shoot_bow(
    hero_arrows: &mut u8,
    target_row: usize,
    target_col: usize,
    board: &mut Vec<Vec<BoardPiece>>,
) -> ArrowTarget {
    let colors = get_colors();
    if *hero_arrows == 0 {
        println!(
            "{}No arrows left! You might find one in the caves.{}",
            colors.get("yellow").unwrap(),
            colors.get("reset").unwrap()
        );
        return ArrowTarget::None;
    }
    *hero_arrows -= 1;
    println!("{} arrows left!", hero_arrows);
    let target = board
        .get_mut(target_row)
        .and_then(|r| r.get_mut(target_col))
        .map(|p| p.clone());
    match target {
        Some(BoardPiece::Wumpus) => {
            println!("{}After a long and grueling hunt, you finally catch up to the wumpus and take it down with a well-aimed shot, its massive body collapsing at your feet.{}",colors.get("green").unwrap(),colors.get("reset").unwrap());
            return ArrowTarget::Wumpus;
        }
        Some(BoardPiece::Bats) => {
            println!(
                "{}You hit a swarm of bats, sending them flying in all directions.{}",
                colors.get("green").unwrap(),
                colors.get("reset").unwrap()
            );
            return ArrowTarget::Bats;
        }
        _ => {
            println!("You release the arrow, but your aim is off and it misses its mark. You hear the sound of it clattering against the rock wall, a painful reminder of your failure.");
            return ArrowTarget::None;
        }
    }
}
