use anyhow::anyhow;
pub struct GlWindow {
    pub gl: std::rc::Rc<crate::gl::Gl>,
    pub gl_context: sdl2::video::GLContext,
    pub window: sdl2::video::Window,
    pub video: sdl2::VideoSubsystem,
    pub sdl: sdl2::Sdl,
}

pub enum GlProfile {
    _Core43,
    ES3,
}

impl GlWindow {
    pub fn new(title: &str, size: glm::UVec2, profile: GlProfile) -> anyhow::Result<Self> {
        // Initialize SDL
        let sdl = sdl2::init().map_err(|e| anyhow!(e))?;

        // Setup the video subsystem
        let video = sdl.video().map_err(|e| anyhow!(e))?;

        let context_params = match profile {
            GlProfile::_Core43 => (sdl2::video::GLProfile::Core, 4, 3),
            GlProfile::ES3 => (sdl2::video::GLProfile::GLES, 3, 0),
        };

        video.gl_attr().set_context_profile(context_params.0);
        video.gl_attr().set_context_major_version(context_params.1);
        video.gl_attr().set_context_minor_version(context_params.2);

        // Create a window
        let mut window = video
            .window(title, size.x, size.y)
            .resizable()
            .opengl()
            .position_centered()
            .build()?;

        // Create an OpenGL context
        let gl_context = window.gl_create_context().map_err(|e| anyhow!(e))?;

        assert!(gl_context.is_current()); //don't think this should ever happen

        // Load the OpenGL API library
        let gl = std::rc::Rc::new(crate::gl::Gl::load_with(|s| {
            video.gl_get_proc_address(s) as *const _
        }));

        window.set_size(size.x, size.y)?;

        Ok(Self {
            sdl,
            video,
            window,
            gl_context,
            gl,
        })
    }
}
