use std::time::Duration;

use crate::shogi::position::{mv::Move, Position};

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum EngineCommand {
    ShowEngineInfo,
    ShowOptions,
    ShowString(String),
    SetOption {
        name: String,
        value: String,
    },
    Initialize,
    SetPosition {
        startpos: Position,
        moves: Vec<Move>,
    },
    StartSearching {
        limits: SearchLimits,
    },
    StartMateSearching {
        limits: SearchLimits,
    },
    StopSearching,
    Quit,
}

#[derive(Default, Debug, Clone)]
pub struct SearchLimits {
    pub time: Option<TimeControl>,
    pub nodes: Option<u64>,
    pub depth: Option<u32>,
    pub moves: Option<Vec<Move>>,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct TimeControl {
    pub black: PlayerTime,
    pub white: PlayerTime,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct PlayerTime {
    pub base: Duration,
    pub overtime: Overtime,
}

#[derive(Debug, Copy, Clone)]
pub enum Overtime {
    Increment(Duration),
    Byoyomi(Duration),
}

impl Default for Overtime {
    fn default() -> Self {
        Self::Increment(Default::default())
    }
}

#[derive(Debug, Clone)]
pub struct EngineInfo {
    pub name: String,
    pub author: String,
}

#[derive(Debug, Copy, Clone)]
pub enum OptionKind {
    Check {
        default: bool,
        value: bool,
    },
    Spin {
        default: i64,
        value: i64,
        min: i64,
        max: i64,
    },
}

#[derive(Debug, Clone)]
pub struct EngineOption {
    pub name: String,
    pub kind: OptionKind,
}

impl EngineOption {
    pub fn check(name: impl Into<String>, default: bool) -> Self {
        Self {
            name: name.into(),
            kind: OptionKind::Check {
                default,
                value: default,
            },
        }
    }

    pub fn spin(name: impl Into<String>, default: i64, min: i64, max: i64) -> Self {
        Self {
            name: name.into(),
            kind: OptionKind::Spin {
                default,
                value: default,
                min,
                max,
            },
        }
    }

    pub fn set_from_str(&mut self, s: &str) -> Result<(), String> {
        match &mut self.kind {
            OptionKind::Check { value, .. } => {
                let v = s
                    .parse::<bool>()
                    .map_err(|e| format!("Invalid value for check option: {e}"))?;
                *value = v;
            }
            OptionKind::Spin {
                value, min, max, ..
            } => {
                let v = s
                    .parse::<i64>()
                    .map_err(|e| format!("Invalid value for spin option: {e}"))?;

                if !(*min..=*max).contains(&v) {
                    return Err(format!(
                        "Invalid value for spin option: {v} is out of range ({min}..={max})"
                    ));
                }

                *value = v;
            }
        }

        Ok(())
    }
}
