use std::{env, fs};

const POINTER_RIGHT: usize = 1;
const POINTER_LEFT: usize = 2;
const INCREMENT: usize = 3;
const DECREMENT: usize = 4;
const PRINT_CHAR: usize = 5;
const INPUT_CHAR: usize = 6;
const START_LOOP: usize = 7;
const END_LOOP: usize = 8;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <file>", args[0]);
        return;
    }

    let code = fs::read_to_string(&args[1]).expect("Cannot read file");
    if &args[0] == "jooooosh_transpile" {
        transpile_from_bf(&code);
    } else {
        run_code(&code);
    }
}

fn run_code(code: &str) {
    let mut cells: Vec<u8> = vec![0; 30000];
    let mut ptr = 0;
    let mut pc = 0;
    let mut lines = code.lines();

    lines.find(|line| line.contains("j"))
         .expect("Invalid program - no j found");
    let code = lines.collect::<Vec<&str>>();

    while pc < code.len() {
        let line = code[pc];

        match line.chars().filter(|c| *c == 'o').count() {
            POINTER_RIGHT => {
                ptr += 1;
                if ptr >= cells.len() {
                    cells.push(0);
                }
            }
            POINTER_LEFT => {
                ptr -= 1;
            }
            INCREMENT => {
                cells[ptr] = cells[ptr].checked_add(1).unwrap_or(0);
            }
            DECREMENT => {
                cells[ptr] = cells[ptr].checked_sub(1).unwrap_or(u8::MAX);
            }
            PRINT_CHAR => {
                print!("{}", cells[ptr] as char);
            }
            INPUT_CHAR => {
                // TODO: input is kinda cooked, find a buffered solution
                // currently input is restricted to one char per line

                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                if let Some(c) = input.as_bytes().first() {
                    cells[ptr] = *c as u8;
                }
            }
            START_LOOP => {
                if cells[ptr] == 0 {
                    let mut depth = 1;
                    while depth > 0 {
                        if pc >= code.len() {
                            panic!("Parsing error: unmatched [");
                        }
                        pc += 1;
                        match line.chars().filter(|c| *c == 'o').count() {
                            // TODO: broken logic
                            START_LOOP => depth += 1,
                            END_LOOP => depth -= 1,
                            _ => {}
                        }
                    }
                }
            }
            END_LOOP => {
                if cells[ptr] != 0 {
                    let mut depth = 1;
                    while depth > 0 {
                        if pc == 0 {
                            panic!("Parsing error: unmatched ]");
                        }
                        pc -= 1;
                        match line.chars().filter(|c| *c == 'o').count() {
                            START_LOOP => depth += 1,
                            END_LOOP => depth -= 1,
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }
        if line.contains("sh") {
            break;
        }

        pc += 1;
    }
}

fn transpile_from_bf(code: &str) {
    let mut pc = 0;

    print!("j");

    while pc < code.len() {
        let c = code.chars().nth(pc).unwrap();

        match c {
            '>' => {
                println!("o");
            }
            '<' => {
                println!("oo");
            }
            '+' => {
                println!("ooo");
            }
            '-' => {
                println!("oooo");
            }
            '.' => {
                println!("ooooo");
            }
            ',' => {
                println!("oooooo");
            }
            '[' => {
                let mut depth = 1;
                let mut _pc = pc;
                while depth > 0 {
                    if _pc >= code.len() {
                        panic!("Parsing error: unmatched [");
                    }
                    _pc += 1;
                    match code.chars().nth(_pc).unwrap() {
                        '[' => depth += 1,
                        ']' => depth -= 1,
                        _ => {}
                    }
                }
                println!("ooooooo");
            }
            ']' => {
                let mut depth = 1;
                let mut _pc = pc;
                while depth > 0 {
                    if _pc == 0 {
                        panic!("Parsing error: unmatched ]");
                    }
                    _pc -= 1;
                    match code.chars().nth(_pc).unwrap() {
                        ']' => depth += 1,
                        '[' => depth -= 1,
                        _ => {}
                    }
                }
                println!("oooooooo");
            }
            _ => {
                print!("{c}");
            }
        }

        pc += 1;
    }

    println!("sh");
}
