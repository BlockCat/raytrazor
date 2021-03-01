use ggez::{
    graphics::{Color, Image},
    nalgebra::Vector3,
};

#[derive(Debug, Clone)]
pub enum Materials {
    Color(Color),
    Image(Image),
}

#[derive(Debug, Clone)]
pub struct Material {
    refractive: f32,
    pub reflective: f32,
    map: Materials,
}

impl Material {
    pub fn color(col: Color) -> Material {
        Material {
            refractive: 0f32,
            reflective: 0f32,
            map: Materials::Color(col),
        }
    }
    pub fn color_ref(col: Color, reflective: f32) -> Material {
        Material {
            refractive: 0f32,
            reflective: reflective,
            map: Materials::Color(col),
        }
    }
    pub fn get_color(&self, _position: Vector3<f32>) -> Color {
        match self.map {
            Materials::Color(color) => color.clone(),
            _ => unimplemented!(),
        }
    }
}
