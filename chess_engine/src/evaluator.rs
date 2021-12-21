use chess::{Board, BoardStatus, ChessMove, Color, Game, MoveGen, Piece};
use mcts::{Evaluator, GameState};
use rand::{prelude::SliceRandom, Rng};

use crate::{mcts_handler::ChessMCTS, state::GameWrapper};

pub struct ChessEvaluator(pub Color);

pub const SCALE: f64 = 1e9;

#[derive(Debug, Clone, PartialEq)]
pub enum ChessEvaluation {
    Winner(Color),
    Draw,
    Evaluation(f64),
}

impl Evaluator<ChessMCTS> for ChessEvaluator {
    type StateEvaluation = ChessEvaluation;

    fn evaluate_new_state(
        &self,
        state: &GameWrapper,
        moves: &mcts::MoveList<ChessMCTS>,
        _: Option<mcts::SearchHandle<ChessMCTS>>,
    ) -> (Vec<mcts::MoveEvaluation<ChessMCTS>>, Self::StateEvaluation) {
        let evals = moves.iter().map(|_| ()).collect();

        if state.is_terminal() {
            let winner = state.get_winner();

            let points = match winner {
                Some(color) => ChessEvaluation::Winner(color),
                None => ChessEvaluation::Draw,
            };

            (evals, points)
        } else {
            (
                evals,
                evaluate_board(self.0, &state.0.current_position(), &moves),
                // evaluate_rollout(self.0, state.0.current_position(), &moves),
            )
        }
    }

    fn evaluate_existing_state(
        &self,
        state: &GameWrapper,
        existing_evaln: &Self::StateEvaluation,
        handle: mcts::SearchHandle<ChessMCTS>,
    ) -> Self::StateEvaluation {
        existing_evaln.clone()
    }

    fn interpret_evaluation_for_player(
        &self,
        evaluation: &Self::StateEvaluation,
        player: &mcts::Player<ChessMCTS>,
    ) -> f64 {
        // println!("player: {:?}, evaluation: {:?}", player,  evaluation);
        match evaluation {
            ChessEvaluation::Winner(winner) if winner == player => 100.0 * SCALE,
            ChessEvaluation::Winner(_) => -100.0 * SCALE,
            ChessEvaluation::Evaluation(e) => {
                if &self.0 == player {
                    *e * SCALE
                } else {
                    -e * SCALE
                }
            }
            ChessEvaluation::Draw => 0.0,
        }
    }
}

fn evaluate_board(turn: Color, board: &Board, moves: &Vec<ChessMove>) -> ChessEvaluation {
    let queens = score_piece(turn, board, Piece::Queen, 9.0);
    let rooks = score_piece(turn, board, Piece::Rook, 5.0);
    let bishops = score_piece(turn, board, Piece::Bishop, 3.0);
    let knights = score_piece(turn, board, Piece::Knight, 3.0);
    let pawns = score_piece(turn, board, Piece::Pawn, 1.0);

    let my_mobility = moves.len() as f64 * 0.1;
    let o_mobility = {
        if let Some(o) = board.clone().null_move() {
            MoveGen::new_legal(&o).count() as f64 * 0.1
        } else {
            0.0
        }
    };

    // println!("q: {}, r: {}, b: {}, k: {}, p: {}", queens, rooks, bishops, knights, pawns);

    ChessEvaluation::Evaluation(
        (queens + rooks + bishops + knights + pawns) + my_mobility - o_mobility,
    )
}

fn score_piece(turn: Color, board: &Board, piece: Piece, score: f64) -> f64 {
    board
        .pieces(piece)
        .filter_map(|s| {
            board
                .color_on(s)
                .map(|c| if c == turn { score } else { -score })
        })
        .sum()
}
