use pyo3::prelude::*;

pub mod board;
pub mod error;
pub mod game;
pub mod mono_strategy;
pub mod player;
pub mod referee;
pub mod strategy;

pub mod prelude {
    pub use crate::game::Game;
    pub use crate::player::Player;
    pub use crate::strategy::*;
}

use crate::player::Player;
use crate::game::{Game, CompletedGame};

#[pyclass(name="Game")]
pub struct PyGame {
    game: Game<Player, Player>
}

impl From<Game<Player, Player>> for PyGame {
    fn from(game: Game<Player, Player>) -> Self {
        PyGame { game }
    }
}

#[pymethods]
impl PyGame {
    /*
    #[rustfmt::skip]
    pub fn new_with_players(player_1: Player, player_2: Player) -> PyGame {
        Game::new()
            .add_player(player_1)
            .add_player(player_2)
            .into()
    }
    */

    #[deprecated]
    #[rustfmt::skip]
    pub fn new_with_hardcoded_players() -> PyGame {
        Game::new()
            .add_player(
                Player::builder()
                    .human()
                    .with_name("Thomas")
                    .build()
        )
        .add_player(
            Player::builder()
                .with_name("Jay")
                .with_strategy(
                    PredefinedMoves::from_iterable_nondiscarding([5, 1, 3, 7, 9, 2, 4, 6, 8]).unwrap()
                )
                .build()
        )
        .into()
    }

    pub fn play_match(&self) -> String {
        self.game.clone().play_match().to_string()
    }

}


#[pymodule]
mod tictactoe {
    use super::*;

    #[pymodule_export]
    use PyGame;


    #[pymodule]
    mod exception {

    }

    #[pymodule_init]
    pub fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        pyo3_log::init();

        // Submodule setup, <https://github.com/PyO3/pyo3/discussions/5397#discussioncomment-14298706>
        let modules = PyModule::import(m.py(), "sys")?.getattr("modules")?;
        modules.set_item("tictactoe.exception", m.getattr("exception")?)?;

        Ok(())
    }
}
