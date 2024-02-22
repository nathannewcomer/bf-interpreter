use std::env;
use std::fs;
use std::io::Read;
use std::io::Write;

#[derive(Debug)]
enum Instruction {
    MoveLeft,
    MoveRight,
    Increment,
    Decrement,
    Output,
    Input,
    Loop(Box<Vec<Instruction>>)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: brainfuck [filename]");
        return;
    }

    // Get content form file
    let contents = match fs::read_to_string(&args[1]) {
        Ok(text) => text,
        Err(_) => panic!("Could not open file.")
    };

    let instrs = scan(&contents);

    let mut memory: [u8; 30_000] = [0; 30_000];
    let mut p = 0;
    let _ = std::io::stdout().flush();
    interpret(&instrs, &mut memory, &mut p);
    //print_tree(&instrs, &mut p);
}

fn scan(content: &String) -> Vec<Instruction> {
    let mut current: usize = 0;
    let chars: Vec<char> = content.chars().collect();
    let mut instructions: Vec<Instruction> = Vec::new();

    while current < chars.len() {
        if let Some(instr) = scan_instr(&chars, &mut current) {
            instructions.push(instr);
        }
    }

    instructions
}

fn scan_instr(chars: &Vec<char>, current: &mut usize) -> Option<Instruction> {
    let ch = chars[*current];
    *current += 1;
    match ch {
        '>' => Some(Instruction::MoveLeft),
        '<' => Some(Instruction::MoveRight),
        '+' => Some(Instruction::Increment),
        '-' => Some(Instruction::Decrement),
        '.' => Some(Instruction::Output),
        ',' => Some(Instruction::Input),
        '[' => Some(scan_loop(chars, current)),
        ']' => {
            *current -= 1;
            None
        },
        _ => None
    }
}

fn scan_loop(chars: &Vec<char>, current: &mut usize) -> Instruction {
    let mut loop_instr: Vec<Instruction> = Vec::new();
    while chars[*current] != ']' {
        if let Some(instr) = scan_instr(chars, current) {
            loop_instr.push(instr);
        }
    }
    *current += 1;  // move past ] char
    Instruction::Loop(Box::new(loop_instr))
}

fn interpret(instrs: &Vec<Instruction>, memory: &mut [u8], p: &mut usize) {
    for instr in instrs {
        match instr {
            Instruction::MoveLeft => *p = p.wrapping_add(1),
            Instruction::MoveRight => *p = p.wrapping_sub(1),
            Instruction::Increment => memory[*p] = memory[*p].wrapping_add(1),
            Instruction::Decrement => memory[*p] = memory[*p].wrapping_sub(1),
            Instruction::Output => print!("{}", memory[*p] as char),
            Instruction::Input => {
                let mut buf: [u8; 1] = [0; 1];
                let _ = std::io::stdin().read_exact(&mut buf);
                memory[*p] = buf[0] as u8;
            },
            Instruction::Loop(loop_instrs) => {
                while memory[*p] != 0 {
                    interpret(loop_instrs, memory, p);
                }
            },
        }
    }
}



// fn print_tree(instrs: &Vec<Instruction>, current: &mut usize) {
//     while *current < instrs.len() {
//         match &instrs[*current] {
//             Instruction::MoveLeft => print!(">"),
//             Instruction::MoveRight => print!("<"),
//             Instruction::Increment => print!("+"),
//             Instruction::Decrement => print!("-"),
//             Instruction::Output => print!("."),
//             Instruction::Input => print!(","),
//             Instruction::Loop(loop_instrs) => {
//                 print!("[");
//                 print_tree(&loop_instrs, &mut 0);
//                 print!("]");
//             },
//         }
//         *current += 1;
//     }
// }