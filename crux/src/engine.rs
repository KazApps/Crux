use crux_lib::{
    engine::{event::EngineEvent, Engine},
    protocol::types::{EngineInfo, EngineOption, SearchLimits, SetOptionError},
    shogi::{
        movegen::{generate, is_legal, is_pseudo_legal},
        position::{mv::Move, Position},
    },
};

pub struct Crux {
    current_position: Position,
}

impl Default for Crux {
    fn default() -> Self {
        Self {
            current_position: Position::startpos(),
        }
    }
}

impl Engine for Crux {
    fn engine_info(&self) -> EngineInfo {
        EngineInfo::new("Crux", "Kazuki Yamashita and m5t0")
    }

    fn engine_options(&self) -> &[(String, EngineOption)] {
        &[]
    }

    fn set_option(&mut self, _name: &str, _value: &str) -> Result<(), SetOptionError> {
        Ok(())
    }

    fn initialize<F: FnMut(EngineEvent)>(&mut self, _handler: F) {
        self.current_position = Position::startpos();
    }

    fn finalize<F: FnMut(EngineEvent)>(&mut self, _handler: F) {}

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
        let moves = generate(&self.current_position);
        let moves = moves
            .iter()
            .filter(|&mv| is_legal(&mut self.current_position, *mv))
            .collect::<Vec<_>>();
        if moves.is_empty() {
            handler(EngineEvent::Resign);
        } else {
            let best_move = *moves[self.current_position.key().value() as usize % moves.len()];

            handler(EngineEvent::BestMove(best_move));
        }
    }

    fn mate_search<F: FnMut(EngineEvent)>(&mut self, _limits: SearchLimits, _handler: F) {
        todo!()
    }

    fn stop(&mut self) {
        todo!()
    }
}
