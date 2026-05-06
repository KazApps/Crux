use std::time::Duration;

use crate::{
    notation::{
        usi::{self, ParseMoveError, ParsePositionError},
        Notation,
    },
    protocol::{
        types::{EngineCommand, EngineInfo, EngineOption, OptionKind, Overtime, SearchLimits},
        Protocol,
    },
    shogi::position::Position,
};

pub struct Usi;

impl Protocol for Usi {
    type ParseError = ParseError;

    fn parse_line(line: &str) -> Result<Vec<EngineCommand>, Self::ParseError> {
        use EngineCommand::*;

        let parse_limits = |mut args: &[&str]| {
            let mut limits = SearchLimits::default();

            while !args.is_empty() {
                match args {
                    ["searchmoves", rest @ ..] => {
                        limits.moves = Some(vec![]);

                        for token in rest {
                            match usi::Usi::parse_move(token) {
                                Ok(mv) => limits.moves.as_mut().unwrap().push(mv),
                                Err(_) => break,
                            }
                        }

                        args = &args[limits.moves.as_mut().unwrap().len() + 1..];
                    }
                    ["ponder", ..] => unimplemented!(), // args = &args[1..],
                    ["btime" | "wtime" | "binc" | "winc" | "movetime" | "byoyomi", ms, ..] => {
                        let ms = ms
                            .parse::<u64>()
                            .map_err(|_| ParseGoArgsError::InvalidValue)?;
                        let ms = Duration::from_millis(ms);
                        let time = limits.time.get_or_insert_default();

                        match args[0] {
                            "btime" => time.black.base = ms,
                            "wtime" => time.white.base = ms,
                            "binc" => time.black.overtime = Overtime::Increment(ms),
                            "winc" => time.white.overtime = Overtime::Increment(ms),
                            "movetime" => {
                                *time = Default::default();
                                time.black.overtime = Overtime::Byoyomi(ms);
                                time.white.overtime = Overtime::Byoyomi(ms);
                            }
                            "byoyomi" => {
                                time.black.overtime = Overtime::Byoyomi(ms);
                                time.white.overtime = Overtime::Byoyomi(ms);
                            }
                            _ => unreachable!(),
                        }

                        args = &args[2..];
                    }
                    ["depth", d] => {
                        limits.depth = Some(d.parse().map_err(|_| ParseGoArgsError::InvalidValue)?);
                        args = &args[2..];
                    }
                    ["nodes", n] => {
                        limits.nodes = Some(n.parse().map_err(|_| ParseGoArgsError::InvalidValue)?);
                        args = &args[2..];
                    }
                    _ => return Err(ParseGoArgsError::UnknownArgumentOrMissingValue),
                }
            }

            Ok(limits)
        };

        match line.split_whitespace().collect::<Vec<_>>().as_slice() {
            ["usi"] => Ok(vec![
                ShowEngineInfo,
                ShowOptions,
                ShowString("usiok".into()),
            ]),
            ["isready"] => Ok(vec![ShowString("readyok".into())]),
            ["setoption", "name", name, "value", value] => Ok(vec![SetOption {
                name: (*name).into(),
                value: (*value).into(),
            }]),
            ["usinewgame"] => Ok(vec![Initialize]),
            ["position", "sfen", rest @ ..] if rest.contains(&"moves") => {
                let moves_idx = rest.iter().position(|&t| t == "moves").unwrap();
                let sfen = &rest[..moves_idx].join(" ");
                let moves = &rest[moves_idx + 1..];

                Ok(vec![SetPosition {
                    startpos: usi::Usi::parse_position(sfen)
                        .map_err(ParseError::InvalidPosition)?,
                    moves: moves
                        .iter()
                        .map(|mv| usi::Usi::parse_move(mv).map_err(ParseError::InvalidMove))
                        .collect::<Result<_, _>>()?,
                }])
            }
            ["position", "sfen", sfen @ ..] => Ok(vec![SetPosition {
                startpos: usi::Usi::parse_position(sfen.join(" ").as_str())
                    .map_err(ParseError::InvalidPosition)?,
                moves: vec![],
            }]),
            ["position", "startpos"] => Ok(vec![SetPosition {
                startpos: Position::startpos(),
                moves: vec![],
            }]),
            ["position", "startpos", "moves", moves @ ..] => Ok(vec![SetPosition {
                startpos: Position::startpos(),
                moves: moves
                    .iter()
                    .map(|mv| usi::Usi::parse_move(mv).map_err(ParseError::InvalidMove))
                    .collect::<Result<_, _>>()?,
            }]),
            ["go", "mate", n, args @ ..] => {
                Ok(vec![StartMateSearching {
                    limits: SearchLimits {
                        depth: Some(n.parse().map_err(|_| {
                            ParseError::InvalidGoArgs(ParseGoArgsError::InvalidValue)
                        })?),
                        ..parse_limits(args).map_err(ParseError::InvalidGoArgs)?
                    },
                }])
            }
            ["go", "infinite", args @ ..] => Ok(vec![StartSearching {
                limits: SearchLimits {
                    moves: parse_limits(args).map_err(ParseError::InvalidGoArgs)?.moves,
                    ..Default::default()
                },
            }]),
            ["go", args @ ..] => Ok(vec![StartSearching {
                limits: parse_limits(args).map_err(ParseError::InvalidGoArgs)?,
            }]),
            ["stop"] => Ok(vec![StopSearching]),
            ["quit"] => Ok(vec![Quit]),
            _ => Err(ParseError::UnknownCommand),
        }
    }

    fn format_engine_info(info: &EngineInfo) -> String {
        format!("id name {}\nid author {}", info.name, info.author)
    }

    fn format_options(options: &[EngineOption]) -> String {
        options
            .iter()
            .map(|option| match option.kind {
                OptionKind::Check { default, .. } => {
                    format!("option name {} type check default {}", option.name, default)
                }
                OptionKind::Spin {
                    min, max, default, ..
                } => {
                    format!(
                        "option name {} type spin default {} min {} max {}",
                        option.name, default, min, max
                    )
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ParseError {
    InvalidPosition(ParsePositionError),
    InvalidMove(ParseMoveError),
    InvalidGoArgs(ParseGoArgsError),
    UnknownCommand,
}

#[derive(Debug, Copy, Clone)]
pub enum ParseGoArgsError {
    InvalidValue,
    UnknownArgumentOrMissingValue,
}
