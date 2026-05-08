use crate::{
    engine::{event::EngineEvent, Engine},
    notation::Notation,
    protocol::{types::EngineCommand::*, Protocol},
};

/// Parses and executes a single protocol input line.
///
/// The line is parsed using the specified protocol implementation,
/// then translated into engine operations.
///
/// Engine output and status notifications are emitted through `handler`.
///
/// Returns:
/// - `Ok(true)` if a quit command was received.
/// - `Ok(false)` if processing completed normally.
/// - `Err(...)` if parsing failed.
pub fn run_line<P: Protocol, E: Engine, F: FnMut(EngineEvent)>(
    engine: &mut E,
    line: &str,
    mut handler: F,
) -> Result<bool, P::ParseError> {
    let commands = P::parse_line(line)?;

    for command in commands {
        match command {
            ShowEngineInfo => handler(EngineEvent::Raw(P::format_engine_info(
                &engine.engine_info(),
            ))),
            ShowOptions => handler(EngineEvent::Raw(
                engine
                    .engine_options()
                    .iter()
                    .map(|(name, option)| P::format_option(name, option))
                    .collect::<Vec<_>>()
                    .join("\n"),
            )),
            ShowString(s) => handler(EngineEvent::Raw(s)),
            SetOption { name, value } => match engine.set_option(&name, &value) {
                Ok(_) => {}
                Err(e) => handler(EngineEvent::Warning(format!(
                    "set option failed: name={name} value={value}, error={e:?}",
                ))),
            },
            Initialize => engine.initialize(&mut handler),
            SetPosition { startpos, moves } => match engine.set_position(&startpos, &moves) {
                Ok(_) => {}
                Err(mv) => handler(EngineEvent::Warning(format!(
                    "invalid move: {}",
                    P::Notation::format_move(mv)
                ))),
            },
            StartSearching { limits } => engine.search(limits, &mut handler),
            StartMateSearching { limits } => engine.mate_search(limits, &mut handler),
            StopSearching => engine.stop(),
            Quit => {
                engine.finalize(&mut handler);
                return Ok(true);
            }
        }
    }

    Ok(false)
}
