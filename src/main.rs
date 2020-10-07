extern crate nalgebra_glm as glm;
extern crate sdl2;

mod game;
mod gl;
mod scene;
mod window;

fn main() {
    let game = game::Game::new().unwrap();

    emscripten_main_loop::run(game);
}
