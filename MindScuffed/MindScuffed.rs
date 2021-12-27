//MindScuffed Interpreter

use std::io::Read;
use rand::prelude::*;

const CELL_MAX: i32 = 255;
const CELL_SIZE: usize = 15;

fn printcells(instructions: [i32; CELL_SIZE]) {
    for x in instructions.iter() {
        print!("| {} ", x);
    }
    println!("|");
}

fn main() {


    let path = String::from("D:\\Languages\\MindScuff\\src\\MindScuff.ms");
    let instructions = std::fs::read_to_string(path).unwrap();

    let mut cells:[i32; CELL_SIZE] = [0; CELL_SIZE];
    let mut current: usize = 0;


    let mut x: usize = 0;
    while x < instructions.len() {
        let mut chr = instructions.as_bytes()[x] as char;
        if chr == '+' { cells[current] += 1; if cells[current] > CELL_MAX { cells[current] = CELL_MAX; } }
        if chr == '-' { cells[current] -= 1; if cells[current] < 0 { cells[current] = 0; } }
        if chr == ':' { cells[current] = CELL_MAX; }
        if chr == ';' { cells[current] = 0; }
        if chr == '>' { if current >= CELL_SIZE-1 { current = 0; } else { current += 1; }}
        if chr == '<' { if current <= 0 { current = CELL_SIZE-1; } else { current -= 1; }}
        if chr == '.' { println!("{}", cells[current]); }
        if chr == ',' {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            cells[current] = input.trim().parse::<i32>().unwrap();
            if cells[current] > CELL_MAX { cells[current] = CELL_MAX; }
            else if cells[current] < 0 { cells[current] = 0; }
        } // input
        if chr == '[' { if cells[current] == 0 {
            while chr != ']' {
                x+=1;
                chr = instructions.as_bytes()[x] as char;

            }
            x+=1;}
        }
        if chr == ']' { if cells[current] != 0 {
            while chr != '[' {
                x-=1;
                chr = instructions.as_bytes()[x] as char;
            }
            x-=1; }
        }
        if chr == '*' { cells[current] *= 2; }
        if chr == '/' { cells[current] /= 2; }
        if chr == '&' { cells[current] += cells[current+1]; cells[current+1] = 0; }
        if chr == '~' { cells[current] = rand::thread_rng().gen_range(0..CELL_MAX); }
        if chr == '\\' { cells[current] -= cells[current+1]; cells[current+1] = 0; }
        if chr == 'l' { printcells(cells); }
        x+=1;
    }
}
