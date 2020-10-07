#[derive(Debug, Clone, Copy)]
pub struct Perspective {
    pub vertical_fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
}
#[derive(Debug, Clone, Copy)]
pub struct Orthographic {
    pub size: glm::Vec2,
    pub zoom: f32,
    pub near: f32,
    pub far: f32,
}
#[derive(Debug, Clone, Copy)]
pub enum Projection {
    Perspective(Perspective),
    Orthographic(Orthographic),
}

impl Projection {
    pub fn new_perspective(vertical_fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        Projection::Perspective(Perspective {
            vertical_fov,
            aspect,
            near,
            far,
        })
    }

    pub fn new_orthographic(size: glm::Vec2, zoom: f32, near: f32, far: f32) -> Self {
        Projection::Orthographic(Orthographic {
            size,
            zoom,
            near,
            far,
        })
    }

    pub fn orthographic_from_edges(
        top: f32,
        bottom: f32,
        left: f32,
        right: f32,
        near: f32,
        far: f32,
    ) -> Self {
        let size = glm::vec2(right - left, bottom - top);

        Projection::Orthographic(Orthographic {
            size,
            zoom: 1.0,
            near,
            far,
        })
    }

    pub fn as_perspective_ref(&self) -> Option<&Perspective> {
        match self {
            Projection::Perspective(p) => Some(p),
            Projection::Orthographic(_) => None,
        }
    }

    pub fn as_perspective_mut(&mut self) -> Option<&mut Perspective> {
        match self {
            Projection::Perspective(p) => Some(p),
            Projection::Orthographic(_) => None,
        }
    }

    pub fn as_orthographic_ref(&self) -> Option<&Orthographic> {
        match self {
            Projection::Perspective(_) => None,
            Projection::Orthographic(o) => Some(o),
        }
    }

    pub fn as_orthographic_mut(&mut self) -> Option<&mut Orthographic> {
        match self {
            Projection::Perspective(_) => None,
            Projection::Orthographic(o) => Some(o),
        }
    }
}

impl Projection {
    fn matrix(&self) -> glm::Mat4 {
        match &self {
            Projection::Perspective(pers) => {
                glm::perspective(pers.aspect, pers.vertical_fov, pers.near, pers.far)
            }
            Projection::Orthographic(orth) => {
                let half_size = orth.size / 2.0;
                let top_left = -half_size;
                let bottom_right = half_size;

                let top_left = top_left * (1.0 / orth.zoom);
                let bottom_right = bottom_right * (1.0 / orth.zoom);

                glm::ortho(
                    top_left.x,
                    bottom_right.x,
                    bottom_right.y,
                    top_left.y,
                    orth.near,
                    orth.far,
                )
            }
        }
    }
}

impl From<Projection> for glm::Mat4 {
    fn from(p: Projection) -> Self {
        p.matrix()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub translation: glm::Vec3,
    pub orientation: glm::Quat,
    pub projection: Projection,
}

impl Camera {
    pub fn new(projection: Projection) -> Self {
        Self {
            translation: glm::zero(),
            orientation: glm::quat_identity(),
            projection,
        }
    }
    pub fn _with_translation(translation: glm::Vec3, projection: Projection) -> Self {
        Self {
            translation,
            orientation: glm::quat_identity(),
            projection,
        }
    }

    pub fn view_matrix(&self) -> glm::Mat4 {
        let direction = glm::vec3(0.0, 0.0, -1.0);
        let direction = glm::quat_rotate_vec3(&self.orientation, &direction);
        let look_at_p = self.translation + direction;
        glm::look_at(&self.translation, &look_at_p, &glm::vec3(0.0, 1.0, 0.0))
    }

    pub fn view_projection_matrix(&self) -> glm::Mat4 {
        self.projection.matrix() * self.view_matrix()
    }
}
