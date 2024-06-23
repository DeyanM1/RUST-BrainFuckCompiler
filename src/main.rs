#![allow(non_snake_case)]

use std::env;
use std::io::{self, Write};
use std::time::Instant;
use std::fs;



fn readFile(filename: &str) -> Vec<char> {
    let mut code: Vec<char> = vec![];
    let commands = "<>+-[],.";
    let content = fs::read_to_string(filename)
        .expect("Failed to read file!");

    for char in content.chars() {
        if commands.contains(char) {
            code.push(char);
        } else {
            continue;
        }
    }

    code
}


fn compiler(code: Vec<char>) -> (String, Vec<i32>) {
    //let code_com: Vec<char> = vec![];
    let mut cells = vec![0];
    let mut output= String::new();

    let mut pointer = 0;
    let mut current_code_place = 1;


    //code.push('%');

    loop {
        if code[current_code_place -1] == '+' {
            if cells[pointer] == 255 {
                cells[pointer] = 0;
            } else {
                cells[pointer] += 1;
            };
        } else if code[current_code_place -1] == '-' {
            if cells[pointer] == 0 {
                cells[pointer] = 255;
            } else {
                cells[pointer] -= 1;
            };
        } else if code[current_code_place -1] == '>' {
            if cells.len()-1 == pointer {
                cells.push(0); 
                pointer += 1;
            } else { 
                pointer += 1;
            };
        } else if code[current_code_place -1] == '<' {
            if pointer == 0 {
                println!("Memory error: -1");
            } else { 
                pointer -= 1;
            };
        } else if code[current_code_place -1] == '.' {

            output.push(cells[pointer] as u8 as char);

        } else if code[current_code_place -1] == ',' {
            print!("Enter char -> ");
            io::stdout().flush().expect("Failed to flush stdout");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let input = input.trim();
            if input.chars().count() == 1 {
                let character = input.chars().next().unwrap();
                cells[pointer] = character as i32;
            } else {
                println!("Please enter exactly one character.");
            }

        } else if code[current_code_place -1] == '[' {
            let  starting_pointer = pointer;
            let starting_code_place = current_code_place;

            loop {
                
                if cells[starting_pointer] == 0 {
                    break
                }
                if code[current_code_place -1] == '+' {
                    //println!("Add");
                    if cells[pointer] == 255 {
                        cells[pointer] = 0;
                    } else {
                        cells[pointer] += 1;
                    };
                } else if code[current_code_place -1] == '-' {
                    //println!("Remove");
                    if cells[pointer] == 0 {
                        cells[pointer] = 255;
                    } else {
                        cells[pointer] -= 1;
                    };
                } else if code[current_code_place -1] == '>' {
                    //println!("Move Right");
                    if cells.len()-1 == pointer {
                        cells.push(0); 
                        pointer += 1;
                    } else { 
                        pointer += 1;
                    };
                } else if code[current_code_place -1] == '<' {
                    //println!("Move Left");
                    if pointer == 0 {
                        println!("Memory error: -1");
                    } else { 
                        pointer -= 1;
                    };
                } else if code[current_code_place -1] == '.' {
                    //println!("Print");
                    output.push(cells[pointer] as u8 as char);
        
                } else if code[current_code_place -1] == ',' {
                    print!("Enter char -> ");
                    io::stdout().flush().expect("Failed to flush stdout");
                    let mut input = String::new();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");
                    let input = input.trim();
                    if input.chars().count() == 1 {
                        let character = input.chars().next().unwrap();
                        cells[pointer] = character as i32;
                    } else {
                        println!("Please enter exactly one character.");
                    }
                        
                } else if code[current_code_place -1] == ']' {
                    //println!("Stop loop");
                    current_code_place = starting_code_place;
                    if cells[starting_pointer] == 0 {
                        break
                    }
                    continue
                } 
            current_code_place += 1;
            }
        }



        if current_code_place != code.len() {
            current_code_place += 1;
        } else {
            break;
        } 
    
    }

    (output, cells)
}


fn main() { 
    let args: Vec<String> = env::args().collect();
    let filename: &str = &args[1];
    let now = Instant::now();

    let code = readFile(filename);

    println!("Code: {:?}", &code);

    let (output, cells) = compiler(code);

    let elapsed = now.elapsed();
    println!("Output: {}", output);
    println!("Cells: {:?}", cells);
    println!("Elapsed: {:.2?}", elapsed);
}