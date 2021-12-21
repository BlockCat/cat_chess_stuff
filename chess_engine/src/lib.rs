use chess::{ChessMove, Game, Color, MoveGen};
use evaluator::ChessEvaluator;
use mcts::{transposition_table::ApproxTable, tree_policy::UCTPolicy, MCTSManager, Evaluator};
use mcts_handler::ChessMCTS;
use state::GameWrapper;

pub mod evaluator;
pub mod mcts_handler;
pub mod state;

pub fn find_move(game: &Game, playouts: u32, cores: usize, color: Color) -> ChessMove {
    
    let mut manager = MCTSManager::new(
        GameWrapper(game.clone()),
        ChessMCTS,
        ChessEvaluator(color),
        UCTPolicy::new(10.5),
        ApproxTable::new(1024)
    );

    manager.perf_test_to_stderr(cores);

    let moves = MoveGen::new_legal(&game.current_position()).collect::<Vec<_>>();
    let e = ChessEvaluator(color).evaluate_new_state(&GameWrapper(game.clone()), &moves, None);    

    manager.playout_n_parallel(playouts, cores);
    // manager.playout_n_parallel(1, 16);

    println!("current evaluation: {:?}", e.1);
    manager.tree().display_moves();

    manager.best_move().unwrap()
}
