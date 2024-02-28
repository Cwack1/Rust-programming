use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn open_file() -> io::Result<Vec<[[i32; 9]; 9]>> {
    let path = Path::new("C:\\Users\\t92\\Documents\\sudoku.txt");
    let mut puzzles: Vec<[[i32; 9]; 9]> = Vec::with_capacity(50);
    if path.exists(){
        let file = File::open(&path)?;
        let reader = io::BufReader::new(file);

        let mut puz: [[i32; 9]; 9] = [[0; 9]; 9];//Vec<[i32; 9]>;
        let mut puz_num = 0;
        for line in reader.lines(){
            let line = line?;
            if !line.starts_with("Grid"){
                //Add lines to the board
                //println!("{}", line);
                //Convert line to array and push to puz
                let mut vec: [i32; 9] = [0; 9];
                let mut num = 0;
                for c in line.chars() {
                    match c.to_digit(10) {
                        Some(x) => vec[num] = x as i32,
                        None => println!("{} is not a digit.", c),
                    }
                    num += 1;
                }
                //println!("{:?}", vec);
                //puz.push(vec);
                //println!("{:?}", puz_num);
                puz[puz_num] = vec;
                puz_num += 1;
            }else{
                //println!("{}", line);
                //Set up a new board
                puzzles.push(puz);
                //puz.clear();
                //Trenger puz å bli cleared? Den overskrives uansett
                puz_num = 0;
            }
            
        }
        //println!("{:?}", puzzles[1]);
    }else{
        println!("File does not exist.");
    }


    Ok(puzzles)
}

fn process_puzzle(mut puzzle: [[i32; 9]; 9]) -> [[i32; 9]; 9] {
    loop {
        let mut updated = false;
        for i in 0..9 { // Horizontal
            for j in 0..9 { // Vertical
                if puzzle[i][j] == 0 {
                    let mut _pos = vec![1,2,3,4,5,6,7,8,9];
                    let grid_x = i / 3 * 3;
                    let grid_y = j / 3 * 3;
                    for x in grid_x..grid_x+3 {
                        for y in grid_y..grid_y+3 {
                            _pos.retain(|value| *value != puzzle[x][y]);
                        }
                    }
                    for c in 0..9 {
                        // Check horizontal:
                        _pos.retain(|value| *value != puzzle[i][c]);
                        // Check vertical:
                        _pos.retain(|value| *value != puzzle[c][j]);
                    }
                    if _pos.len() == 1 {
                        puzzle[i][j] = _pos[0];
                        updated = true;
                    }
                }
            }
        }
        if !updated {
            break;
        }
    }
    puzzle
}

fn main() {
    //let _content = open_file();
    match open_file() {
        Ok(puzzles) => {
            let solved_puzzle = process_puzzle(puzzles[4]);
            for row in &solved_puzzle {
                println!("{:?}", row);
            }
        },
        Err(_) => todo!()
    }
    let _puzzle = [
        [5,3,0,0,7,0,0,0,0],
        [6,0,0,1,9,5,0,0,0],
        [0,9,8,0,0,0,0,6,0],
        [8,0,0,0,6,0,0,0,3],
        [4,0,0,8,0,3,0,0,1],
        [7,0,0,0,2,0,0,0,6],
        [0,6,0,0,0,0,2,8,0],
        [0,0,0,4,1,9,0,0,5],
        [0,0,0,0,8,0,0,7,9]
    ];
    /*
    let solved_puzzle = process_puzzle(puzzle);
    for row in &solved_puzzle {
        //println!("{:?}", row);
    }
    */
}
