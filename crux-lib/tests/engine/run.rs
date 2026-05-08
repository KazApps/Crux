use crux_lib::{
    engine::{
        event::EngineEvent::{self, *},
        run::run_line,
        Engine,
    },
    notation::{usi, Notation},
    protocol::{
        types::{EngineInfo, EngineOption, SearchLimits, SetOptionError},
        usi::Usi,
    },
    shogi::{
        movegen::{is_legal, is_pseudo_legal},
        position::{mv::Move, Position},
    },
};

use crate::MATSURI_SFEN;

#[test]
fn generate_events() {
    let mut engine = MockEngine::default();
    let mut events = vec![];

    let commands = [
        "usi",
        "isready",
        "usinewgame",
        "position startpos",
        "go",
        "position startpos moves 2g2f",
        "go",
        &format!("position sfen {MATSURI_SFEN}"),
        "go",
        &format!("position sfen {MATSURI_SFEN} moves G*3c"),
        "go",
    ];

    for command in commands {
        match run_line::<Usi, _, _>(&mut engine, command, |event| {
            events.push(event);
        }) {
            Ok(quit) => {
                assert!(!quit);
            }
            Err(err) => panic!("{:?}", err),
        }
    }

    match run_line::<Usi, _, _>(&mut engine, "quit", |event| {
        events.push(event);
    }) {
        Ok(quit) => {
            assert!(quit);
        }
        Err(err) => panic!("{:?}", err),
    }

    assert_eq!(
        events,
        vec![
            Raw("id name mock
id author engine"
                .into()),
            Raw("option name USI_Hash type spin default 64 min 1 max 131072
option name Threads type spin default 1 min 1 max 2048
option name USI_MultiPV type spin default 1 min 1 max 600
option name Minimal type check default false"
                .into()),
            Raw("usiok".into()),
            Raw("readyok".into()),
            Info("Initializing...".into()),
            BestMove(usi::Usi::parse_move("2g2f").unwrap()),
            BestMove(usi::Usi::parse_move("8c8d").unwrap()),
            BestMove(usi::Usi::parse_move("G*3c").unwrap()),
            BestMove(usi::Usi::parse_move("4b3b").unwrap()),
            Info("Finalizing...".into()),
        ]
    );
}

pub struct MockEngine {
    current_position: Position,
    options: [(String, EngineOption); 4],
}

impl Default for MockEngine {
    fn default() -> Self {
        Self {
            current_position: Position::startpos(),
            options: [
                ("USI_Hash".into(), EngineOption::int_range(64, 1, 131072)),
                ("Threads".into(), EngineOption::int_range(1, 1, 2048)),
                ("USI_MultiPV".into(), EngineOption::int_range(1, 1, 600)),
                ("Minimal".into(), EngineOption::bool(false)),
            ],
        }
    }
}

impl Engine for MockEngine {
    fn engine_info(&self) -> EngineInfo {
        EngineInfo::new("mock", "engine")
    }

    fn engine_options(&self) -> &[(String, EngineOption)] {
        &self.options
    }

    fn set_option(&mut self, _name: &str, _value: &str) -> Result<(), SetOptionError> {
        Ok(())
    }

    fn initialize<F: FnMut(EngineEvent)>(&mut self, mut handler: F) {
        handler(Info("Initializing...".into()))
    }

    fn finalize<F: FnMut(EngineEvent)>(&mut self, mut handler: F) {
        handler(Info("Finalizing...".into()))
    }

    fn set_position(&mut self, startpos: &Position, moves: &[Move]) -> Result<(), Move> {
        self.current_position = startpos.clone();

        for mv in moves {
            if !is_pseudo_legal(&self.current_position, *mv)
                || !is_legal(&mut self.current_position, *mv)
            {
                return Err(*mv);
            }

            self.current_position.make_move(*mv);
        }

        Ok(())
    }

    fn search<F: FnMut(EngineEvent)>(&mut self, _limits: SearchLimits, mut handler: F) {
        match usi::Usi::format_position(&self.current_position).as_str() {
            "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1" => {
                handler(BestMove(usi::Usi::parse_move("2g2f").unwrap()))
            }
            "lnsgkgsnl/1r5b1/ppppppppp/9/9/7P1/PPPPPPP1P/1B5R1/LNSGKGSNL w - 2" => {
                handler(BestMove(usi::Usi::parse_move("8c8d").unwrap()))
            }
            "l6nl/5+P1gk/2np1S3/p1p4Pp/3P2Sp1/1PPb2P1P/P5GS1/R8/LN4bKL w RGgsn5p 1" => {
                handler(BestMove(usi::Usi::parse_move("G*3c").unwrap()))
            }
            "l6nl/5+P1gk/2np1Sg2/p1p4Pp/3P2Sp1/1PPb2P1P/P5GS1/R8/LN4bKL b RGsn5p 2" => {
                handler(BestMove(usi::Usi::parse_move("4b3b").unwrap()))
            }
            _ => handler(Resign),
        };
    }

    fn mate_search<F: FnMut(EngineEvent)>(&mut self, _limits: SearchLimits, _handler: F) {
        unimplemented!()
    }

    fn stop(&mut self) {
        unimplemented!()
    }
}
