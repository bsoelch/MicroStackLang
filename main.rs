use std::fs;
use std::io::{self, Write};

// tokenizer
#[derive(Debug,Clone,Copy)]
enum Token {
    Vpush(i16), // push immediate value to value stack
    CpushV, // move top value stack value to call stack
    VpushC, // move top call stack value to value stack
    Vswap, // swap top two values
    Vdup, // duplicate top value
    Vdrop, // discrad top value
    VswapIf, // discard top stack value, swap next two elements if value is non-zero
    CswapIp, // swap current instruction pointer (position of next instruction) with top value on call-stack
    IsNegative,
    Add,
    Sub,
    Read,
    Print,
    Mswap,
    // one additional op allowed
}

fn tokenize<'a>(input: &'a [u8]) -> Vec<Token> {
    let mut output = Vec::new();
    let mut num_start = 0;
    let mut is_num = false;
    for (i, &c) in input.into_iter().enumerate() {
        if c.is_ascii_digit() {
            if ! is_num {
                num_start = i;
                is_num = true;
            }
        } else if is_num {
            is_num = false;
            let num = str::from_utf8(&input[num_start..i]).unwrap().parse::<i64>().unwrap() as i16;
            output.push(Token::Vpush(num));
        }
        match c {
            b'>' => {output.push(Token::CpushV);}
            b'<' => {output.push(Token::VpushC);}
            b'^' => {output.push(Token::Vswap);}
            b':' => {output.push(Token::Vdup);}
            b'.' => {output.push(Token::Vdrop);}
            b'?' => {output.push(Token::VswapIf);}
            b';' => {output.push(Token::CswapIp);}
            b'~' => {output.push(Token::IsNegative);}
            b'+' => {output.push(Token::Add);}
            b'-' => {output.push(Token::Sub);}
            b'_' => {output.push(Token::Read);}
            b'"' => {output.push(Token::Print);}
            b'@' => {output.push(Token::Mswap);}
            _ => {}
        }
    }
    return output
}

// interpreter
fn run_program(tokens: &Vec<Token>) {
    let mut memory: [i16; 0x10000] = [0; 0x10000]; // storing this on the stack is a bad idea, but rust does not like mutable global variables
    let mut vstack = Vec::new();
    let mut cstack = Vec::new();
    let mut ip = 0;
    let mut n = 0;
    let debug = false;
    while ip < tokens.len() {
        let next_token = &tokens[ip];
        if debug {
            println!("{ip}: {:?} {:?} {:?}",next_token,vstack,cstack);
            n += 1;
            if n > 250 {return;}
        }
        ip += 1;
        match next_token {
            Token::Vpush(val) => {vstack.push(*val);}
            Token::CpushV => {cstack.push(vstack.pop().unwrap_or_default());}
            Token::VpushC => {vstack.push(cstack.pop().unwrap_or_default());}
            Token::Vswap => {
                let a = vstack.pop().unwrap_or_default();
                let b = vstack.pop().unwrap_or_default();
                vstack.push(a);
                vstack.push(b);
            }
            Token::Vdup => {
                vstack.push(vstack.last().map(|x|*x).unwrap_or_default());
            }
            Token::Vdrop => {
                vstack.pop();
            }
            Token::VswapIf => {
                let c = vstack.pop().unwrap_or_default();
                if c != 0 {
                    let a = vstack.pop().unwrap_or_default();
                    let b = vstack.pop().unwrap_or_default();
                    vstack.push(a);
                    vstack.push(b);
                }
            }
            Token::CswapIp => {
                let old_ip = ip as i16;
                ip = cstack.pop().unwrap_or_default() as u16 as usize;
                cstack.push(old_ip);
            }
            Token::IsNegative => {
                let c = vstack.pop().unwrap_or_default();
                vstack.push(if c < 0 {1} else {0});
            }
            Token::Add => {
                let b = vstack.pop().unwrap_or_default();
                let a = vstack.pop().unwrap_or_default();
                vstack.push(a+b);
            }
            Token::Sub => {
                let b = vstack.pop().unwrap_or_default();
                let a = vstack.pop().unwrap_or_default();
                vstack.push(a-b);
            }
            Token::Read => {
                panic!("unimplemented: read");
            }
            Token::Print => {
                let a = vstack.pop().unwrap_or_default();
                print!("{}",a as u8 as char);
            }
            Token::Mswap => {
                let val = vstack.pop().unwrap_or_default();
                let id = vstack.pop().unwrap_or_default();
                let old_val = memory[id as usize];
                memory[id as usize] = val;
                vstack.push(old_val);
            }
        }
    }
}

// main
fn main() -> io::Result<()> {
    // Read the content of the input file
    let input = fs::read("in.txt")?;

    let tokens = tokenize(&input);

    // Write the output to the output file
    let mut out_file = fs::File::create("tokens.txt")?;
    for &token in tokens.iter() {
        writeln!(out_file, "{:?}", token)?;
    }
    run_program(&tokens);

    Ok(())
}
