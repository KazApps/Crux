use crate::protocol::types::{EngineCommand, EngineInfo, EngineOption};

pub mod types;
pub mod usi;

pub trait Protocol {
    type ParseError;

    fn parse_line(line: &str) -> Result<Vec<EngineCommand>, Self::ParseError>;

    fn format_engine_info(info: &EngineInfo) -> String;
    fn format_options(options: &[EngineOption]) -> String;
}
