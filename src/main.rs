use lliw::Fg;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use synoptic::{from_extension, TokOpt};

fn main() {
    let args: Vec<String> = env::args().collect();

    let run_env_vars: HashMap<&str, String> = HashMap::from([
        (
            "HOME",
            env::var("HOME").expect("Environment variable: \"HOME\" not found."),
        ),
        (
            "SHELL",
            env::var("SHELL")
                .expect("Environment variable: \"SHELL\" not found.")
                .split("/")
                .last()
                .expect("String was unable to be split.")
                .to_string(),
        ),
    ]);

    let (code, h) = get_history_commands(&run_env_vars);

    if args.len() > 1 {
        let line_num: usize = args[1].parse().expect("Not a valid number");
        let cmd = &code[line_num - 1];

        println!("{}", &cmd);
        Command::new(&run_env_vars["SHELL"])
            .args(["-c", &cmd])
            .spawn()
            .expect(&format!("Command failed to start: \"{}\"", &cmd));
    } else {
        // Render the output
        render_history(code, h);
    }
}

fn render_history(code: Vec<String>, h: synoptic::Highlighter) {
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

fn get_history_commands(env_vars: &HashMap<&str, String>) -> (Vec<String>, synoptic::Highlighter) {
    // let home_dir: String = env::var("HOME").expect("Environment variable: \"HOME\" not found.");

    let path = PathBuf::from(format!("{}/.bash_history", env_vars["HOME"]));

    let code = fs::read_to_string(path).expect("Should have been able to read the file");

    let code = code
        .split('\n')
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

    let mut h = from_extension("bash", 4).expect("Highlighter should have been created.");

    h.keyword("keyword", r"\b(git|sudo|apt)\b");

    // The run method takes a vector of strings (for each line)
    h.run(&code);
    (code, h)
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
