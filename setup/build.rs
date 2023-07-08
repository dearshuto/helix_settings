extern crate helix_loader;
use helix_loader::grammar::{build_grammars, fetch_grammars};

fn main() {
    fetch_grammars().expect("");
    build_grammars(Some(std::env::var("TARGET").unwrap())).expect("");
}
