#[cfg(test)]
use hamcrest2::prelude::*;
use rstest::*;

use tictactoe::game::EndState;
use tictactoe::prelude::*;

#[rstest]
fn test_out_of_moves_player_1() {
    let moves_for_tom: &[usize] = &[5, 1];
    let moves_for_jay: &[usize] = &[5, 1, 3, 7, 9, 2, 4, 6, 8];

    #[rustfmt::skip]
    let game = Game::new()
        .add_player(
            Player::builder()
                .with_name("Thomas")
                .with_strategy(
                    PredefinedMoves::try_from(moves_for_tom).unwrap()
                )
                .build()
        )
        .add_player(
            Player::builder()
                .with_name("Jay")
                .with_strategy(
                    PredefinedMoves::try_from(moves_for_jay).unwrap()
                )
                .build(),
        )
        .play_match();

    assert_that!(&game.winner, is(some()));
    assert_that!(&game.loser, is(some()));

    assert_that!(game.winner.unwrap().get_name(), is(equal_to("Jay")));
    assert_that!(game.loser.unwrap().get_name(), is(equal_to("Thomas")));

    assert_that!(game.end_state, is(equal_to(EndState::Forfeit)));
}

#[rstest]
fn test_out_of_moves_player_2() {
    #[rustfmt::skip]
    let game = Game::new()
        .add_player(
            Player::builder()
                .with_name("Thomas")
                .with_strategy(
                    PredefinedMoves::try_from(&[5, 1, 3, 7, 9, 2, 4, 6, 8]).unwrap()
                )
                .build()
        )
        .add_player(
            Player::builder()
                .with_name("Jay")
                .with_strategy(
                    PredefinedMoves::try_from(&[5, 1]).unwrap()
                )
                .build(),
        )
        .play_match();

    assert_that!(&game.winner, is(some()));
    assert_that!(&game.loser, is(some()));

    assert_that!(game.winner.unwrap().get_name(), is(equal_to("Thomas")));
    assert_that!(game.loser.unwrap().get_name(), is(equal_to("Jay")));

    assert_that!(game.end_state, is(equal_to(EndState::Forfeit)));
}

#[rstest]
fn test_stalemate() {
    #[rustfmt::skip]
    let game = Game::new()
        .add_player(
            Player::builder()
                .with_name("Thomas")
                .with_strategy(
                    PredefinedMoves::try_from(&[5, 1, 3, 7, 9, 2, 4, 6, 8]).unwrap()
                )
                .build()
        )
        .add_player(
            Player::builder()
                .with_name("Jay")
                .with_strategy(
                    PredefinedMoves::try_from(&[5, 1, 3, 7, 9, 2, 4, 6, 8]).unwrap()
                )
                .build(),
        )
        .play_match();

    assert_that!(&game.winner, is(none()));
    assert_that!(&game.loser, is(none()));

    assert_that!(game.end_state, is(equal_to(EndState::Stalemate)));
}

#[rstest]
fn test_win_player_1() {
    #[rustfmt::skip]
    let game = Game::new()
        .add_player(
            Player::builder()
                .with_name("Thomas")
                .with_strategy(
                    PredefinedMoves::try_from(&[5, 1, 9]).unwrap()
                )
                .build()
        )
        .add_player(
            Player::builder()
                .with_name("Jay")
                .with_strategy(
                    PredefinedMoves::try_from(&[5, 3, 7, 9, 2, 4, 6, 8]).unwrap()
                )
                .build(),
        )
        .play_match();

    assert_that!(&game.winner, is(some()));
    assert_that!(&game.loser, is(some()));

    assert_that!(game.winner.unwrap().get_name(), is(equal_to("Thomas")));
    assert_that!(game.loser.unwrap().get_name(), is(equal_to("Jay")));

    assert_that!(game.end_state, is(equal_to(EndState::Win)));
}

#[rstest]
fn test_win_player_2() {
    #[rustfmt::skip]
    let game = Game::new()
        .add_player(
            Player::builder()
                .with_name("Thomas")
                .with_strategy(
                    PredefinedMoves::try_from(&[5, 1, 2, 4]).unwrap()
                )
                .build()
        )
        .add_player(
            Player::builder()
                .with_name("Jay")
                .with_strategy(
                    PredefinedMoves::try_from(&[3, 6, 9]).unwrap()
                )
                .build(),
        )
        .play_match();

    assert_that!(&game.winner, is(some()));
    assert_that!(&game.loser, is(some()));

    assert_that!(game.winner.unwrap().get_name(), is(equal_to("Jay")));
    assert_that!(game.loser.unwrap().get_name(), is(equal_to("Thomas")));

    assert_that!(game.end_state, is(equal_to(EndState::Win)));
}
