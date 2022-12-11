use std::ops::Not;

use chess::{Board, Color, Piece};
use mcts::{Evaluator, GameState};

use crate::{mcts_handler::ChessMCTS, state::GameWrapper};

pub struct ChessEvaluator(pub Color);

pub const SCALE: f64 = 1e7;

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
                // evaluate_board(self.0, &state.0.current_position(), &moves),
                ChessEvaluation::Evaluation(simplified_evaluation_function::board_value(
                    self.0,
                    &state.0.current_position(),
                    simplified_evaluation_function::GameTime::Middle,
                ) as f64), // evaluate_rollout(self.0, state.0.current_position(), &moves),
            )
        }
    }

    fn evaluate_existing_state(
        &self,
        _state: &GameWrapper,
        existing_evaln: &Self::StateEvaluation,
        _handle: mcts::SearchHandle<ChessMCTS>,
    ) -> Self::StateEvaluation {
        existing_evaln.clone()
    }

    fn interpret_evaluation_for_player(
        &self,
        evaluation: &Self::StateEvaluation,
        player: &mcts::Player<ChessMCTS>,
    ) -> f64 {
        // println!("player: {:?}, evaluation: {:?}", player,  evaluation);
        (match evaluation {
            ChessEvaluation::Winner(winner) if winner == player => 100000.0,
            ChessEvaluation::Winner(_) => -100000.0,
            ChessEvaluation::Evaluation(e) => {
                // println!("E: {}", e);
                // *e
                if &self.0 == player {
                    *e
                } else {
                    -*e
                }
            }
            ChessEvaluation::Draw => 0.0,
        } * SCALE)
    }
}

// fn evaluate_board(turn: Color, board: &Board, moves: &Vec<ChessMove>) -> ChessEvaluation {
//     // let queens = score_piece(turn, board, Piece::Queen, 9.0);
//     // let rooks = score_piece(turn, board, Piece::Rook, 5.0);
//     // let bishops = score_piece(turn, board, Piece::Bishop, 3.5);
//     // let knights = score_piece(turn, board, Piece::Knight, 3.0);
//     // let pawns = score_piece(turn, board, Piece::Pawn, 1.0);

//     // let my_mobility = moves.len() as f64 * 1.3;
//     // let o_mobility = {
//     //     if let Some(o) = board.clone().null_move() {
//     //         MoveGen::new_legal(&o).count() as f64 * 0.3
//     //     } else {
//     //         0.0
//     //     }
//     // };

//     // println!("q: {}, r: {}, b: {}, k: {}, p: {}", queens, rooks, bishops, knights, pawns);

//     // ChessEvaluation::Evaluation(
//     //     (queens + rooks + bishops + knights + pawns), // + my_mobility - o_mobility,
//     // )
// }

fn score_piece(turn: Color, board: &Board, piece: Piece, score: f64) -> f64 {
    let me = board.pieces(piece) & board.color_combined(turn);
    let them = board.pieces(piece) & board.color_combined(turn.not());

    (me.count() as f64 - them.count() as f64) * score
}

// https://www.chessprogramming.org/Simplified_Evaluation_Function
mod simplified_evaluation_function {
    use chess::{Board, Color, Piece};
    use rayon::prelude::{IntoParallelIterator, ParallelIterator};

    const CENTIPAWN_PAWN: f32 = 100f32;
    const CENTIPAWN_KNIGHT: f32 = 320f32;
    const CENTIPAWN_BISHOP: f32 = 330f32;
    const CENTIPAWN_ROOK: f32 = 500f32;
    const CENTIPAWN_QUEEN: f32 = 900f32;
    const CENTIPAWN_KING: f32 = 200000f32;

    const WHITE_PAWN_MATRIX: [[f32; 8]; 8] = [
        [0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32],
        [50f32, 50f32, 50f32, 50f32, 50f32, 50f32, 50f32, 50f32],
        [10f32, 10f32, 20f32, 30f32, 30f32, 20f32, 10f32, 10f32],
        [5f32, 5f32, 10f32, 25f32, 25f32, 10f32, 5f32, 5f32],
        [0f32, 0f32, 0f32, 20f32, 20f32, 0f32, 0f32, 0f32],
        [5f32, -5f32, -10f32, 0f32, 0f32, -10f32, -5f32, 5f32],
        [5f32, 10f32, 10f32, -20f32, -20f32, 10f32, 10f32, 5f32],
        [0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32],
    ];

