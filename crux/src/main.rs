mod engine;

use std::io::BufRead;

use crux_lib::{
    engine::run::run_line,
    protocol::{usi::Usi, Protocol},
};

use crate::engine::Crux;

fn main() {
    let mut crux = Crux::default();

    let stdin = std::io::stdin();

    for line in stdin.lock().lines() {
        let line = line.unwrap();

        match run_line::<Usi, _, _>(&mut crux, &line, |event| {
            println!("{}", Usi::format_event(&event))
        }) {
            Ok(quit) => {
                if quit {
                    return;
                }
            }
            Err(e) => println!("{e:?}"),
        }
    }
}
