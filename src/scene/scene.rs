use std::cmp::Ordering;

use ggez::{graphics::Color, nalgebra::Vector3};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{REFLECTION_DEPTH, SHADOW, material::Material};

use super::{
    camera::Camera,
    model::{RayCollision, SceneModel},
};

pub struct Scene {
    pub camera: Camera,
    models: Vec<Box<dyn SceneModel + Sync>>,
    light: (Vector3<f32>, Color),
}

impl Scene {
    pub fn new(camera: Camera) -> Scene {
        Scene {
            models: Vec::new(),
            camera,
            light: (
                [0f32, 10f32, 5f32].into(),
                Color::new(100f32, 100f32, 100f32, 1f32),
            ),
        }
    }

    pub fn add(&mut self, model: Box<dyn SceneModel + Sync>) {
        self.models.push(model);
    }

    fn apply_light(&self, color: Color, position: Vector3<f32>) -> Color {
        let light = self.light.1;
        let ehm = (position - self.light.0).magnitude_squared();
        let light = multiply_colour(&light, 1f32 / ehm);
        multiply_colours(&color, &light)
        
    }

    fn apply_shading(&self, color: Color, normal: Vector3<f32>, shadow_ray: Vector3<f32>) -> Color {
        multiply_colour(
            &color,
            Vector3::dot(&shadow_ray, &normal.normalize())
                .max(0f32)
                .min(1f32),
        )
    }

    fn apply_mirror(&self, color: Color, collision: RayCollision, max_depth: usize) -> Color {
        let reflection = collision.material.reflective;
        if max_depth == 0 {
            return color;
        }
        if reflection <= 0.00001 || max_depth == 0 {
            return color;   
        }

        // println!("Mirror");
        let reflection_ray = collision.original_ray.mirror(
            collision.normal,
            collision.original_ray.point_at_t(collision.t - 0.0001f32),
        );

        // println!("ray: {:?}, normal: {:?}, mirror: {:?}", collision.original_ray.direction(), collision.normal, reflection_ray.direction());
        
        

        let reflection_color = self.shoot_ray_depth(reflection_ray, max_depth - 1);

        // return reflection_color;
        let reflection_color = multiply_colour(&reflection_color, reflection);
        let normal_color = multiply_colour(&color, 1.0f32 - reflection);

        Color::new(
            reflection_color.r + normal_color.r,
            reflection_color.g + normal_color.g,
            reflection_color.b + normal_color.b,
            reflection_color.a + normal_color.a,
        )
    }

    pub fn shoot_ray_depth(&self, ray: Ray, max_depth: usize) -> Color {
        let collision = self
            .models
            .iter()
            .filter_map(|x| x.collides(ray.clone()))
            .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(Ordering::Equal));

        if let Some(collision) = collision {
            let material_color = collision.material.get_color(collision.position);
            let shadow_ray: Vector3<f32> = (self.light.0 - collision.position).normalize();

            let mut color = self.apply_light(material_color, collision.position);
            color = self.apply_shading(color, collision.normal.normalize(), shadow_ray);

            if self.shadow_ray(Ray::new(
                collision.position + shadow_ray * 0.0001f32,
                shadow_ray,
            )) {
                color = multiply_colour(&color, SHADOW);
            }

            color = self.apply_mirror(color, collision, max_depth);

            color
        } else {
            Color::new(0.2f32, 0.2f32, 0.4f32, 1f32)
        }
    }

    pub fn shoot_ray(&self, ray: Ray) -> Color {
        self.shoot_ray_depth(ray, REFLECTION_DEPTH)
    }

    // Check if there is a collision
    fn shadow_ray(&self, ray: Ray) -> bool {
        self.models
            .par_iter()
            .filter_map(|x| x.collides(ray.clone()))
            .any(|t| t.t > 0f32)
    }
}

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
}

impl Ray {
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Ray {
        Ray {
            origin,
            direction: direction.normalize(),
        }
    }
    pub fn mirror(&self, normal: Vector3<f32>, position: Vector3<f32>) -> Ray {
        let normal = normal.normalize();
        let direction: Vector3<f32> =
        self.direction - (2f32 * normal.dot(&self.direction)) * normal;

        Ray {
            origin: position,
            direction: direction.normalize(),
        }
    }
    pub fn direction(&self) -> Vector3<f32> {
        self.direction
    }
    pub fn origin(&self) -> Vector3<f32> {
        self.origin
    }
    pub fn point_at_t(&self, t: f32) -> Vector3<f32> {
        self.origin + self.direction * t
    }
}

fn multiply_colour(color: &Color, m: f32) -> Color {
    Color::new(color.r * m, color.g * m, color.b * m, color.a)
}
fn multiply_colours(color1: &Color, color2: &Color) -> Color {
    Color::new(
        color1.r * color2.r,
        color1.g * color2.g,
        color1.b * color2.b,
        color1.a * color2.a,
    )
}

fn add_colours(color1: &Color, color2: &Color) -> Color {
    Color::new(
        color1.r + color2.r,
        color1.g + color2.g,
        color1.b + color2.b,
        color1.a + color2.a,
    )
}