    const WHITE_KNIGHT_MATRIX: [[f32; 8]; 8] = [
        [
            -50f32, -40f32, -30f32, -30f32, -30f32, -30f32, -40f32, -50f32,
        ],
        [-40f32, -20f32, 0f32, 0f32, 0f32, 0f32, -20f32, -40f32],
        [-30f32, 0f32, 10f32, 15f32, 15f32, 10f32, 0f32, -30f32],
        [-30f32, 5f32, 15f32, 20f32, 20f32, 15f32, 5f32, -30f32],
        [-30f32, 0f32, 15f32, 20f32, 20f32, 15f32, 0f32, -30f32],
        [-30f32, 5f32, 10f32, 15f32, 15f32, 10f32, 5f32, -30f32],
        [-40f32, -20f32, 0f32, 5f32, 5f32, 0f32, -20f32, -40f32],
        [
            -50f32, -40f32, -30f32, -30f32, -30f32, -30f32, -40f32, -50f32,
        ],
    ];

    const WHITE_BISHOP_MATRIX: [[f32; 8]; 8] = [
        [
            -20f32, -10f32, -10f32, -10f32, -10f32, -10f32, -10f32, -20f32,
        ],
        [-10f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, -10f32],
        [-10f32, 0f32, 5f32, 10f32, 10f32, 5f32, 0f32, -10f32],
        [-10f32, 5f32, 5f32, 10f32, 10f32, 5f32, 5f32, -10f32],
        [-10f32, 0f32, 10f32, 10f32, 10f32, 10f32, 0f32, -10f32],
        [-10f32, 10f32, 10f32, 10f32, 10f32, 10f32, 10f32, -10f32],
        [-10f32, 5f32, 0f32, 0f32, 0f32, 0f32, 5f32, -10f32],
        [
            -20f32, -10f32, -10f32, -10f32, -10f32, -10f32, -10f32, -20f32,
        ],
    ];

    const WHITE_ROOK_MATRIX: [[f32; 8]; 8] = [
        [0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32],
        [5f32, 10f32, 10f32, 10f32, 10f32, 10f32, 10f32, 5f32],
        [-5f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, -5f32],
        [-5f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, -5f32],
        [-5f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, -5f32],
        [-5f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, -5f32],
        [-5f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, -5f32],
        [0f32, 0f32, 0f32, 5f32, 5f32, 0f32, 0f32, 0f32],
    ];

    const WHITE_QUEEN_MATRIX: [[f32; 8]; 8] = [
        [-20f32, -10f32, -10f32, -5f32, -5f32, -10f32, -10f32, -20f32],
        [-10f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, -10f32],
        [-10f32, 0f32, 5f32, 5f32, 5f32, 5f32, 0f32, -10f32],
        [-5f32, 0f32, 5f32, 5f32, 5f32, 5f32, 0f32, -5f32],
        [0f32, 0f32, 5f32, 5f32, 5f32, 5f32, 0f32, -5f32],
        [-10f32, 5f32, 5f32, 5f32, 5f32, 5f32, 0f32, -10f32],
        [-10f32, 0f32, 5f32, 0f32, 0f32, 0f32, 0f32, -10f32],
        [-20f32, -10f32, -10f32, -5f32, -5f32, -10f32, -10f32, -20f32],
    ];

    const WHITE_KING_MID_MATRIX: [[f32; 8]; 8] = [
        [
            -30f32, -40f32, -40f32, -50f32, -50f32, -40f32, -40f32, -30f32,
        ],
        [
            -30f32, -40f32, -40f32, -50f32, -50f32, -40f32, -40f32, -30f32,
        ],
        [
            -30f32, -40f32, -40f32, -50f32, -50f32, -40f32, -40f32, -30f32,
        ],
        [
            -30f32, -40f32, -40f32, -50f32, -50f32, -40f32, -40f32, -30f32,
        ],
        [
            -20f32, -30f32, -30f32, -40f32, -40f32, -30f32, -30f32, -20f32,
        ],
        [
            -10f32, -20f32, -20f32, -20f32, -20f32, -20f32, -20f32, -10f32,
        ],
        [20f32, 20f32, 0f32, 0f32, 0f32, 0f32, 20f32, 20f32],
        [20f32, 30f32, 10f32, 0f32, 0f32, 10f32, 30f32, 20f32],
    ];

