use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn open_file() -> io::Result<Vec<[[i32; 9]; 9]>> {
    //Path of the sudoku.txt file
    let path = Path::new("C:\\Users\\t92\\Documents\\sudoku.txt");
    //puzzles will hold all 50 of the puzzles loaded from the text file
    let mut puzzles: Vec<[[i32; 9]; 9]> = Vec::with_capacity(50);
    
    if path.exists(){
        //If the path exists ^ -> open and read the file
        let file = File::open(&path)?;
        let reader = io::BufReader::new(file);

        //This array is constructed to hold a 2D array of 9x9 integers
        let mut puz: [[i32; 9]; 9] = [[0; 9]; 9];//Vec<[i32; 9]>;
        //Since we're not using a Vec we need to iterate with puz_num
        let mut puz_num = 0;
        
        for line in reader.lines(){
            let line = line?;
            if !line.starts_with("Grid"){
                //Fetch each line, and add each char to the array to create a puzzle with the style of the codewars puzzle
                //vec will hold an array of 9 integers
                let mut vec: [i32; 9] = [0; 9];
                let mut num = 0;
                for c in line.chars() {
                    match c.to_digit(10) {
                        //Add char to the array
                        Some(x) => vec[num] = x as i32,
                        //If the char read is not a digit
                        None => println!("{} is not a digit.", c),
                    }
                    num += 1;
                }
                //Add the line to the puz array
                puz[puz_num] = vec;
                //Increment the placement of the next line in the array
                puz_num += 1;
            }else{ //If the line does start with "Grid"
                //This happens when each puzzle is loaded, so we push it to the array holding all 50 puzzles
                puzzles.push(puz);
                //We wipe the iterator of which line we add to the currently loading puzzle, so its overwritten on the next passing
                puz_num = 0;
            }
            
        }
        //println!("{:?}", puzzles[1]);
    }else{
        println!("File does not exist.");
    }

    //Return all the puzzles to the main() function
    Ok(puzzles)
}

fn possible_remove_hor_ver(i: usize, j: usize, puzzle: [[i32; 9]; 9]) -> Vec<i32>{
    let mut possible = vec![1,2,3,4,5,6,7,8,9];
    for c in 0..9 {
        // Check horizontal:
        possible.retain(|value| *value != puzzle[i][c]);
        // Check vertical:
        possible.retain(|value| *value != puzzle[c][j]);
    }
    possible
}

fn possible_remove_grid(i: usize, j:usize, mut possible: Vec<i32>, puzzle: [[i32; 9]; 9]) -> Vec<i32> {
    let grid_x = i / 3 * 3;
    let grid_y = j / 3 * 3;
    for x in grid_x..grid_x+3 {
        for y in grid_y..grid_y+3 {
            possible.retain(|value| *value != puzzle[x][y]);
        }
    }
    possible
}

//This should return an integer if it's the only possible solution in the grid. possible.clear() possible.push("4")
fn possible_single_placement(i: usize, j:usize, mut possible: Vec<i32>, mut puzzle: [[i32; 9]; 9]) -> i32 {//Vec<Vec<i32>> {
    //let mut grid_options: [[i32; 9]; 9] = [[0; 9]; 9];
    let mut grid_options: Vec<Vec<i32>> = Vec::with_capacity(9);
    let grid_x = i / 3 * 3;
    let grid_y = j / 3 * 3;
    let mut num = 0;
    for x in grid_x..grid_x+3 {
        for y in grid_y..grid_y+3 {
            if !(x == i && y == j) {
                if puzzle[x][y] == 0 {

                let mut possibility = possible_remove_hor_ver(x, y, puzzle);
                possibility = possible_remove_grid(x, y, possibility, puzzle);
                //grid_options.push(possibility);
                //println!("{:?}", possibility);
                grid_options.push(possibility);
            }
            }
        }
    }
    //println!("{:?}", grid_options);
    let mut value_not_found = true;

    for value in &possible {
        let found_in_any_row = grid_options.iter().any(|row|row.contains(value));

        if !found_in_any_row {
            println!("Value {} is not found in the 2D array. Position: {:?}x{:?}", value, i, j);
            //puzzle[i][j] = *value;
            return *value;
        }
    }

    if value_not_found {
        //println!("All values from `possible` are present in the 2D array.");
        println!("{:?}x{:?} searching: {:?} with grid_options: {:?}", i, j, possible, grid_options);
    }

    //grid_options
    return 0;
}

fn new_process_puzzle(mut puzzle: [[i32; 9]; 9]) -> [[i32; 9]; 9] {
    loop {
        let mut updated = false;
        for i in 0..9 { // Horizontal
            for j in 0..9 { // Vertical
                if puzzle[i][j] == 0 {
                    //Remove horizontal and vertical numbers
                    let mut possible = possible_remove_hor_ver(i, j, puzzle);
                    //Remove grid numbers
                    possible = possible_remove_grid(i, j, possible, puzzle);

                    if possible.len() == 1 {
                        puzzle[i][j] = possible[0];
                        println!("{:?}x{:?} filled with {:?}", i, j, possible[0]);
                        updated = true;
                    }else{
                        let single_placement = possible_single_placement(i, j, possible, puzzle);
                        if single_placement != 0 {
                            puzzle[i][j] = single_placement;
                            println!("{:?}x{:?} filled with single placement {:?}", i, j, single_placement);
                            updated = true;
                        }
                    }
                    
                }
                
            }
        }
        if !updated {
            break;
        }
    }

    //Still need to add a way to check if the number 2 can just be placed on a spesific row, when there are two or more options
    puzzle
}

fn process_puzzle(mut puzzle: [[i32; 9]; 9]) -> [[i32; 9]; 9] {
    loop {
        let mut updated = false;
        for i in 0..9 { // Horizontal
            for j in 0..9 { // Vertical
                if puzzle[i][j] == 0 {
                    let mut _pos = vec![1,2,3,4,5,6,7,8,9];
                    //Grid 3x3
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
                    //If there is only one option -> swap the 0 with the only option
                    if _pos.len() == 1 {
                        puzzle[i][j] = _pos[0];
                        updated = true;
                    }
                    //Search through each grid - if this spot is the only one who can contain on specific int, place it!
                    let mut _map_grid_pos: Vec<[i32; 9]> = Vec::with_capacity(9);
                    let grid_x = i / 3 * 3;
                    let grid_y = j / 3 * 3;
                    for x in grid_x..grid_x+3 {
                        for y in grid_y..grid_y+3 {
                            //_pos.retain(|value| *value != puzzle[x][y]);
                            //Map all possible values that can be placed in this puzzle[x][y] and add it to an array
                            
                        }
                    }
                    //If there is a value that is in _pos and does not exist within the array _map_grid_pos, place it!
                }
                

            }
        }
        //If nothing has changed within one loop, break. This can be that it is complete or failed
        if !updated {
            break;
        }
    }
    puzzle
}

fn main() {
    //Open text file with 50 puzzles
    match open_file() {
        //If the file is read -> Pass it to solving one at the time
        Ok(puzzles) => {
            //let solved_puzzle = process_puzzle(puzzles[1]);
            let solved_puzzle = new_process_puzzle(puzzles[2]);
            for row in &solved_puzzle {
                println!("{:?}", row);
            }
        },
        //On error TODO!
        Err(_) => todo!()
    }
    //Default Puzzle from Codewars
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

    //Solve the puzzle from Codewars
    /*
    let solved_puzzle = process_puzzle(puzzle);
    for row in &solved_puzzle {
        //println!("{:?}", row);
    }
    */
}
