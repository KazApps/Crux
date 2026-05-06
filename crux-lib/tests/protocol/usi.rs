use std::time::Duration;

use crux_lib::{
    notation::{self, Notation},
    protocol::{
        types::{EngineCommand::*, Overtime, PlayerTime, SearchLimits, TimeControl},
        usi::Usi,
        Protocol,
    },
    shogi::{
        core::{PieceType, Square},
        position::{mv::Move, Position},
    },
};

use super::super::MATSURI_SFEN;

#[test]
fn parse_usi() {
    let commands = Usi::parse_line("usi").unwrap();

    assert!(
        matches!(commands.as_slice(), [ShowEngineInfo, ShowOptions, ShowString(s)] if s == "usiok")
    );
}

#[test]
fn parse_isready() {
    let commands = Usi::parse_line("isready").unwrap();

    assert!(matches!(commands.as_slice(), [ShowString(s)] if s == "readyok"));
}

#[test]
fn parse_setoption() {
    let commands = Usi::parse_line("setoption name Threads value 16").unwrap();

    assert!(
        matches!(commands.as_slice(), [SetOption { name, value }] if name == "Threads" && value == "16")
    );
}

#[test]
fn parse_usinewgame() {
    let commands = Usi::parse_line("usinewgame").unwrap();

    assert!(matches!(commands.as_slice(), [Initialize]));
}

#[test]
fn parse_position_startpos() {
    let commands = Usi::parse_line("position startpos").unwrap();

    assert!(
        matches!(commands.as_slice(), [SetPosition { startpos, moves }] if {
            assert_eq!(startpos.key(), Position::startpos().key());
            assert!(moves.is_empty());

            true
        })
    );
}

#[test]
fn parse_position_startpos_with_moves() {
    let commands = Usi::parse_line("position startpos moves 2g2f 8c8d").unwrap();

    assert!(
        matches!(commands.as_slice(), [SetPosition { startpos, moves }] if {
            assert_eq!(startpos.key(), Position::startpos().key());

            assert_eq!(moves[0].from(), Square::S27);
            assert_eq!(moves[0].to(), Square::S26);
            assert_eq!(moves[1].from(), Square::S83);
            assert_eq!(moves[1].to(), Square::S84);

            true
        })
    );
}

#[test]
fn parse_position_sfen() {
    let commands = Usi::parse_line(&format!("position sfen {MATSURI_SFEN}")).unwrap();

    assert!(
        matches!(commands.as_slice(), [SetPosition { startpos, moves }] if {
            assert_eq!(
                startpos.key(),
                notation::usi::Usi::parse_position(MATSURI_SFEN)
                    .unwrap()
                    .key()
            );
            assert!(moves.is_empty());

            true
        })
    );
}

#[test]
fn parse_position_sfen_with_moves() {
    let commands =
        Usi::parse_line(&format!("position sfen {MATSURI_SFEN} moves G*3c 4b3b")).unwrap();

    assert!(
        matches!(commands.as_slice(), [SetPosition { startpos, moves }] if {
            assert_eq!(
                startpos.key(),
                notation::usi::Usi::parse_position(MATSURI_SFEN)
                    .unwrap()
                    .key()
            );

            assert_eq!(moves[0].drop_piece_type(), PieceType::Gold);
            assert_eq!(moves[0].to(), Square::S33);
            assert_eq!(moves[1].from(), Square::S42);
            assert_eq!(moves[1].to(), Square::S32);

            true
        })
    );
}

#[test]
fn parse_go() {
    let commands = Usi::parse_line("go").unwrap();

    assert!(
        matches!(commands.as_slice(), [StartSearching { limits }] if *limits == SearchLimits::default())
    );
}

#[test]
fn parse_go_with_depth() {
    let commands = Usi::parse_line("go depth 10").unwrap();

    assert!(
        matches!(commands.as_slice(), [StartSearching { limits }] if *limits == SearchLimits{ depth: Some(10), ..Default::default() })
    );
}

#[test]
fn parse_go_with_nodes() {
    let commands = Usi::parse_line("go nodes 1000000").unwrap();

    assert!(
        matches!(commands.as_slice(), [StartSearching { limits }] if *limits == SearchLimits{ nodes: Some(1000000), ..Default::default() })
    );
}

#[test]
fn parse_go_with_moves() {
    let commands = Usi::parse_line("go searchmoves 2g2f 7g7f").unwrap();

    assert!(
        matches!(commands.as_slice(), [StartSearching { limits }] if *limits == SearchLimits {
            moves: Some(vec![Move::normal(Square::S27, Square::S26), Move::normal(Square::S77, Square::S76)]),
            ..Default::default()
        })
    );
}

#[test]
fn parse_go_with_btime() {
    let commands = Usi::parse_line("go btime 100").unwrap();

    assert!(matches!(
        commands.as_slice(), [StartSearching { limits }]
        if *limits == SearchLimits {
            time: Some(
                TimeControl {
                    black: PlayerTime {
                        base: Duration::from_millis(100),
                        overtime: Default::default()
                    },
                    white: Default::default()
                }
            ),
            ..Default::default()
        }
    ));
}

