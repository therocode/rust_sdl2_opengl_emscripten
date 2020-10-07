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

        // Pass in an Rc of Gl to the scene so that it can create the Gl entities required
        let scene = scene::Scene::new(window.gl.clone(), window_size);

        Ok(Self { window, scene })
    }
}

impl emscripten_main_loop::MainLoop for Game {
    fn main_loop(&mut self) -> emscripten_main_loop::MainLoopEvent {
        // Extract and use all the input events from the window
        for event in self.window.sdl.event_pump().unwrap().poll_iter() {
            match event {
                // Terminate if we get a Quit event or the user presses Q
                Event::Quit { .. } => return emscripten_main_loop::MainLoopEvent::Terminate,
                Event::KeyDown {
                    scancode: Some(Scancode::Q),
                    ..
                } => return emscripten_main_loop::MainLoopEvent::Terminate,
                // Randomize the scene if the user presses Space
                Event::KeyDown {
                    scancode: Some(Scancode::Space),
                    ..
                } => self.scene.randomize(),
                _ => {}
            }
        }

        // Advance the logic of the scene one frame
        self.scene.update();

        // Render a single frame
        self.scene.render(&self.window.gl);

        // Display the rendered frame on the window
        self.window.window.gl_swap_window();

        emscripten_main_loop::MainLoopEvent::Continue
    }
}
