use chess::{ChessMove, Color, Game};
use licoricedev::{client::Lichess, errors::LichessError, models::board::BoardState};
use std::{str::FromStr, sync::Arc};
use tokio_stream::StreamExt;

pub struct LiGame {
    id: String,
    lichess: Arc<Lichess>,
}

impl LiGame {
    pub async fn new(lichess: Arc<Lichess>, game_id: String) -> Result<Self, LichessError> {
        Ok(LiGame {
            id: game_id,
            lichess,
        })
    }

    pub async fn start_game_loop(&self) -> Result<(), LichessError> {
        let mut stream = self.lichess.stream_bot_game_state(&self.id).await?;

        let (bot_color, game) = match stream.try_next().await? {
            Some(board_state) => parse_initial_board_state(board_state),
            None => panic!("Could not get initial state!"),
        };

        println!("Moving as colour: {:?}", bot_color);

        if game.side_to_move() == bot_color {
            self.search_move(&game, bot_color).await?;
        }

        while let Some(board_state) = stream.next().await {
            match board_state {
                Ok(board_state) => {
                    if let Some(game) = parse_board_state(board_state) {
                        if game.side_to_move() == bot_color {
                            if let Err(e) = self.search_move(&game, bot_color).await {
                                println!("Could not play move, {:?}", e);
                            }
                        }
                    } else {
                        println!("Could not get board, probably a chat message.");
                    }
                }
                Err(e) => println!("move error: {:?}", e),
            }
        }

        Ok(())
    }

    pub async fn search_move(&self, game: &Game, color: Color) -> Result<(), LichessError> {
        println!("Start searching move for: {:?}", color);
        let chess_move = chess_engine::find_move(game, 100_000, 14, color);
        let uci_move = chess_move.to_string();

        println!("make move: {}", uci_move);
        self.lichess
            .make_a_bot_move(&self.id, &uci_move, false)
            .await?;
        Ok(())
    }
}

fn parse_board_state(board: BoardState) -> Option<Game> {
    let state = match board {
        BoardState::GameFull(full) => full.state,
        BoardState::GameState(state) => state,
        BoardState::ChatLine(_) => return None,
    };

    let mut game = Game::new();

    if !state.moves.is_empty() {
        for chess_move in state.moves.split(' ').map(ChessMove::from_str) {
            let chess_move = chess_move.unwrap();
            game.make_move(chess_move);
        }
    }

    Some(game)
}

fn parse_initial_board_state(board_state: BoardState) -> (Color, Game) {
    let full = match board_state {
        BoardState::GameFull(full) => full,
        BoardState::GameState(_) => unreachable!(),
        BoardState::ChatLine(_) => unreachable!(),
    };

    let is_white = match full.white {
        licoricedev::models::board::Challengee::LightUser(user)
            if user.username == "BlockCat_bot" =>
        {
            true
        }
        _ => false,
    };

    let is_black = match full.black {
        licoricedev::models::board::Challengee::LightUser(user)
            if user.username == "BlockCat_bot" =>
        {
            true
        }
        _ => false,
    };

    let color = match (is_white, is_black) {
        (true, false) => Color::White,
        (false, true) => Color::Black,
        _ => unreachable!("Bot is neither of the players?"),
    };

    let mut game = Game::new();
    if !full.state.moves.is_empty() {
        for chess_move in full
            .state
            .moves
            .split(' ')
            .map(ChessMove::from_str)
            .collect::<Vec<_>>()
        {
            let chess_move = chess_move.expect("Could not parse chess move");
            game.make_move(chess_move);
        }
    }

    (color, game)
}
