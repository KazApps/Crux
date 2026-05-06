use crate::protocol::types::{EngineCommand, EngineInfo, EngineOption};

pub mod types;
pub mod usi;

/// Defines an interface for engine communication protocols.
///
/// A protocol is responsible for:
/// - Parsing incoming text commands into structured `EngineCommand`s
/// - Formatting engine responses into protocol-compliant strings
///
/// This abstraction allows supporting multiple protocols
/// without coupling them to the engine core.
pub trait Protocol {
    type ParseError;

    /// Parses a single line of protocol input into one or more `EngineCommand`s.
    ///
    /// A single input line may produce multiple commands
    /// (e.g. `usi` expands into several initialization steps).
    ///
    /// Returns a protocol-specific parse error if the input is invalid.
    fn parse_line(line: &str) -> Result<Vec<EngineCommand>, Self::ParseError>;

    /// Formats engine identification information according to the protocol.
    fn format_engine_info(info: &EngineInfo) -> String;

    /// Formats all available engine options into protocol-compliant output.
    fn format_options(options: &[EngineOption]) -> String;
}
