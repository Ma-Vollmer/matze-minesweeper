#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![new_board])
        .invoke_handler(tauri::generate_handler![mark])
        .invoke_handler(tauri::generate_handler![reveal])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use std::cell::RefCell;
use rand::Rng;

#[derive(Debug)]
struct Cell {
    value: u8,
    marked: bool,
    revealed: bool,
}

impl Copy  for Cell {}
impl Clone for Cell {
    fn clone(&self) -> Cell {
        return Cell { value: self.value, marked: self.marked, revealed: self.revealed };
    }
}
impl Cell {
    fn new() -> Cell {
        return Cell { value: 0, marked: false, revealed: false };
    }
}


//static BOARD: [[Cell; 8]; 8] = [[Cell::new(); 8]; 8];
thread_local!(static BOARD: RefCell<[[Cell; 8]; 8]> = RefCell::new([[Cell::new(); 8]; 8]));


#[tauri::command]
fn new_board() {
    init_board();
    reveal(7, 7);
    print_board();
}

fn init_board () {
    BOARD.with(|b|{
        let mut temp_b = *b.borrow_mut();

        //init the Board
        for cell in temp_b.iter_mut().flat_map(|c| c.iter_mut()) {
            cell.value = 0;
            cell.revealed = false;
            cell.marked = false;
        }
        
        for _i in 0..10 {
            // getting a free & random location for the bomb
            let mut rng = rand::thread_rng();
            let random: u8 = rng.gen_range(0..63);
            let mut x = random%8;
            let mut y = random/8;
            while temp_b[x as usize][y as usize].value > 8 {
                let random: u8 = rng.gen_range(0..63);
                x = random%8;
                y = random/8;
            }

            // placing bomb
            if x==0 && y==0 {
                temp_b[x as usize][y as usize]      .value = 9;
                temp_b[x as usize +1][y as usize +1].value += 1;
                temp_b[x as usize +1][y as usize]   .value += 1;
                temp_b[x as usize]   [y as usize +1].value += 1;
            } else if x==7 && y==7 {
                temp_b[x as usize][y as usize]      .value = 9;
                temp_b[x as usize]   [y as usize -1].value += 1;
                temp_b[x as usize -1][y as usize]   .value += 1;
                temp_b[x as usize -1][y as usize -1].value += 1;
            }   else if x==7 && y==0 {
                temp_b[x as usize][y as usize]      .value = 9;
                temp_b[x as usize]   [y as usize +1].value += 1;
                temp_b[x as usize -1][y as usize +1].value += 1;
                temp_b[x as usize -1][y as usize]   .value += 1;
            }else if x==0 && y==7 {
                temp_b[x as usize][y as usize]      .value = 9;
                temp_b[x as usize +1][y as usize]   .value += 1;
                temp_b[x as usize +1][y as usize -1].value += 1;
                temp_b[x as usize]   [y as usize -1].value += 1;
            }else if x==7 {
                temp_b[x as usize][y as usize]      .value = 9;
                temp_b[x as usize]   [y as usize +1].value += 1;
                temp_b[x as usize]   [y as usize -1].value += 1;
                temp_b[x as usize -1][y as usize +1].value += 1;
                temp_b[x as usize -1][y as usize]   .value += 1;
                temp_b[x as usize -1][y as usize -1].value += 1;
            }else if x==0 {
                temp_b[x as usize][y as usize]      .value = 9;
                temp_b[x as usize +1][y as usize +1].value += 1;
                temp_b[x as usize +1][y as usize]   .value += 1;
                temp_b[x as usize +1][y as usize -1].value += 1;
                temp_b[x as usize]   [y as usize +1].value += 1;
                temp_b[x as usize]   [y as usize -1].value += 1;
            }else if y==7 {
                temp_b[x as usize][y as usize]      .value = 9;
                temp_b[x as usize +1][y as usize]   .value += 1;
                temp_b[x as usize +1][y as usize -1].value += 1;
                temp_b[x as usize]   [y as usize -1].value += 1;
                temp_b[x as usize -1][y as usize]   .value += 1;
                temp_b[x as usize -1][y as usize -1].value += 1;
            }else if y==0 {
                temp_b[x as usize][y as usize]      .value = 9;
                temp_b[x as usize +1][y as usize +1].value += 1;
                temp_b[x as usize +1][y as usize]   .value += 1;
                temp_b[x as usize]   [y as usize +1].value += 1;
                temp_b[x as usize -1][y as usize +1].value += 1;
                temp_b[x as usize -1][y as usize]   .value += 1;
            }else {
                //default case
                temp_b[x as usize][y as usize]      .value = 9;
                temp_b[x as usize +1][y as usize +1].value += 1;
                temp_b[x as usize +1][y as usize]   .value += 1;
                temp_b[x as usize +1][y as usize -1].value += 1;
                temp_b[x as usize]   [y as usize +1].value += 1;
                temp_b[x as usize]   [y as usize -1].value += 1;
                temp_b[x as usize -1][y as usize +1].value += 1;
                temp_b[x as usize -1][y as usize]   .value += 1;
                temp_b[x as usize -1][y as usize -1].value += 1;
            }
        }
        *b.borrow_mut() = temp_b;
    })
}