#[test]
fn parse_go_with_wtime() {
    let commands = Usi::parse_line("go wtime 100").unwrap();

    assert!(matches!(
        commands.as_slice(), [StartSearching { limits }]
        if *limits == SearchLimits {
            time: Some(
                TimeControl {
                    black: Default::default(),
                    white: PlayerTime {
                        base: Duration::from_millis(100),
                        overtime: Default::default()
                    }
                }
            ),
            ..Default::default()
        }
    ));
}

#[test]
fn parse_go_with_binc() {
    let commands = Usi::parse_line("go binc 100").unwrap();

    assert!(matches!(
        commands.as_slice(), [StartSearching { limits }]
        if *limits == SearchLimits {
            time: Some(
                TimeControl {
                    black: PlayerTime {
                        base: Default::default(),
                        overtime: Overtime::Increment(Duration::from_millis(100)),
                    },
                    white: Default::default()
                }
            ),
            ..Default::default()
        }
    ));
}

#[test]
fn parse_go_with_winc() {
    let commands = Usi::parse_line("go winc 100").unwrap();

    assert!(matches!(
        commands.as_slice(), [StartSearching { limits }]
        if *limits == SearchLimits {
            time: Some(
                TimeControl {
                    black: Default::default(),
                    white: PlayerTime {
                        base: Default::default(),
                        overtime: Overtime::Increment(Duration::from_millis(100)),
                    }
                }
            ),
            ..Default::default()
        }
    ));
}

#[test]
fn parse_go_with_byoyomi() {
    let commands = Usi::parse_line("go byoyomi 100").unwrap();

    assert!(matches!(
        commands.as_slice(), [StartSearching { limits }]
        if *limits == SearchLimits {
            time: Some(
                TimeControl {
                    black: PlayerTime {
                        base: Default::default(),
                        overtime: Overtime::Byoyomi(Duration::from_millis(100)),
                    },
                    white: PlayerTime {
                        base: Default::default(),
                        overtime: Overtime::Byoyomi(Duration::from_millis(100)),
                    }
                }
            ),
            ..Default::default()
        }
    ));
}

#[test]
fn parse_go_with_movetime() {
    let commands = Usi::parse_line("go movetime 100").unwrap();

    assert!(matches!(
        commands.as_slice(), [StartSearching { limits }]
        if *limits == SearchLimits {
            time: Some(
                TimeControl {
                    black: PlayerTime {
                        base: Default::default(),
                        overtime: Overtime::Byoyomi(Duration::from_millis(100)),
                    },
                    white: PlayerTime {
                        base: Default::default(),
                        overtime: Overtime::Byoyomi(Duration::from_millis(100)),
                    }
                }
            ),
            ..Default::default()
        }
    ));
}

#[test]
fn parse_go_with_all_limits() {
    let commands = Usi::parse_line(
        "go searchmoves 2g2f 7g7f btime 1000 wtime 2000 binc 100 winc 100 depth 15 nodes 1000000",
    )
    .unwrap();

    assert!(matches!(
        commands.as_slice(), [StartSearching { limits }]
        if *limits == SearchLimits {
            time: Some(
                TimeControl {
                    black: PlayerTime {
                        base: Duration::from_millis(1000),
                        overtime: Overtime::Increment(Duration::from_millis(100)),
                    },
                    white: PlayerTime {
                        base: Duration::from_millis(2000),
                        overtime: Overtime::Increment(Duration::from_millis(100)),
                    }
                }
            ),
            depth: Some(15),
            nodes: Some(1000000),
            moves: Some(vec![Move::normal(Square::S27, Square::S26), Move::normal(Square::S77, Square::S76)]),
        }
    ));
}

#[test]
fn parse_go_infinite() {
    let commands = Usi::parse_line("go infinite").unwrap();

    assert!(
        matches!(commands.as_slice(), [StartSearching { limits }] if *limits == SearchLimits::default())
    );
}

#[test]
fn parse_go_infinite_with_moves() {
    let commands = Usi::parse_line("go infinite searchmoves 2g2f 7g7f").unwrap();

    assert!(
        matches!(commands.as_slice(), [StartSearching { limits }] if *limits == SearchLimits {
            moves: Some(vec![Move::normal(Square::S27, Square::S26), Move::normal(Square::S77, Square::S76)]),
            ..Default::default()
        })
    );
}

#[test]
fn parse_go_mate() {
    let commands = Usi::parse_line("go mate 10").unwrap();

    assert!(
        matches!(commands.as_slice(), [StartMateSearching { limits }] if *limits == SearchLimits {
            depth: Some(10),
            ..Default::default()
        })
    );
}

#[test]
fn parse_go_mate_with_nodes() {
    let commands = Usi::parse_line("go mate 10 nodes 1000000").unwrap();

    assert!(
        matches!(commands.as_slice(), [StartMateSearching { limits }] if *limits == SearchLimits {
            depth: Some(10),
            nodes: Some(1000000),
            ..Default::default()
        })
    );
}

