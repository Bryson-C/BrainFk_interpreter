//MindScuffed Interpreter

fn printcells(instructions: [i32; 8]) {
    for x in instructions.iter() {
        print!("| {} ", x);
    }
    println!("|");
}

fn main() {
    const CELL_MAX:i32 = 255;

    let instructions = String::from(">>++++++++.>---.l<&.l;");
    

    let mut cells:[i32; 8] = [0; 8];
    let mut current: usize = 0;

    
    let mut x: usize = 0;
    while x < instructions.len() {
        let mut chr = instructions.as_bytes()[x] as char;
        if chr == '+' { cells[current] += 1; }
        if chr == '-' { cells[current] -= 1; }
        if chr == ':' { cells[current] = CELL_MAX; }
        if chr == ';' { cells[current] = 0; }
        if chr == '>' { if current >= 7 { current = 0; } else { current += 1; }}
        if chr == '<' { if current <= 0 { current = 7; } else { current -= 1; }}
        if chr == '.' { println!("{}", cells[current]); }
        if chr == ',' { } // input
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
        if chr == '~' { } // random
        if chr == '\\' { cells[current] -= cells[current+1]; cells[current+1] = 0; }
        if chr == 'l' { printcells(cells); }
        x+=1;
    }
    printcells(cells);
}
