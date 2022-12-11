use chess::{Action, ChessMove, Color, Game, GameResult, MoveGen};
use mcts::GameState;
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct GameWrapper(pub Game);

impl Default for GameWrapper {
    fn default() -> Self {
        Self(Game::new())
    }
}

impl Hash for GameWrapper {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let mut count = 0;
        for action in self.0.actions() {
            if let Action::MakeMove(m) = action {
                state.write_u8(m.get_source().to_int());
                state.write_u8(m.get_dest().to_int());
                state.write_usize(m.get_promotion().map(|p| p.to_index()).unwrap_or(100));
                count += 1;
            }
        }
        state.write_usize(count);
        state.write_u64(self.0.current_position().get_hash());
    }
}

impl GameState for GameWrapper {
    type Move = ChessMove;
    type Player = chess::Color;
    type MoveList = Vec<ChessMove>;

    fn current_player(&self) -> Self::Player {
        self.0.side_to_move()
    }

    fn available_moves(&self) -> Self::MoveList {
        MoveGen::new_legal(&self.0.current_position()).collect()
    }

    fn make_move(&mut self, mov: &Self::Move) {
        self.0.make_move(mov.clone());
    }

    fn get_winner(&self) -> Option<Self::Player> {
        let result = self.0.result();

        match result {
            Some(GameResult::WhiteCheckmates) => Some(Color::White),
            Some(GameResult::WhiteResigns) => Some(Color::Black),
            Some(GameResult::BlackCheckmates) => Some(Color::Black),
            Some(GameResult::BlackResigns) => Some(Color::White),
            _ => None,
        }
    }

    fn is_terminal(&self) -> bool {
        self.0
            .result()
            .map(|result| match result {
                GameResult::WhiteCheckmates => true,
                GameResult::WhiteResigns => true,
                GameResult::BlackCheckmates => true,
                GameResult::BlackResigns => true,
                GameResult::Stalemate => true,
                GameResult::DrawAccepted => true,
                GameResult::DrawDeclared => false,
            })
            .unwrap_or(false)
    }
}