    const WHITE_KING_END_MATRIX: [[f32; 8]; 8] = [
        [
            -50f32, -40f32, -30f32, -20f32, -20f32, -30f32, -40f32, -50f32,
        ],
        [-30f32, -20f32, -10f32, 0f32, 0f32, -10f32, -20f32, -30f32],
        [-30f32, -10f32, 20f32, 30f32, 30f32, 20f32, -10f32, -30f32],
        [-30f32, -10f32, 30f32, 40f32, 40f32, 30f32, -10f32, -30f32],
        [-30f32, -10f32, 30f32, 40f32, 40f32, 30f32, -10f32, -30f32],
        [-30f32, -10f32, 20f32, 30f32, 30f32, 20f32, -10f32, -30f32],
        [-30f32, -30f32, 0f32, 0f32, 0f32, 0f32, -30f32, -30f32],
        [
            -50f32, -30f32, -30f32, -30f32, -30f32, -30f32, -30f32, -50f32,
        ],
    ];

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum GameTime {
        Start,
        Middle,
        End,
    }

    pub fn board_value(turn: Color, board: &Board, game: GameTime) -> f32 {
        let king_matrix = match game {
            GameTime::Start => WHITE_KING_MID_MATRIX,
            GameTime::Middle => WHITE_KING_MID_MATRIX,
            GameTime::End => WHITE_KING_END_MATRIX,
        };
        [
            (Piece::Pawn, Color::White, CENTIPAWN_PAWN, WHITE_PAWN_MATRIX),
            (
                Piece::Knight,
                Color::White,
                CENTIPAWN_KNIGHT,
                WHITE_KNIGHT_MATRIX,
            ),
            (
                Piece::Bishop,
                Color::White,
                CENTIPAWN_BISHOP,
                WHITE_BISHOP_MATRIX,
            ),
            (Piece::Rook, Color::White, CENTIPAWN_ROOK, WHITE_ROOK_MATRIX),
            (
                Piece::Queen,
                Color::White,
                CENTIPAWN_QUEEN,
                WHITE_QUEEN_MATRIX,
            ),
            (
                Piece::King,
                Color::White,
                CENTIPAWN_KING,
                king_matrix.clone(),
            ),
            (Piece::Pawn, Color::Black, CENTIPAWN_PAWN, WHITE_PAWN_MATRIX),
            (
                Piece::Knight,
                Color::Black,
                CENTIPAWN_KNIGHT,
                WHITE_KNIGHT_MATRIX,
            ),
            (
                Piece::Bishop,
                Color::Black,
                CENTIPAWN_BISHOP,
                WHITE_BISHOP_MATRIX,
            ),
            (Piece::Rook, Color::Black, CENTIPAWN_ROOK, WHITE_ROOK_MATRIX),
            (
                Piece::Queen,
                Color::Black,
                CENTIPAWN_QUEEN,
                WHITE_QUEEN_MATRIX,
            ),
            (Piece::King, Color::Black, CENTIPAWN_KING, king_matrix),
        ]
        .into_par_iter()
        .map(|(piece, color, piece_value, piece_matrix)| {
            let value = calculate_piece(color, board, piece, piece_value, piece_matrix);

            match color == turn {
                true => value,
                false => -value,
            }
        })
        .sum::<f32>()
    }

    fn calculate_piece(
        turn: Color,
        board: &Board,
        piece: Piece,
        piece_value: f32,
        piece_matrix: [[f32; 8]; 8],
    ) -> f32 {
        let pieces = board.pieces(piece) & board.color_combined(turn);

        let centipawn = pieces.count() as f32 * piece_value;
        let position: f32 = pieces
            .map(|square| {
                let rank = square.get_rank() as usize;
                let file = square.get_file() as usize;

                match turn {
                    Color::Black => piece_matrix[rank][file],
                    Color::White => piece_matrix[7 - rank][file],
                }
            })
            .sum();

        centipawn + position
        // centipawn
    }
}
