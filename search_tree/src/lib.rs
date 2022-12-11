pub mod mini_max;
// pub mod alpha_beta;

pub type Node<S> = <S as SearchGame>::Node;
pub type Move<S> = <<S as SearchGame>::Node as GameNode>::Move;
pub type Evaluator<S> = <S as SearchGame>::Evaluator;
pub type Evaluation<S> = <<S as SearchGame>::Evaluator as GameEvaluator<S>>::Evaluation;
pub type Player<S> = <<S as SearchGame>::Node as GameNode>::Player;
pub trait SearchGame: Sized {
    type Node: GameNode;
    type Evaluator: GameEvaluator<Self>;
}
pub trait GameNode {
    type Move: Clone;
    type TerminalResult;
    type Player;

    fn current_player(&self) -> Self::Player;

    fn legal_moves(&self) -> Vec<Self::Move>;

    fn is_terminal(&self) -> Option<Self::TerminalResult>;

    fn make_move(&self, m: Self::Move) -> Self;
}

pub trait GameEvaluator<S: SearchGame> {
    type Evaluation: Clone;

    fn evaluate(&self, node: &Node<S>, depth: usize) -> Self::Evaluation; 
    fn interpret_for_player(&self, evaluation: &Self::Evaluation, player: Player<S>) -> f64;
}