#[test]
fn parse_go_mate_with_moves() {
    let commands = Usi::parse_line("go mate 10 searchmoves 2g2f 7g7f").unwrap();

    assert!(
        matches!(commands.as_slice(), [StartMateSearching { limits }] if *limits == SearchLimits {
            depth: Some(10),
            moves: Some(vec![Move::normal(Square::S27, Square::S26), Move::normal(Square::S77, Square::S76)]),
            ..Default::default()
        })
    );
}

#[test]
fn parse_go_mate_with_btime() {
    let commands = Usi::parse_line("go mate 10 btime 100").unwrap();

    assert!(matches!(
        commands.as_slice(), [StartMateSearching { limits }]
        if *limits == SearchLimits {
            depth: Some(10),
            time: Some(
                TimeControl {
                    black: PlayerTime {
                        base: Duration::from_millis(100),
                        overtime: Default::default()
                    },
                    white: Default::default()
                }
            ),
            ..Default::default()
        }
    ));
}

#[test]
fn parse_go_mate_with_wtime() {
    let commands = Usi::parse_line("go mate 10 wtime 100").unwrap();

    assert!(matches!(
        commands.as_slice(), [StartMateSearching { limits }]
        if *limits == SearchLimits {
            depth: Some(10),
            time: Some(
                TimeControl {
                    black: Default::default(),
                    white: PlayerTime {
                        base: Duration::from_millis(100),
                        overtime: Default::default()
                    }
                }
            ),
            ..Default::default()
        }
    ));
}

#[test]
fn parse_go_mate_with_binc() {
    let commands = Usi::parse_line("go mate 10 binc 100").unwrap();

    assert!(matches!(
        commands.as_slice(), [StartMateSearching { limits }]
        if *limits == SearchLimits {
            depth: Some(10),
            time: Some(
                TimeControl {
                    black: PlayerTime {
                        base: Default::default(),
                        overtime: Overtime::Increment(Duration::from_millis(100)),
                    },
                    white: Default::default()
                }
            ),
            ..Default::default()
        }
    ));
}

#[test]
fn parse_go_mate_with_winc() {
    let commands = Usi::parse_line("go mate 10 winc 100").unwrap();

    assert!(matches!(
        commands.as_slice(), [StartMateSearching { limits }]
        if *limits == SearchLimits {
            depth: Some(10),
            time: Some(
                TimeControl {
                    black: Default::default(),
                    white: PlayerTime {
                        base: Default::default(),
                        overtime: Overtime::Increment(Duration::from_millis(100)),
                    }
                }
            ),
            ..Default::default()
        }
    ));
}

#[test]
fn parse_go_mate_with_byoyomi() {
    let commands = Usi::parse_line("go mate 10 byoyomi 100").unwrap();

    assert!(matches!(
        commands.as_slice(), [StartMateSearching { limits }]
        if *limits == SearchLimits {
            depth: Some(10),
            time: Some(
                TimeControl {
                    black: PlayerTime {
                        base: Default::default(),
                        overtime: Overtime::Byoyomi(Duration::from_millis(100)),
                    },
                    white: PlayerTime {
                        base: Default::default(),
                        overtime: Overtime::Byoyomi(Duration::from_millis(100)),
                    }
                }
            ),
            ..Default::default()
        }
    ));
}

#[test]
fn parse_go_mate_with_movetime() {
    let commands = Usi::parse_line("go mate 10 movetime 100").unwrap();

    assert!(matches!(
        commands.as_slice(), [StartMateSearching { limits }]
        if *limits == SearchLimits {
            depth: Some(10),
            time: Some(
                TimeControl {
                    black: PlayerTime {
                        base: Default::default(),
                        overtime: Overtime::Byoyomi(Duration::from_millis(100)),
                    },
                    white: PlayerTime {
                        base: Default::default(),
                        overtime: Overtime::Byoyomi(Duration::from_millis(100)),
                    }
                }
            ),
            ..Default::default()
        }
    ));
}

#[test]
fn parse_go_mate_with_all_limits() {
    let commands = Usi::parse_line(
        "go mate 10 searchmoves 2g2f 7g7f btime 1000 wtime 2000 binc 100 winc 100 nodes 1000000",
    )
    .unwrap();

    assert!(matches!(
        commands.as_slice(), [StartMateSearching { limits }]
        if *limits == SearchLimits {
            time: Some(
                TimeControl {
                    black: PlayerTime {
                        base: Duration::from_millis(1000),
                        overtime: Overtime::Increment(Duration::from_millis(100)),
                    },
                    white: PlayerTime {
                        base: Duration::from_millis(2000),
                        overtime: Overtime::Increment(Duration::from_millis(100)),
                    }
                }
            ),
            depth: Some(10),
            nodes: Some(1000000),
            moves: Some(vec![Move::normal(Square::S27, Square::S26), Move::normal(Square::S77, Square::S76)]),
        }
    ));
}

#[test]
fn parse_stop() {
    let commands = Usi::parse_line("stop").unwrap();

    assert!(matches!(commands.as_slice(), [StopSearching]));
}

#[test]
fn parse_quit() {
    let commands = Usi::parse_line("quit").unwrap();

    assert!(matches!(commands.as_slice(), [Quit]));
}
