#![allow(unused_variables)]
#![allow(dead_code)]
mod game_logik;

#[derive(Debug, Clone, PartialEq)]
enum State {
    Dead,
    Alive,
}

fn main() {
    // launch(App);
    game_logik::play();
}
