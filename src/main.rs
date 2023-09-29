use std::cmp;
use std::env;
use std::process::exit;

use colored::*;

mod util;
use util::get_file_content;

#[derive(Debug)]
enum DiffSymbol {
    Insert,
    Delete,
}

#[derive(Debug)]
struct DiffTrace {
    symbol: Option<DiffSymbol>,
}

impl DiffTrace {
    fn close(&mut self) -> String {
        match self.symbol {
            Some(DiffSymbol::Insert) => {
                self.symbol = None;
                "+}".green().to_string()
            }
            Some(DiffSymbol::Delete) => {
                self.symbol = None;
                "-]".red().to_string()
            }
            None => "".to_string(),
        }
    }

    fn delete(&mut self) -> String {
        match self.symbol {
            Some(DiffSymbol::Insert) => {
                self.symbol = Some(DiffSymbol::Delete);
                format!("{}{}", "+}".green(), "[-".red())
            }
            Some(DiffSymbol::Delete) => "".to_string(),
            None => {
                self.symbol = Some(DiffSymbol::Delete);
                "[-".red().to_string()
            }
        }
    }

    fn insert(&mut self) -> String {
        match self.symbol {
            Some(DiffSymbol::Insert) => "".to_string(),
            Some(DiffSymbol::Delete) => {
                self.symbol = Some(DiffSymbol::Insert);
                format!("{}{}", "-]".red(), "{+".green())
            }
            None => {
                self.symbol = Some(DiffSymbol::Insert);
                "{+".green().to_string()
            }
        }
    }
}

enum DiffColor {
    Red,
    Green,
}

impl DiffColor {
    fn get_string(&self, ch: &char) -> String {
        match self {
            DiffColor::Red => ch.to_string().red().to_string(),
            DiffColor::Green => ch.to_string().green().to_string(),
        }
        
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len().ne(&3) {
        println!("usage: diffr <file_1> <file_2>");
        exit(1);
    }

    let file_1 = &args[1];
    let file_2 = &args[2];

    let content_1 = get_file_content(file_1).unwrap_or_else(|err| {
        println!("diffr: {}: {}", file_1, err.to_string());
        exit(2);
    });
    let content_2 = get_file_content(file_2).unwrap_or_else(|err| {
        println!("diffr: {}: {}", file_2, err.to_string());
        exit(2);
    });

    let diff = diff_char(&content_1, &content_2);
    println!("{}", diff);
}

fn diff_char(data_1: &String, data_2: &String) -> String {
    let u: Vec<char> = data_1.chars().collect();
    let v: Vec<char> = data_2.chars().collect();

    let m = u.len();
    let n = v.len();
    let mut lcs = vec![vec![0; n + 1]; m + 1];

    for r in 0..m + 1 {
        lcs[r][n] = 0;
    }
    for c in 0..n + 1 {
        lcs[m][c] = 0;
    }

    for c in (0..n).rev() {
        for r in (0..m).rev() {
            if u[r].eq(&v[c]) {
                lcs[r][c] = 1 + lcs[r + 1][c + 1];
            } else {
                lcs[r][c] = cmp::max(lcs[r + 1][c], lcs[r][c + 1]);
            }
        }
    }

    let mut r = 0;
    let mut c = 0;
    let mut diff = "".to_string();
    let mut diff_symbol = DiffTrace { symbol: None };

    while r < m && c < n {
        if u[r].eq(&v[c]) {
            diff.push_str(diff_symbol.close().as_ref());
            diff.push(u[r]);
            r += 1;
            c += 1;
        } else if lcs[r + 1][c].ge(&lcs[r][c + 1]) {
            diff.push_str(diff_symbol.delete().as_ref());
            diff.push_str(DiffColor::Red.get_string(&u[r]).as_ref());
            r += 1;
        } else {
            diff.push_str(diff_symbol.insert().as_ref());
            diff.push_str(DiffColor::Green.get_string(&v[c]).as_ref());
            c += 1;
        }
    }

    while r < m {
        diff.push_str(diff_symbol.delete().as_ref());
        diff.push_str(DiffColor::Red.get_string(&u[r]).as_ref());
        r += 1;
    }
    while c < n {
        diff.push_str(diff_symbol.insert().as_ref());
        diff.push_str(DiffColor::Green.get_string(&v[c]).as_ref());
        c += 1;
    }

    diff.push_str(diff_symbol.close().as_ref());

    diff
}