fn rec_reveal(x: u8, y: u8, mut board: [[Cell; 8]; 8]) -> [[Cell; 8]; 8] {
    board[x as usize][y as usize].revealed = true;
    if board[x as usize][ y as usize].revealed || board[x as usize][ y as usize].value > 0 { return board; }
                if x==0 && y==0 {
                    rec_reveal(x+1, y+1, board);
                    rec_reveal(x+1, y, board);
                    rec_reveal(x, y+1, board);
                    rec_reveal(x-1, y+1, board);
                    rec_reveal(x-1, y, board);
                    rec_reveal(x-1, y-1, board);
                } else if x==7 && y==7 {
                    rec_reveal(x, y-0, board);
                    rec_reveal(x-1, y, board);
                    rec_reveal(x-1, y-1, board);
                } else if x==0 && y==7 {
                    rec_reveal(x+1, y, board);
                    rec_reveal(x+1, y-1, board);
                    rec_reveal(x, y-0, board);
                }else if x==7 && y==0 {
                    rec_reveal(x, y+1, board);
                    rec_reveal(x-1, y+1, board);
                    rec_reveal(x-1, y, board);
                }else if x==7 {
                    rec_reveal(x, y+1, board);
                    rec_reveal(x, y-0, board);
                    rec_reveal(x-1, y+1, board);
                    rec_reveal(x-1, y, board);
                    rec_reveal(x-1, y-1, board);
                }else if x==0 {
                    rec_reveal(x+1, y+1, board);
                    rec_reveal(x+1, y, board);
                    rec_reveal(x+1, y-1, board);
                    rec_reveal(x, y+1, board);
                    rec_reveal(x, y-0, board);
                }else if y==7 {
                    rec_reveal(x+1, y, board);
                    rec_reveal(x+1, y-1, board);
                    rec_reveal(x, y-0, board);
                    rec_reveal(x-1, y, board);
                    rec_reveal(x-1, y-1, board);
                }else if y==0 {
                    rec_reveal(x+1, y+1, board);
                    rec_reveal(x+1, y, board);
                    rec_reveal(x, y+1, board);
                    rec_reveal(x-1, y+1, board);
                    rec_reveal(x-1, y, board);
                }else {
                    //default case
                    rec_reveal(x+1, y+1, board);
                    rec_reveal(x+1, y, board);
                    rec_reveal(x+1, y-1, board);
                    rec_reveal(x, y+1, board);
                    rec_reveal(x, y-0, board);
                    rec_reveal(x-1, y+1, board);
                    rec_reveal(x-1, y, board);
                    rec_reveal(x-1, y-1, board);
                }

    return board;
}

#[tauri::command]
fn reveal (x: u8, y: u8) -> bool{
    println!("it is in revealed!");
    let mut result: bool = true;
    BOARD.with(|b| {
        let mut temp_b = *b.borrow_mut();
        temp_b[x as usize][y as usize].revealed = true;
        if temp_b[x as usize][y as usize].value == 0 { temp_b = rec_reveal(x, y, temp_b) };
        if temp_b[x as usize][y as usize].value >= 9 { result = false };
        *b.borrow_mut() = temp_b;
    });

    return result;
}

#[tauri::command]
fn mark (x: u8, y: u8) -> bool{
    let mut won = true;
    BOARD.with(|b| {
        let mut temp_b = *b.borrow_mut();
        temp_b[x as usize][y as usize].marked = !temp_b[x as usize][y as usize].marked;

        //check if you have won - return true if yes
        for cell in temp_b.iter_mut().flat_map(|c| c.iter_mut()) {
            if cell.marked && cell.value<8 { won = false }
            if !cell.marked && cell.value>8 { won = false }
        }
        *b.borrow_mut() = temp_b;
    });
    return won;
}

fn print_board () {
    BOARD.with(|b| {
        let temp_b = *b.borrow_mut();
        println!("{:#?}", b);
        *b.borrow_mut() = temp_b;
    })
}
