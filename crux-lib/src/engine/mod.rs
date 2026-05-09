pub mod event;
pub mod run;

use crate::{
    engine::event::EngineEvent,
    protocol::types::{EngineInfo, EngineOption, SearchLimits, SetOptionError},
    shogi::position::{mv::Move, Position},
};

/// A generic shogi engine interface.
///
/// Engines implementing this trait can be controlled through
/// a protocol adapter such as USI.
pub trait Engine {
    /// Returns static information about the engine.
    ///
    /// This includes the engine name and author.
    #[must_use]
    fn engine_info(&self) -> EngineInfo;

    /// Returns all available engine options.
    #[must_use]
    fn engine_options(&self) -> &[(String, EngineOption)];

    /// Updates an engine option from its string representation.
    ///
    /// Returns an error if the option does not exists or the value is invalid.
    fn set_option(&mut self, name: &str, value: &str) -> Result<(), SetOptionError>;

    /// Initializes the engine for a new game.
    ///
    /// Engines may use this to clear transposition tables,
    /// reset internal state or load an NNUE network.
    fn initialize<F: FnMut(EngineEvent)>(&mut self, handler: F);

    /// Finalizes the engine before shutdown.
    ///
    /// Engines may use this to stop worker threads,
    /// flush logs or release external resources.
    fn finalize<F: FnMut(EngineEvent)>(&mut self, handler: F);

    /// Set the position from a start position and a sequence of moves.
    fn set_position(&mut self, startpos: &Position, moves: &[Move]) -> Result<(), Move>;

    /// Starts a normal search using the given search limits.
    ///
    /// Search progress and results are emitted through `handler`.
    fn search<F: FnMut(EngineEvent)>(&mut self, limits: SearchLimits, handler: F);

    /// Starts a mate search using the given search limits.
    ///
    /// Search progress and results are emitted through `handler`.
    fn mate_search<F: FnMut(EngineEvent)>(&mut self, limits: SearchLimits, handler: F);

    /// Requests the currently running search to stop.
    ///
    /// Engines should terminate the search as soon as practical.
    fn stop(&mut self);
}
