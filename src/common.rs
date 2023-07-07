use crate::THE_SOURCE;

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

// Fix println! not working in wasm
#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;
#[cfg(target_family = "wasm")]
#[wasm_bindgen]
#[cfg(target_family = "wasm")]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[cfg(target_family = "wasm")]
macro_rules! println {
    ($($t:tt)*) => (crate::common::log(&format_args!($($t)*).to_string()))
}

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
    THE_SOURCE.lines().nth(stream.line - 1)
        .expect("failed to read file for errors").to_string()
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

fn _get_builtins(extended: bool) -> Vec<(String, i32)> {
    let mut ret: Vec<(String, i32)> = vec![
        ("print", 1),
        ("input", 1),
        ("type", 1),
        ("len", 1),
        ("range", 2),
        ("args", 0),
        ("open", 2),
        ("close", 1),
        ("read", 1),
        ("write", 2),
        ("seek", 2),
        ("flush", 1),
        ("int", 1),
        ("float", 1),
        ("string", 1),
        ("byte", 1),
        ("__burlap_range", 2),
        ("init", 3),
        ("should_close", 0),
        ("begin_drawing", 0),
        ("clear_background", 1),
        ("draw_text", 5),
        ("load_texture", 1),
        ("draw_texture", 4),
    ].iter().map(|(n, a)| (n.to_string(), *a)).collect();
    if extended {
        let mut tmp = vec![
            ("__burlap_typed_eq", 2),
            ("__burlap_print", 1),
            ("__burlap_throw", 1),
        ].iter().map(|(n, a)| (n.to_string(), *a)).collect();
        ret.append(&mut tmp);
        #[cfg(feature = "cffi")]
        {
            tmp = vec![
                ("__burlap_load_lib", 1),
                ("__burlap_load_functi", 2),
                ("__burlap_ffi_call", 3),
                ("__burlap_ptr", 1),
            ].iter().map(|(n, a)| (n.to_string(), *a)).collect();
            ret.append(&mut tmp);
        }
    }
    return ret;
}

pub fn get_builtins(extended: bool) -> &'static Vec<(String, i32)> {
    unsafe {
        static mut BUILTINS: Option<Vec<(String, i32)>> = None;
        static mut EXTENDED: bool = false;
        if BUILTINS == None || EXTENDED != extended {
            EXTENDED = extended;
            BUILTINS = Some(_get_builtins(extended));
        }
        return BUILTINS.as_mut().unwrap();
    }
}
