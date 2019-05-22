use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

use super::mokou;

#[derive(Debug)]
enum Input {
    Exit,
    Ls,
    Cd,
    Mokou,
    Else,
    Enter,
}

pub fn repl() {
    loop {
        match rep() {
            Input::Exit => break,
            _ => continue,
        }
    }
}

fn rep() -> Input {
    print!("$ ");
    io::stdout().flush().ok();

    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();

    let args = input
        .split_whitespace()
        .map(|token| String::from(token))
        .collect::<Vec<String>>();

    if args.len() > 0 {
        let command = &args[0];
        let com = parse(&command);
        match com {
            Input::Exit => com,
            _ => {
                do_command(&command, &com, &args);
                com
            }
        }
    } else {
        Input::Enter
    }
}

fn parse(command: &str) -> Input {
    match command {
        "exit" | "bye" | "quit" => Input::Exit,
        "ls" => Input::Ls,
        "cd" => Input::Cd,
        "mokou" => Input::Mokou,
        _ => Input::Else,
    }
}

fn do_command(command: &str, com: &Input, args: &Vec<String>) {
    match com {
        Input::Ls => ls(args),
        Input::Cd => cd(args),
        Input::Mokou => mokou::mokou(),
        Input::Else => exec(command, args),
        _ => println!(
            "実装してないコマンドですが、なにか問題でも？: {}",
            command
        ),
    }
}

fn exec(command: &str, args: &Vec<String>) {
    let mut args = args.clone();
    if args.len() > 0 {
        args.remove(0);
    }
    let child = Command::new(command).args(args).spawn();
    match child {
        Ok(mut child) => {
            let ecode = child.wait().expect("command wasn't runnning.");
            ecode.code().expect("exec command fail");
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}

fn ls(args: &Vec<String>) {
    let len = args.len();
    let path = if len == 1 { "./" } else { &args[1] };

    match std::fs::read_dir(path) {
        Ok(paths) => {
            for p in paths {
                println!("{}", p.unwrap().file_name().to_str().unwrap());
            }
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}

fn cd(args: &Vec<String>) {
    let len = args.len();
    let path = if len == 1 { "./" } else { &args[1] };
    match std::env::set_current_dir(Path::new(path)) {
        Ok(()) => (),
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}
