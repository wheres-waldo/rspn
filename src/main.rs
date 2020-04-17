use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let calculation = &args[1..];
    let mut stack: Vec<i32> = vec![];

    calculation.iter().for_each(|arg| {
        if let Ok(i) = arg.parse::<i32>() {
            stack.push(i);
        } else {
            match arg.as_bytes() {
                [b'+'] => {
                    if let Some(i) = stack.pop() {
                        if let Some(j) = stack.pop() {
                            stack.push(i + j);
                        } else {
                            panic!("Not enough operands");
                        }
                    } else {
                        panic!("Not enough operands");
                    }
                }
                [b'-'] => {
                    if let Some(i) = stack.pop() {
                        if let Some(j) = stack.pop() {
                            stack.push(i - j);
                        } else {
                            panic!("Not enough operands");
                        }
                    } else {
                        panic!("Not enough operands");
                    }
                },
                [b'/'] => {
                    if let Some(i) = stack.pop() {
                        if let Some(j) = stack.pop() {
                            stack.push(i / j);
                        } else {
                            panic!("Not enough operands");
                        }
                    } else {
                        panic!("Not enough operands");
                    }
                },
                [b'*'] => {
                    if let Some(i) = stack.pop() {
                        if let Some(j) = stack.pop() {
                            stack.push(i * j);
                        } else {
                            panic!("Not enough operands");
                        }
                    } else {
                        panic!("Not enough operands");
                    }
                },
                _ => panic!("WTF ARE YOU FEEDING ME!!")
            }
        }
    });

    println!("{}", stack.pop().unwrap())
}
