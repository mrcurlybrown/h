use std::fs;
// use std::io::prelude::*;
use synoptic::{from_extension, TokOpt};
use lliw::Fg;

fn main() {
    let path = "/home/tejasdeshpande/.bash_history";

    let code = fs::read_to_string(path)
        .expect("Should have been able to read the file");
    
    let code = code
        .split('\n')
        .map(|line| line.to_string())
        .collect::<Vec<String>>();
    
    let mut h = from_extension("bash", 4).expect("Highlighter should have been created.");

    h.keyword("keyword", r"\b(git|sudo|apt)\b");
    
    // The run method takes a vector of strings (for each line)
    h.run(&code);
    
    // Render the output
    for (line_number, line) in code.iter().enumerate() {
        if line_number + 1 == code.len() {
            break;
        }
        print!("{}\t", line_number + 1);
        // Line returns tokens for the corresponding line from file
        for token in h.line(line_number, &line) {
            // Tokens can either require highlighting or not require highlighting
            match token {
                // This is some text that needs to be highlighted
                TokOpt::Some(text, kind) => print!("{}{text}{}", colour(&kind), Fg::Reset),
                // This is just normal text with no highlighting
                TokOpt::None(text) => print!("{text}"),
            }
        }
        // Insert a newline at the end of every line
        println!();
    }
}

fn colour(name: &str) -> Fg {
    // This function will take in the function name
    // And it will output the correct foreground colour
    match name {
        "comment" => Fg::LightBlack,
        "digit" => Fg::Purple,
        "string" => Fg::Green,
        "macros" => Fg::LightPurple,
        "boolean" => Fg::Blue,
        "keyword" => Fg::Yellow,
        "operator" => Fg::Red,
        _ => panic!("unknown token name: {name}"),
    }
}
