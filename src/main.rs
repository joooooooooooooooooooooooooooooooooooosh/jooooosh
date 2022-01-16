use std::{env, fs};

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

    while pc < code.len() {
        let c = code.chars().nth(pc).unwrap();

        match c {
            '>' => {
                ptr += 1;
                if ptr >= cells.len() {
                    cells.push(0);
                }
            }
            '<' => {
                ptr -= 1;
            }
            '+' => {
                cells[ptr] = cells[ptr].checked_add(1).unwrap_or(0);
            }
            '-' => {
                cells[ptr] = cells[ptr].checked_sub(1).unwrap_or(u8::MAX);
            }
            '.' => {
                print!("{}", cells[ptr] as char);
            }
            ',' => {
                // TODO: input is kinda cooked, find a buffered solution
                // currently input is restricted to one char per line

                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                if let Some(c) = input.as_bytes().first() {
                    cells[ptr] = *c as u8;
                }
            }
            '[' => {
                if cells[ptr] == 0 {
                    let mut depth = 1;
                    while depth > 0 {
                        if pc >= code.len() {
                            panic!("Parsing error: unmatched [");
                        }
                        pc += 1;
                        match code.chars().nth(pc).unwrap() {
                            '[' => depth += 1,
                            ']' => depth -= 1,
                            _ => {}
                        }
                    }
                }
            }
            ']' => {
                if cells[ptr] != 0 {
                    let mut depth = 1;
                    while depth > 0 {
                        if pc == 0 {
                            panic!("Parsing error: unmatched ]");
                        }
                        pc -= 1;
                        match code.chars().nth(pc).unwrap() {
                            ']' => depth += 1,
                            '[' => depth -= 1,
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
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
