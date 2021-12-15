use crate::loadinput::loadinput;

fn brace_lut(chr: char) -> usize {
    match chr {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _   => 0,
    }
}

pub fn day10p1() {
    let input = loadinput("./input/day10.txt");
    let mut accumulator = 0;

    for line in &input {
        let mut chunk_stack : Vec<char> = Vec::new();
        for chr in line.chars() {
            if (chr == '(') ||
               (chr == '[') ||
               (chr == '{') ||
               (chr == '<') {
                chunk_stack.push(chr);
            }else{
                if let Some(check) = chunk_stack.pop() {
                    match chr {
                        ')' => {
                            if check != '(' {
                                accumulator += brace_lut(chr);
                                break;                     
                            }                              
                        },                                 
                        ']' => {                           
                            if check != '[' {              
                                accumulator += brace_lut(chr);
                                break;                     
                            }                              
                        },                                 
                        '}' => {                           
                            if check != '{' {              
                                accumulator += brace_lut(chr);
                                break;                     
                            }                              
                        },                                 
                        '>' => {                           
                            if check != '<' {              
                                accumulator += brace_lut(chr);
                                break;
                            }
                        },
                        _ => {
                            eprintln!("Uh oh");
                        }
                    }
                }
            }
        }
    }
    println!("Corruption score: {}", accumulator);
}

pub fn day10p2() {
    let input = loadinput("./input/day10.txt");
    let mut corrupt_lines : Vec<usize> = Vec::new();

    for (lnum, line) in input.iter().enumerate() {
        let mut chunk_stack : Vec<char> = Vec::new();
        for chr in line.chars() {
            if (chr == '(') ||
               (chr == '[') ||
               (chr == '{') ||
               (chr == '<') {
                chunk_stack.push(chr);
            }else{
                if let Some(check) = chunk_stack.pop() {
                    match chr {
                        ')' => {
                            if check != '(' {
                                corrupt_lines.push(lnum);
                                break;                     
                            }                              
                        },                                 
                        ']' => {                           
                            if check != '[' {              
                                corrupt_lines.push(lnum);
                                break;                     
                            }                              
                        },                                 
                        '}' => {                           
                            if check != '{' {              
                                corrupt_lines.push(lnum);
                                break;                     
                            }                              
                        },                                 
                        '>' => {                           
                            if check != '<' {              
                                corrupt_lines.push(lnum);
                                break;
                            }
                        },
                        _ => {
                            eprintln!("Uh oh");
                        }
                    }
                }
            }
        }
    }

    let mut incomplete_lines = input.clone();
    corrupt_lines.sort(); corrupt_lines.reverse();
    for lnum in corrupt_lines { incomplete_lines.remove(lnum); }
    let mut line_endings : Vec<String> = Vec::new();

    for line in &incomplete_lines {
        let mut line_ending : String = String::new();
        let mut chunk_stack : Vec<char> = Vec::new();
        for chr in line.chars() {
            if (chr == '(') ||
               (chr == '[') ||
               (chr == '{') ||
               (chr == '<') {
                chunk_stack.push(chr);
            }else{
                chunk_stack.pop();
            }
        }
        while let Some(brace) = chunk_stack.pop() {
            match brace {
                '(' => line_ending.push(')'),
                '[' => line_ending.push(']'),
                '{' => line_ending.push('}'),
                '<' => line_ending.push('>'),
                _ => ()
            }
        }
        line_endings.push(line_ending);
    }

    let mut scores : Vec<usize> = Vec::new();
    for line_ending in &line_endings {
        let mut score = 0;
        for chr in line_ending.chars() {
            score *= 5;
            score += match chr {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _   => 0,
            };
        }
        //println!("{}: {}", line_ending, score);
        scores.push(score);
    }

    scores.sort();
    println!("Middle score: {}", scores[scores.len() / 2]);
}
