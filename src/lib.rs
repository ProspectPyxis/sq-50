pub mod english_draughts;
pub mod error;
pub mod game;
pub mod hub;
pub mod square;

use crate::game::Game;
// use rayon::prelude::*;

// const NON_PARALLEL_CAP: usize = 3;

pub fn perft(depth: usize, game: &mut english_draughts::GameEnglishDraughts) -> (usize, usize) {
    if depth == 0 {
        return (1, 1);
    }

    let moves = game.gen_moves();
    let mc = moves.len();
    // Bulk counting
    if depth == 1 {
        return (mc, mc);
    }

    let fold_iter = |acc: (usize, usize), m| {
        let mut game = game.clone();
        game.make_move(m)
            .unwrap_or_else(|x| panic!("move error: {}\n{:#?}", x, m));
        let (node, count) = perft(depth - 1, &mut game);
        (acc.0 + node, acc.1.max(count))
    };

    moves.iter().fold((0, mc), fold_iter)

    /*
    moves
        .par_iter()
        .fold(|| (0, mc), fold_iter)
        .reduce(|| (0, 0), |a, b| (a.0 + b.0, a.1.max(b.1)))
    */

    /*
    if depth <= NON_PARALLEL_CAP {
        moves.iter().fold((0, mc), fold_iter)
    } else {
        moves
            .par_iter()
            .fold(|| (0, mc), fold_iter)
            .reduce(|| (0, 0), |a, b| (a.0 + b.0, a.1.max(b.1)))
    }
    */
}
