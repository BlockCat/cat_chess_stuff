use crate::ligame::LiGame;
use licoricedev::{client::Lichess, errors::LichessError, models::board::Event};
use std::{collections::HashMap, sync::Arc};
use tokio::task::JoinHandle;
use tokio_stream::StreamExt;

pub struct LiWorld {
    pub current_games: HashMap<String, JoinHandle<()>>,
    lichess: Arc<Lichess>,
}

impl LiWorld {
    pub fn new(lichess: Arc<Lichess>) -> Self {
        Self {
            lichess,
            current_games: Default::default(),
        }
    }

    pub async fn listen(&mut self) -> Result<(), LichessError> {
        let mut stream = self.lichess.stream_incoming_events().await?;

        while let Some(event) = stream.try_next().await? {
            match event {
                Event::GameStart { game } => self.start_game(game.id),
                Event::GameFinish { game } => {
                    if let Some(handle) = self.current_games.get(&game.id) {
                        handle.abort();
                    }
                }
                Event::Challenge { challenge } => {
                    println!("Received challenge: {:?}", challenge);
                    if challenge.variant.name.to_lowercase() != "standard" {
                        println!("Rejected challenge, not standard but: {}", challenge.variant.name);
                        self.reject_challenge(challenge.id).await?;
                        continue;
                    } else if challenge.speed.to_lowercase() != "correspondence" {
                        println!("Rejected challenge, not correspondence but: {}", challenge.speed);
                        self.reject_challenge(challenge.id).await?;
                        continue;
                    } else {
                        self.accept_challenge(challenge.id).await?;
                    }
                }
                Event::ChallengeCanceled { challenge: _ } => {}
                Event::ChallengeDeclined { challenge: _ } => {}
            }
        }
        Ok(())
    }

    pub fn start_game(&mut self, game_id: String) {
        println!("Starting game: {}", game_id);
        let lichess = self.lichess.clone();

        let game_id_2 = game_id.clone();

        let handle = tokio::spawn(async move {
            let li_game = LiGame::new(lichess, game_id.clone())
                .await
                .expect("Could not get lichess game");
            match li_game.start_game_loop().await {
                Ok(_) => {}
                Err(e) => println!("Failed game loop: {:?}", e),
            }
        });

        self.current_games.insert(game_id_2, handle);
    }

    pub async fn reject_challenge(&self, challenge_id: String) -> Result<(), LichessError> {
        println!("Rejected challenge: {}", challenge_id);
        self.lichess.challenge_decline(&challenge_id, None).await
    }

    pub async fn accept_challenge(&self, challenge_id: String) -> Result<(), LichessError> {
        println!("Accepted challenge: {}", challenge_id);
        self.lichess.challenge_accept(&challenge_id).await
    }
}
