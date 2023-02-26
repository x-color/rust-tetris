// mod keyword links child modules to this module.
// If no mod keyword, Rust ignores the module files except for the root module(main.rs or lib.rs).
mod block;
mod game;

// use keyword imports functions, structs, constant, enum etc...
// We can use 2 types of the keyword.
// - Import functions, types: write a parent path for the definitions, such as `use std::io`.
// - Import others: write a full path for the definitions, such as `use std::fs::File`.
// See https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#creating-idiomatic-use-paths
use game::{Game, Position};
use getch_rs::{Getch, Key};
use std::sync::{Arc, Mutex};
use std::{thread, time};

fn main() {
    let game = Arc::new(Mutex::new(Game::new()));
    println!("\x1b[2J\x1b[H\x1b[?25l");
    game::draw(&game.lock().unwrap());

    {
        let game = Arc::clone(&game);
        let _ = thread::spawn(move || loop {
            let sleep_msec =
                match 1000u64.saturating_sub((game.lock().unwrap().line as u64 / 10) * 100) {
                    0 => 100,
                    msec => msec,
                };
            thread::sleep(time::Duration::from_millis(sleep_msec));
            let mut game = game.lock().unwrap();
            let new_pos = Position {
                x: game.pos.x,
                y: game.pos.y + 1,
            };
            if !game::is_collision(&game.field, &new_pos, &game.block) {
                game.pos = new_pos;
            } else {
                if game::landing(&mut game).is_err() {
                    game::gameover(&game);
                }
            }
            game::draw(&game);
        });
    }
    let g = Getch::new();
    loop {
        match g.getch() {
            Ok(Key::Left) => {
                let mut game = game.lock().unwrap();
                let new_pos = Position {
                    x: game.pos.x.checked_sub(1).unwrap_or(game.pos.x),
                    y: game.pos.y,
                };
                game::move_block(&mut game, new_pos);
                game::draw(&game);
            }
            Ok(Key::Right) => {
                let mut game = game.lock().unwrap();
                let new_pos = Position {
                    x: game.pos.x + 1,
                    y: game.pos.y,
                };
                game::move_block(&mut game, new_pos);
                game::draw(&game);
            }
            Ok(Key::Down) => {
                let mut game = game.lock().unwrap();
                let new_pos = Position {
                    x: game.pos.x,
                    y: game.pos.y + 1,
                };
                game::move_block(&mut game, new_pos);
                game::draw(&game);
            }
            Ok(Key::Up) => {
                let mut game = game.lock().unwrap();
                game::hard_drop(&mut game);
                if game::landing(&mut game).is_err() {
                    // ブロックを生成できないならゲームオーバー
                    game::gameover(&game);
                }
                game::draw(&game);
            }
            Ok(Key::Char('x')) => {
                let mut game = game.lock().unwrap();
                game::rotate_right(&mut game);
                game::draw(&game);
            }
            Ok(Key::Char('z')) => {
                let mut game = game.lock().unwrap();
                game::rotate_left(&mut game);
                game::draw(&game);
            }
            Ok(Key::Char(' ')) => {
                let mut game = game.lock().unwrap();
                game::hold(&mut game);
                game::draw(&game);
            }
            Ok(Key::Char('q')) => {
                game::quit();
            }
            _ => (),
        }
    }
}
