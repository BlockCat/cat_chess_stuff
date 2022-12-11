use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{Evaluation, Evaluator, GameEvaluator, GameNode, Move, Node, SearchGame};

pub struct MiniMaxResult<N: SearchGame> {
    pub found_move: Move<N>,
    pub evaluation: Evaluation<N>,
    // nodes_evaluated: usize,
}

pub fn mini_max<N: SearchGame>(
    searcher: &Evaluator<N>,
    node: Node<N>,
    depth: usize,
) -> Option<MiniMaxResult<N>>
where
    Move<N>: Send + Sync,
    Node<N>: Send + Sync,
    Evaluation<N>: Send + Sync,
    Evaluator<N>: Send + Sync,
{
    if depth == 0 {
        return None;
    }

    if let Some(_) = node.is_terminal() {
        return None;
    }

    node.legal_moves()
        .into_par_iter()
        .map(|mov| {
            let child_node = node.make_move(mov.clone());
            let eval = max_min_phase::<N>(searcher, child_node, 1, depth);
            let interpreted = searcher.interpret_for_player(&eval, node.current_player());
            (eval, interpreted, mov)
        })
        .max_by(|a, b| a.1.partial_cmp(&b.1).expect("Could not compare"))
        .map(|(eval, _, mov)| MiniMaxResult {
            found_move: mov,
            evaluation: eval,
        })
}

fn max_min_phase<N: SearchGame>(
    searcher: &Evaluator<N>,
    node: N::Node,
    depth: usize,
    max_depth: usize,
) -> Evaluation<N>
where
    Move<N>: Send + Sync,
    Node<N>: Send + Sync,
    Evaluation<N>: Send + Sync,
    Evaluator<N>: Send + Sync,
{
    use rayon::prelude::*;

    if depth == max_depth || node.is_terminal().is_some() {
        return searcher.evaluate(&node, depth);
    }

    node.legal_moves()
        .into_par_iter()
        .map(|mov| {
            let eval = max_min_phase::<N>(searcher, node.make_move(mov), depth + 1, max_depth);
            let interpreted = searcher.interpret_for_player(&eval, node.current_player());
            (eval, interpreted)
        })
        .max_by(|(_, a), (_, b)| a.partial_cmp(&b).unwrap())
        .unwrap()
        .0
}
