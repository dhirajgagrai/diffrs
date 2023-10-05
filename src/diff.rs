use std::cmp;

use colored::*;

#[derive(Debug)]
enum DiffSymbol {
    Insert,
    Delete,
}

// Tracks the last symbol that was used
struct DiffTrace {
    symbol: Option<DiffSymbol>,
}

// Insert symbols depending on the last used symbol
impl DiffTrace {
    fn close(&mut self) -> String {
        match self.symbol {
            Some(DiffSymbol::Insert) => {
                self.symbol = None;
                return "+}".green().to_string();
            }
            Some(DiffSymbol::Delete) => {
                self.symbol = None;
                return "-]".red().to_string();
            }
            None => "".to_string(),
        }
    }

    fn delete(&mut self) -> String {
        match self.symbol {
            Some(DiffSymbol::Insert) => {
                self.symbol = Some(DiffSymbol::Delete);
                return format!("{}{}", "+}".green(), "[-".red());
            }
            Some(DiffSymbol::Delete) => "".to_string(),
            None => {
                self.symbol = Some(DiffSymbol::Delete);
                return "[-".red().to_string();
            }
        }
    }

    fn insert(&mut self) -> String {
        match self.symbol {
            Some(DiffSymbol::Insert) => "".to_string(),
            Some(DiffSymbol::Delete) => {
                self.symbol = Some(DiffSymbol::Insert);
                return format!("{}{}", "-]".red(), "{+".green());
            }
            None => {
                self.symbol = Some(DiffSymbol::Insert);
                return "{+".green().to_string();
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

pub fn diff(string_1: &String, string_2: &String) -> String {
    let u: Vec<char> = string_1.chars().collect();
    let v: Vec<char> = string_2.chars().collect();

    let m = u.len();
    let n = v.len();
    let mut lcs = vec![vec![0; n + 1]; m + 1];

    // LCS algorithm
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

    // Initialize variables for traceback
    let mut r = 0;
    let mut c = 0;
    let mut diff = "".to_string();
    let mut diff_symbol = DiffTrace { symbol: None };

    // Start traceback from column = 0 & row = 0
    while r < m || c < n {
        if r < m && !(c < n) {
            diff.push_str(diff_symbol.delete().as_ref());
            diff.push_str(DiffColor::Red.get_string(&u[r]).as_ref());
            r += 1;
        } else if c < n && !(r < m) {
            diff.push_str(diff_symbol.insert().as_ref());
            diff.push_str(DiffColor::Green.get_string(&v[c]).as_ref());
            c += 1;
        } else if u[r].eq(&v[c]) {
            diff.push_str(diff_symbol.close().as_ref());
            diff.push(u[r]);
            r += 1;
            c += 1;
        } else if lcs[r + 1][c].ge(&lcs[r][c + 1]) {
            diff.push_str(diff_symbol.delete().as_ref());
            diff.push_str(DiffColor::Red.get_string(&u[r]).as_ref());
            r += 1;
        } else if lcs[r + 1][c].lt(&lcs[r][c + 1]) {
            diff.push_str(diff_symbol.insert().as_ref());
            diff.push_str(DiffColor::Green.get_string(&v[c]).as_ref());
            c += 1;
        }
    }

    diff.push_str(diff_symbol.close().as_ref());

    return diff;
}
