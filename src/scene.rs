use std::rc::Rc;

use rand::Rng;

use crate::gl;

pub struct Scene {
    /// Defines how we are looking at the scene
    camera: gl::Camera,
    /// The color to which the background is cleared at the start of the frame
    bg_color: gl::Color,
    /// The color of the triangle
    triangle_color: gl::Color,
    /// The current angle position of the triangle as it rotates around the center
    triangle_rotation: f32,
    /// The speed at which the above angle position changes each frame
    rotational_speed: f32,

    // GL resources
    shader: gl::Shader,
    vao: gl::Vao,
    triangle_positions_vbo: gl::ArrayVbo,
    triangle_colors_vbo: gl::ArrayVbo,
}

impl Scene {
    pub fn new(gl: Rc<gl::Gl>, window_size: glm::UVec2) -> Self {
        // Plain orthographic camera
        let camera = gl::Camera::new(gl::Projection::new_orthographic(
            glm::convert(window_size),
            1.0,
            -100.0,
            100.0,
        ));

        // One VBO for the position attribute, one for the colors
        let triangle_positions_vbo = gl::ArrayVbo::new(gl.clone());
        let triangle_colors_vbo = gl::ArrayVbo::new(gl.clone());

        let vao = gl::Vao::new(
            &vec![
                gl::VertexAttribPointerDefinition::new(
                    &triangle_positions_vbo,
                    0,
                    gl::VertexAttributeDefinition {
                        dimension_count: 3,
                        data_type: gl::FLOAT,
                        normalized: false,
                        stride: 0,
                        offset: 0,
                    },
                ),
                gl::VertexAttribPointerDefinition::new(
                    &triangle_colors_vbo,
                    1,
                    gl::VertexAttributeDefinition {
                        dimension_count: 4,
                        data_type: gl::UNSIGNED_BYTE,
                        normalized: true,
                        stride: 0,
                        offset: 0,
                    },
                ),
            ],
            gl.clone(),
        );

        let mut res = Self {
            camera,
            bg_color: gl::Color {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            },
            triangle_color: gl::Color {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            },
            triangle_rotation: 0.0,
            rotational_speed: 0.0,
            shader: gl::new_default_shader(gl),
            vao,
            triangle_positions_vbo,
            triangle_colors_vbo,
        };

        res.randomize();

        res
    }
    pub fn randomize(&mut self) {
        let rng = &mut rand::thread_rng();
        self.triangle_color = gl::Color {
            r: rng.gen_range(100, 256) as u8,
            g: rng.gen_range(100, 256) as u8,
            b: rng.gen_range(100, 256) as u8,
            a: 255,
        };
        self.bg_color = gl::Color {
            r: rng.gen_range(0, 101) as u8,
            g: rng.gen_range(0, 101) as u8,
            b: rng.gen_range(0, 101) as u8,
            a: 255,
        };
        self.rotational_speed = rng.gen_range(-0.05, 0.05);
    }
    pub fn update(&mut self) {
        // Advance rotation by rotational speed
        self.triangle_rotation += self.rotational_speed;
    }
    pub fn render(&self, gl: &gl::Gl) {
        // Clear the window
        gl::clear(&self.bg_color, gl);

        // Bind gl entities
        self.shader.bind(gl);
        self.vao.bind(gl);

        // Prepare geometry
        let distance_to_center = 200.0;
        let triangle_position =
            glm::rotate_vec2(&glm::vec2(distance_to_center, 0.0), self.triangle_rotation);
        let offset = glm::vec3(triangle_position.x, triangle_position.y, 0.0);
        let z = 0.5;
        let triangle_size_factor = 30.0;

        // 2D model space
        let top_p = glm::vec2(0.0, -2.0) * triangle_size_factor;
        let bottom_right_p = glm::vec2(1.5, 1.0) * triangle_size_factor;
        let bottom_left_p = glm::vec2(-1.5, 1.0) * triangle_size_factor;

        // 3D world space
        let top_p = glm::vec3(top_p.x, top_p.y, z) + offset;
        let bottom_right_p = glm::vec3(bottom_right_p.x, bottom_right_p.y, z) + offset;
        let bottom_left_p = glm::vec3(bottom_left_p.x, bottom_left_p.y, z) + offset;
        let positions = vec![top_p, bottom_right_p, bottom_left_p];

        let colors = vec![
            self.triangle_color,
            self.triangle_color,
            self.triangle_color,
        ];

        // Upload geometry
        self.triangle_positions_vbo
            .upload_array_vbo_vec(gl::STREAM_DRAW, &positions, gl);
        self.triangle_colors_vbo
            .upload_array_vbo_vec(gl::STREAM_DRAW, &colors, gl);

        // Set projection
        let vp_mat = self.camera.view_projection_matrix();
        let projection_matrix_location =
            gl::get_uniform_location(&self.shader, gl::DEFAULT_PROJECTION_UNIFORM, gl).unwrap(); //unwrap since we are using hard coded name

        unsafe {
            gl.UniformMatrix4fv(
                projection_matrix_location,
                1,
                crate::gl::FALSE,
                vp_mat.as_ptr(),
            );
        }

        // Render
        gl::draw_arrays(positions.len() as i32, gl);

        // Clean up
        gl::Shader::unbind(gl);
        gl::Vao::unbind(gl);
    }
}
