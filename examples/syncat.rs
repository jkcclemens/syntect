extern crate syntect;
use syntect::package_set::PackageSet;
use syntect::parser::*;
use syntect::theme::highlighter::*;
use syntect::theme::style::*;
use syntect::util::{as_24_bit_terminal_escaped, debug_print_ops};

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn main() {
    let ps = PackageSet::load_from_folder("testdata/Packages").unwrap();
    let mut state = {
        let syntax = ps.find_syntax_by_name("Ruby on Rails").unwrap();
        ParseState::new(syntax)
    };
    let highlighter = Highlighter::new(PackageSet::get_theme("testdata/spacegray/base16-ocean.\
                                                              dark.tmTheme")
        .unwrap());

    let f = File::open("src/scope.rs").unwrap();
    let file = BufReader::new(&f);

    let mut highlight_state = HighlightState::new(&highlighter, state.scope_stack.clone());
    for maybe_line in file.lines() {
        let line = maybe_line.unwrap();
        // println!("{}", state.scope_stack);
        let ops = state.parse_line(&line);
        // debug_print_ops(&line, &ops);
        let iter = HighlightIterator::new(&mut highlight_state, &ops[..], &line, &highlighter);
        let regions: Vec<(Style, &str)> = iter.collect();
        let escaped = as_24_bit_terminal_escaped(&regions[..], true);
        println!("{}", escaped);
    }
}