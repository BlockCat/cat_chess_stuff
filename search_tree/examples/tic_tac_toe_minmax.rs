use player::Player;
use search_tree::{mini_max::mini_max, GameEvaluator, GameNode, Node, SearchGame};
use std::{fmt::Display, ops::Neg};

fn main() {
    let mut board = TicTacToeState::default();

    println!("{}", board);

    while board.is_terminal().is_none() {
        let result =
            mini_max::<TicTacToeGame>(&MyEvaluator(board.current_player()), board.clone(), 10)
                .unwrap();
        board = board.make_move(result.found_move);
        println!("{}", board);
    }

    println!("Terminal: {:?}", board.is_terminal())
}

mod player {
    use std::ops::Neg;

    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
    pub enum Player {
        Player1,
        Player2,
    }

    impl Default for Player {
        fn default() -> Self {
            Player::Player1
        }
    }

    impl Neg for Player {
        type Output = Player;

        fn neg(self) -> Self::Output {
            match self {
                Player::Player1 => Player::Player2,
                Player::Player2 => Player::Player1,
            }
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct TicTacToeAction(usize, usize);

#[derive(Debug, Default, PartialEq, Eq, std::hash::Hash, Clone)]
struct TicTacToeState {
    current_player: Player,
    board: [[Option<Player>; 3]; 3],
}

impl Display for TicTacToeState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r1 = self.board[0]
            .iter()
            .map(|c| match c {
                Some(Player::Player1) => 'X',
                Some(Player::Player2) => 'O',
                None => ' ',
            })
            .collect::<String>();
        let r2 = self.board[1]
            .iter()
            .map(|c| match c {
                Some(Player::Player1) => 'X',
                Some(Player::Player2) => 'O',
                None => ' ',
            })
            .collect::<String>();
        let r3 = self.board[2]
            .iter()
            .map(|c| match c {
                Some(Player::Player1) => 'X',
                Some(Player::Player2) => 'O',
                None => ' ',
            })
            .collect::<String>();
        f.write_fmt(format_args!("Turn: {:?}\n", self.current_player))?;
        f.write_fmt(format_args!("{}\n", r1))?;
        f.write_fmt(format_args!("{}\n", r2))?;
        f.write_fmt(format_args!("{}\n", r3))?;

        Ok(())
    }
}

impl GameNode for TicTacToeState {
    type Move = TicTacToeAction;
    type TerminalResult = StateEval;
    type Player = Player;

    fn current_player(&self) -> Self::Player {
        self.current_player.clone()
    }

    fn legal_moves(&self) -> Vec<Self::Move> {
        if self.is_terminal().is_some() {
            Vec::new()
        } else {
            self.board
                .iter()
                .enumerate()
                .map(|(y, row)| {
                    row.iter()
                        .enumerate()
                        .filter_map(move |(x, cell)| match cell {
                            None => Some(TicTacToeAction(x, y)),
                            _ => None,
                        })
                })
                .flatten()
                .collect()
        }
    }

    fn is_terminal(&self) -> Option<Self::TerminalResult> {
        for line in &[
            // Rows
            [(0, 0), (1, 0), (2, 0)],
            [(0, 1), (1, 1), (2, 1)],
            [(0, 2), (1, 2), (2, 2)],
            // Cols
            [(0, 0), (0, 1), (0, 2)],
            [(1, 0), (1, 1), (1, 2)],
            [(2, 0), (2, 1), (2, 2)],
            // Diags
            [(0, 0), (1, 1), (2, 2)],
            [(2, 0), (1, 1), (0, 2)],
        ] {
            if line
                .into_iter()
                .all(|&(x, y)| self.board[y][x] == Some(Player::Player1))
            {
                return Some(StateEval::Winner(Player::Player1));
            }
            if line
                .into_iter()
                .all(|&(x, y)| self.board[y][x] == Some(Player::Player2))
            {
                return Some(StateEval::Winner(Player::Player2));
            }
        }

        let places_left = self
            .board
            .iter()
            .flat_map(|r| r.iter())
            .all(|x| x.is_some());
        if places_left {
            return Some(StateEval::Draw);
        } else {
            return None;
        }
    }

    fn make_move(&self, m: Self::Move) -> Self {
        let mut board = self.board.clone();
        board[m.1][m.0] = Some(self.current_player());

        Self {
            current_player: self.current_player.neg(),
            board,
        }
    }
}

#[derive(Debug, Clone)]
enum StateEval {
    Winner(Player),
    Draw,
}

struct TicTacToeGame;

impl SearchGame for TicTacToeGame {
    type Node = TicTacToeState;
    type Evaluator = MyEvaluator;
}

struct MyEvaluator(Player);

impl GameEvaluator<TicTacToeGame> for MyEvaluator {
    type Evaluation = (StateEval, usize);

    fn evaluate(&self, node: &Node<TicTacToeGame>, depth: usize) -> Self::Evaluation {
        if let Some(result) = node.is_terminal() {
            return (result, depth);
        } else {
            unreachable!()
        }
    }

    fn interpret_for_player(
        &self,
        evaluation: &Self::Evaluation,
        player: search_tree::Player<TicTacToeGame>,
    ) -> f64 {
        match evaluation.0 {
            StateEval::Winner(winner) if winner == player => 10.0 - evaluation.1 as f64,
            StateEval::Winner(_) => -10.0 + evaluation.1 as f64,
            StateEval::Draw => 0.0,
        }
    }
}
