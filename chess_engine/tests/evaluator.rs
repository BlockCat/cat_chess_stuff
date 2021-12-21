use std::str::FromStr;

use chess::{Color, Game, MoveGen};
use chess_engine::{
    evaluator::{self, ChessEvaluation, ChessEvaluator},
    state::GameWrapper,
};
use mcts::Evaluator;

#[test]
fn test_mcts() {
    let game = Game::from_str("rnb1kbnr/pppp1ppp/8/4p2Q/4P2q/8/PPPP1PPP/RNB1KBNR w KQkq - 2 3").unwrap();

    chess_engine::find_move(&game, 10_000, 16, Color::White);
    
}

#[test]
fn test_eval_b() {
    let game = Game::from_str("rnb1k1nr/p4ppp/p1p5/8/8/2P2P1P/P5P1/1Rb1K1NR b kq - 1 14").unwrap();

    let moves = MoveGen::new_legal(&game.current_position()).collect::<Vec<_>>();
    let evaluator = ChessEvaluator(Color::Black);
    let eval = evaluator
        .evaluate_new_state(&GameWrapper(game), &moves, None)
        .1;
    assert_eq!(ChessEvaluation::Evaluation(10.0), eval);

    assert_eq!(-10.0, evaluator.interpret_evaluation_for_player(&eval, &Color::White));
    assert_eq!(10.0, evaluator.interpret_evaluation_for_player(&eval, &Color::Black));
}

#[test]
fn test_eval_w() {
    let game = Game::from_str("rnb1k1nr/p4ppp/p1p5/8/8/2P2P1P/P5P1/1Rb1K1NR b kq - 1 14").unwrap();

    let moves = MoveGen::new_legal(&game.current_position()).collect::<Vec<_>>();
    let evaluator = ChessEvaluator(Color::White);
    let eval = evaluator
        .evaluate_new_state(&GameWrapper(game), &moves, None)
        .1;

    assert_eq!(ChessEvaluation::Evaluation(-10.0), eval);
    assert_eq!(-10.0, evaluator.interpret_evaluation_for_player(&eval, &Color::White));
    assert_eq!(10.0, evaluator.interpret_evaluation_for_player(&eval, &Color::Black));
}
