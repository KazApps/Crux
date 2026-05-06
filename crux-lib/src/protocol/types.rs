use std::time::Duration;

use crate::shogi::position::{mv::Move, Position};

/// Represents a command sent from the GUI to the engine
/// in a protocol-agnostic form.
///
/// These are typically parsed from protocol-specific input
/// and consumed by the engine core.
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

/// Represents search constraints.
///
/// All fields are optional and be combined.
#[derive(Default, Debug, Clone)]
pub struct SearchLimits {
    /// Time control information
    pub time: Option<TimeControl>,

    /// Maximum search depth
    pub depth: Option<u32>,

    /// Maximum number of nodes to search
    pub nodes: Option<u64>,

    /// Restrict search to a subset of moves
    pub moves: Option<Vec<Move>>,
}

/// Represents time control settings for both players.
///
/// Used in search commands to specify remaining time
/// and increment/byoyomi settings.
#[derive(Default, Debug, Copy, Clone)]
pub struct TimeControl {
    pub black: PlayerTime,
    pub white: PlayerTime,
}

/// Represents time information for a single player.
///
/// - `base` is the remaining main time
/// - `overtime` defines how extra time is handled
#[derive(Default, Debug, Copy, Clone)]
pub struct PlayerTime {
    pub base: Duration,
    pub overtime: Overtime,
}

/// Represents overtime rules in time control.
///
/// - `Increment`: Fischer-style increment per move
/// - `Byoyomi`: fixed time per move after main time runs out
#[derive(Debug, Copy, Clone)]
pub enum Overtime {
    Increment(Duration),
    Byoyomi(Duration),
}

/// Defaults to zero increment (no effective overtime).
impl Default for Overtime {
    fn default() -> Self {
        Self::Increment(Default::default())
    }
}

/// Basic engine identification information.
#[derive(Debug, Clone)]
pub struct EngineInfo {
    pub name: String,
    pub author: String,
}

/// A configurable engine option exposed to the GUI.
///
/// Combines a name with its type and current value.
#[derive(Debug, Clone)]
pub enum EngineOption {
    Bool {
        name: String,
        default: bool,
        value: bool,
    },
    IntRange {
        name: String,
        default: i64,
        value: i64,
        min: i64,
        max: i64,
    },
}

impl EngineOption {
    /// Creates a boolean option.
    pub fn bool(name: impl Into<String>, default: bool) -> Self {
        Self::Bool {
            name: name.into(),
            default,
            value: default,
        }
    }

    /// Creates an integer option with bounds.
    pub fn int_range(name: impl Into<String>, default: i64, min: i64, max: i64) -> Self {
        Self::IntRange {
            name: name.into(),
            default,
            value: default,
            min,
            max,
        }
    }

    /// Returns the name of this option.
    pub fn name(&self) -> &str {
        match self {
            Self::Bool { name, .. } => name,
            Self::IntRange { name, .. } => name,
        }
    }

    /// Updates the option value from a string representation.
    ///
    /// Returns an error if parsing fails or the value is out of range.
    pub fn set_from_str(&mut self, s: &str) -> Result<(), String> {
        match self {
            Self::Bool { value, .. } => {
                let v = s
                    .parse::<bool>()
                    .map_err(|e| format!("Invalid boolean value: {e}"))?;
                *value = v;
            }
            Self::IntRange {
                value, min, max, ..
            } => {
                let v = s
                    .parse::<i64>()
                    .map_err(|e| format!("Invalid integer value: {e}"))?;

                if !(*min..=*max).contains(&v) {
                    return Err(format!(
                        "Invalid integer value: {v} is out of range ({min}..={max})"
                    ));
                }

                *value = v;
            }
        }

        Ok(())
    }
}
