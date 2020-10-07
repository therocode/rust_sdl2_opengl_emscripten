use crate::scene;
use crate::window;
use sdl2::event::Event;
use sdl2::keyboard::Scancode;

pub struct Game {
    scene: scene::Scene,
    window: window::GlWindow,
}

impl Game {
    pub fn new() -> Result<Self, ()> {
        let window_size = glm::vec2(800, 600);
        let window =
            window::GlWindow::new("Triangle Spinner", window_size, window::GlProfile::ES3).unwrap();
        let scene = scene::Scene::new(window.gl.clone(), window_size);

        Ok(Self { window, scene })
    }
}

impl emscripten_main_loop::MainLoop for Game {
    fn main_loop(&mut self) -> emscripten_main_loop::MainLoopEvent {
        for event in self.window.sdl.event_pump().unwrap().poll_iter() {
            match event {
                Event::Quit { .. } => return emscripten_main_loop::MainLoopEvent::Terminate,
                Event::KeyDown {
                    scancode: Some(Scancode::Q),
                    ..
                } => return emscripten_main_loop::MainLoopEvent::Terminate,
                Event::KeyDown {
                    scancode: Some(Scancode::Space),
                    ..
                } => self.scene.randomize(),
                _ => {}
            }
        }

        self.scene.update();

        self.scene.render(&self.window.gl);

        self.window.window.gl_swap_window();

        emscripten_main_loop::MainLoopEvent::Continue
    }
}
