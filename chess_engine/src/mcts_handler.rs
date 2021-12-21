use chess::Game;
use mcts::{MCTS, tree_policy::UCTPolicy, transposition_table::ApproxTable};

use crate::{state::GameWrapper, evaluator::ChessEvaluator};


#[derive(Default)]
pub struct ChessMCTS;

impl MCTS for ChessMCTS {
    type State = GameWrapper;
    type Eval = ChessEvaluator;
    type TreePolicy = UCTPolicy<()>;
    type TranspositionTable = ApproxTable<Self>;
    type NodeData = ();
    type ExtraThreadData = ();

    fn virtual_loss(&self) -> f64 {
        crate::evaluator::SCALE
    }
}

