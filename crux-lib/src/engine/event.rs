use crate::shogi::position::mv::Move;

/// Events emitted by an engine during operation.
///
/// These events are typically forwarded to a protocol layer
/// and formatted into protocol-specific output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EngineEvent {
    /// Reports the best move found by the search.
    BestMove(Move),

    /// Indicates that the engine resigns the game.
    Resign,

    /// Emits informational text intended for the user or GUI.
    Info(String),

    /// Emits raw protocol-specific output.
    ///
    /// This by passes structured event formatting.
    Raw(String),

    /// Emits a non-fatal warning message.
    Warning(String),

    /// Emits an error message.
    Error(String),
}
