use rand::Rng;
use std::io;

const BOARD_SIZE: usize = 5;

#[derive(Debug, Clone, PartialEq, Eq)]
enum BoardPiece {
    Empty,
    Hero,
    Wumpus,
    Bats,
    Arrow,
    Hole,
}
fn main() {
    let mut board = init_board();
    let (mut hero_row, mut hero_col) = random_row_col();
    place_piece(&mut board, BoardPiece::Hero, hero_row, hero_col);
    println!("Hero at row {}, column {}", hero_row, hero_col);

    //TODO arrow pot chafar a un altre
    let (arrow_row, arrow_col) = random_row_col();
    place_piece(&mut board, BoardPiece::Arrow, arrow_row, arrow_col);

    let (mut wumpus_row, mut wumpus_col) = random_row_col();
    while hero_col == wumpus_col && hero_row == wumpus_row {
        print!("randomiza wumpus en la mateixa posicio");
        let (row, col) = random_row_col();
        wumpus_row = row;
        wumpus_col = col;
    }
    place_piece(&mut board, BoardPiece::Wumpus, wumpus_row, wumpus_col);

    let (mut bat_row, mut bat_col) = random_row_col();
    while (hero_col == bat_col && hero_row == bat_row)
        || (wumpus_col == bat_col && wumpus_row == bat_row)
    {
        let (row, col) = random_row_col();
        bat_row = row;
        bat_col = col;
    }
    place_piece(&mut board, BoardPiece::Bats, bat_row, bat_col);

    let (mut hole_row, mut hole_col) = random_row_col();
    while (hero_col == hole_col && hero_row == hole_row)
        || (wumpus_col == hole_col && wumpus_row == hole_row)
        || (bat_col == hole_col && bat_row == hole_row)
    {
        let (row, col) = random_row_col();
        hole_row = row;
        hole_col = col;
    }
    place_piece(&mut board, BoardPiece::Hole, hole_row, hole_col);

    print_board(&board);

    check_surroundings(hero_row, hero_col, &board);

    let mut hero_arrows = 3;
    loop {
        println!("Move or shoot right, left, down, or up.");
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
                    place_piece(&mut board, BoardPiece::Empty, hero_row, hero_col);
                    hero_col = hero_col + 1;
                    place_piece(&mut board, BoardPiece::Hero, hero_row, hero_col);
                    print_board(&board);
                } else {
                    println!("You reached the end of the cave");
                }
                check_surroundings(hero_row, hero_col, &board);
                println!("hero is at row {} col {} ", hero_row, hero_col);
            }
            ["move", "left"] => {
                if hero_col != 0 {
                    place_piece(&mut board, BoardPiece::Empty, hero_row, hero_col);
                    hero_col = hero_col - 1;
                    place_piece(&mut board, BoardPiece::Hero, hero_row, hero_col);
                    print_board(&board);
                } else {
                    println!("You reached the end of the cave");
                }
                check_surroundings(hero_row, hero_col, &board);
                println!("hero is at row {} col {} ", hero_row, hero_col);
            }
            ["move", "down"] => {
                if hero_row != 4 {
                    place_piece(&mut board, BoardPiece::Empty, hero_row, hero_col);
                    hero_row = hero_row + 1;
                    place_piece(&mut board, BoardPiece::Hero, hero_row, hero_col);
                    print_board(&board);
                } else {
                    println!("You reached the end of the cave");
                }
                check_surroundings(hero_row, hero_col, &board);
                println!("hero is at row {} col {} ", hero_row, hero_col);
            }
            ["move", "up"] => {
                if hero_row != 0 {
                    place_piece(&mut board, BoardPiece::Empty, hero_row, hero_col);
                    hero_row = hero_row - 1;
                    place_piece(&mut board, BoardPiece::Hero, hero_row, hero_col);
                    print_board(&board);
                } else {
                    println!("You reached the end of the cave");
                }
                check_surroundings(hero_row, hero_col, &board);
                println!("hero is at row {} col {} ", hero_row, hero_col);
            }
            ["shoot", "right"] => {
                if hero_arrows == 0 {
                    println!(
                        "{} arrows left!, you might find one in the caves",
                        hero_arrows
                    );
                    continue;
                }
                hero_arrows -= 1;
                println!("{} arrows left!", hero_arrows);

                let is_wumpus = board
                    .get(hero_row)
                    .and_then(|r| r.get(hero_col + 1))
                    .cloned();
                match is_wumpus {
                    Some(BoardPiece::Wumpus) => {
                        println!("You hunted the wumpus");
                        break;
                    }
                    Some(data) => {
                        println!("{:?}", data);
                    }
                    _ => {
                        println!("MAL");
                    }
                }
            }
            ["shoot", "left"] => {
                if hero_arrows == 0 {
                    println!(
                        "{} arrows left!, you might find one in the caves",
                        hero_arrows
                    );
                    continue;
                }
                hero_arrows -= 1;
                println!("{} arrows left!", hero_arrows);
                let is_wumpus = board
                    .get(hero_row)
                    .and_then(|r| r.get(hero_col - 1))
                    .cloned();
                match is_wumpus {
                    Some(BoardPiece::Wumpus) => {
                        println!("You hunted the wumpus");
                        break;
                    }
                    Some(data) => {
                        println!("{:?}", data);
                    }
                    _ => {
                        println!("MAL");
                    }
                }
            }
            ["shoot", "down"] => {
                if hero_arrows == 0 {
                    println!(
                        "{} arrows left!, you might find one in the caves",
                        hero_arrows
                    );
                    continue;
                }
                hero_arrows -= 1;
                println!("{} arrows left!", hero_arrows);
                let is_wumpus = board
                    .get(hero_row + 1)
                    .and_then(|r| r.get(hero_col))
                    .cloned();
                match is_wumpus {
                    Some(BoardPiece::Wumpus) => {
                        println!("You hunted the wumpus");
                        break;
                    }
                    Some(data) => {
                        println!("{:?}", data);
                    }

                    _ => {
                        println!("MAL");
                    }
                }
            }

            ["shoot", "up"] => {
                if hero_arrows == 0 {
                    println!(
                        "{} arrows left!, you might find one in the caves",
                        hero_arrows
                    );
                    continue;
                }
                hero_arrows -= 1;
                println!("{} arrows left!", hero_arrows);
                let is_wumpus = board
                    .get(hero_row - 1)
                    .and_then(|r| r.get(hero_col))
                    .cloned();
                match is_wumpus {
                    Some(BoardPiece::Wumpus) => {
                        println!("You hunted the wumpus");
                        break;
                    }
                    Some(data) => {
                        println!("{:?}", data);
                    }
                    _ => {
                        println!("MAL");
                    }
                }
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
fn place_piece(board: &mut Vec<Vec<BoardPiece>>, piece: BoardPiece, row: usize, col: usize) {
    board[row][col] = piece;
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
    println!();
    println!("adjacents {:?}", adjacents);

    for adjacent in adjacents {
        match adjacent {
            Some(BoardPiece::Wumpus) => println!("You smell Wumpus"),
            Some(BoardPiece::Hole) => println!("You feel a breeze"),
            Some(BoardPiece::Bats) => println!("You hear a flapping"),
            _ => (),
        }
    }
}
