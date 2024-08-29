use std::io::prelude::*;

use term::{self, color::Color};

pub fn error(message: &str) {
    if let Some(mut t) = term::stderr() {
        match t.fg(term::color::RED) {
            Ok(_) => {
                write!(t, "{}", message).unwrap();
                t.reset().unwrap();
            },
            Err(_) => writeln!(t, "{}", message).unwrap()
        };
    } else {
        eprint!("{}", message);
    }
}

pub fn warning(message: &str) {
    if let Some(mut t) = term::stderr() {
        match t.fg(term::color::YELLOW) {
            Ok(_) => {
                write!(t, "{}", message).unwrap();
                t.reset().unwrap();
            },
            Err(_) => writeln!(t, "{}", message).unwrap()
        };
    } else {
        eprint!("{}", message);
    }
}

pub fn success(message: &str) {
    if let Some(mut t) = term::stdout() {
        match t.fg(term::color::GREEN) {
            Ok(_) => {
                write!(t, "{}", message).unwrap();
                t.reset().unwrap();
            },
            Err(_) => writeln!(t, "{}", message).unwrap()
        };
    } else {
        eprint!("{}", message);
    }
}

pub fn write(message: &str, color: Color) {
    if let Some(mut t) = term::stdout() {
        match t.fg(color) {
            Ok(_) => {
                write!(t, "{}", message).unwrap();
                t.reset().unwrap();
            },
            Err(_) => writeln!(t, "{}", message).unwrap()
        };
    } else {
        eprint!("{}", message);
    }
}
