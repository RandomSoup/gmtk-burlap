use std::env;
use std::fs;
use std::io::{BufReader, BufRead};

use crate::repl::get_repl_line;

// Stream
#[derive(Debug, Clone)]
pub struct Stream {
    // File name
    pub name: String,
    // Line #
    pub line: usize,
    // Char pos in line
    pub at: usize,
    // Real AT, the position in the source
    pub rat: usize,
    // Size of token
    pub size: usize,
}

pub const IMPOSSIBLE_STATE: &str =
    "we've reached an impossible state, anything is possible, \
    the limits were in our heads all along, follow your dreams";

// Errors
pub enum ErrType{Err, Warn, Hint}
// Prints and error and returns the color
pub fn print_err(msg: &str, errtype: ErrType, color: bool) -> String {
    // Get the name and color code from errtype
    let (color_code, name) = match errtype {
        // Red
        ErrType::Err => ("\x1b[1;31m", "error"),
        // Yellow
        ErrType::Warn => ("\x1b[1;33m", "warning"),
        // Cyan
        ErrType::Hint => ("\x1b[1;36m", "hint"),
    };
    if color {
        println!("{}{}:\x1b[0m {}", color_code.clone(), name, msg);
        return color_code.to_string();
    } else {
        println!("{}: {}", name, msg);
        return "".to_string();
    }
}

fn get_line(stream: &Stream) -> String {
    // Special cases
    if stream.name == "<cli>" {
        let mut args = env::args();
        args.position(|x| x == "-");
        return args.next()
            .unwrap().lines().nth(stream.line - 1)
            .expect("failed to read file for errors").to_string()
    } else if stream.name == "<stdin>" {
        return get_repl_line().clone().lines().nth(stream.line - 1)
            .unwrap().to_string();
    }

    let name = stream.name.clone();
    // Open
    let Ok(file) = fs::File::open(name) else {
        panic!("Failed to open file for error printing!");
    };
    // Read
    return BufReader::new(file).lines().nth(stream.line - 1)
        .expect("failed to read file for errors").unwrap()
}

pub fn err(stream: &Stream, msg: &str, errtype: ErrType, color: bool) {
    let line = get_line(stream);
    // Print file name and line/char info ("test.sk:1:3: ")
    if color {
        print!("\x1b[1m{}:{}:{}:\x1b[0m ", stream.name, stream.line, stream.at);
    } else {
        print!("{}:{}:{}: ", stream.name, stream.line, stream.at);
    }
    // Print the type ("error:")
    let color_code = print_err(msg, errtype, color);
    // Print the line ("    1 | print("Hello World!");")
    let prefix = format!("    {} | ", stream.line);
    println!("{}{}", prefix, line.replace('\t', "    "));
    // Print arrow ("      |   ^")
    // Adjust for tabs
    let at = line[0..stream.at].matches('\t').count()*3 + stream.at;
    print!(
        "{}| {}",
        " ".repeat(prefix.len() - 2), " ".repeat(at)
    );
    if color {
        println!("{}{}\x1b[0m", color_code, "^".repeat(stream.size));
    } else {
        println!("{}", "^".repeat(stream.size));
    }
}
