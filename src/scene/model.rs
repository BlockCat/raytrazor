use ggez::nalgebra::Vector3;

use crate::{material::Material};

use super::scene::Ray;

pub trait SceneModel {
    fn position(&self) -> Vector3<f32>;
    fn collides(&self, ray: Ray) -> Option<RayCollision>;
}

pub struct RayCollision {
    pub original_ray: Ray,
    pub t: f32,
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub material: Material, // not sure
}

pub mod basic {
    use ggez::nalgebra::Vector3;

    use crate::{material::Material, scene::scene::Ray};

    use super::{RayCollision, SceneModel};

    pub struct Circle {
        position: Vector3<f32>,
        radius: f32,
        material: Material,
    }

    impl Circle {
        pub fn new(position: Vector3<f32>, radius: f32, material: Material) -> Self {
            Circle {
                position,
                radius,
                material,
            }
        }
    }

    impl SceneModel for Circle {
        fn position(&self) -> Vector3<f32> {
            self.position
        }

        fn collides(&self, ray: Ray) -> Option<RayCollision> {
            let offset: Vector3<f32> = ray.origin() - self.position;
            let a = ray.direction().dot(&ray.direction());
            let b = 2f32 * Vector3::dot(&offset, &ray.direction());
            let c = Vector3::dot(&offset, &offset) - (self.radius * self.radius);
            let discriminant = b * b - 4f32 * a * c;
            if discriminant < 0f32 {
                return None;
            }

            let discriminant_sqrt = discriminant.sqrt();

            let numerator = -b - discriminant_sqrt;
            if numerator > 0f32 {
                let t = numerator / (2.0 * a);
                let pos: Vector3<f32> = ray.point_at_t(t - 0.0000000001f32);

                let norm: Vector3<f32> = (pos - self.position()).normalize();
                return Some(RayCollision {
                    original_ray: ray.clone(),
                    t,
                    position: pos,
                    normal: norm,
                    material: self.material.clone(),
                });
            }

            let numerator = -b + discriminant_sqrt;
            if numerator > 0f32 {
                let t = numerator / (2.0 * a);
                let pos: Vector3<f32> = ray.point_at_t(t - 0.0000000001f32);

                let norm: Vector3<f32> = (pos - self.position()).normalize();
                return Some(RayCollision {
                    original_ray: ray.clone(),
                    t,
                    position: pos,
                    normal: norm,
                    material: self.material.clone(),
                });
            }

            return None;
        }
    }

    pub struct Plane {
        position: Vector3<f32>,
        normal: Vector3<f32>,
        material: Material,
    }

    impl Plane {
        pub fn new(position: Vector3<f32>, norm1: Vector3<f32>, material: Material) -> Plane {
            Plane {
                position,
                normal: norm1.normalize(),
                material,
            }
        }
    }

    impl SceneModel for Plane {
        fn position(&self) -> Vector3<f32> {
            self.position
        }

        fn collides(&self, ray: Ray) -> Option<RayCollision> {
            // a + x + x2
            let d = Vector3::dot(&ray.direction(), &-self.normal);
            if d > 0.000001 {
                let offset: Vector3<f32> = self.position - ray.origin();
                let t = Vector3::dot(&offset, &-self.normal) / d;
                let position = ray.point_at_t(t - 0.0000000001f32);
                if t >= 0f32 {
                    return Some(RayCollision {
                        original_ray: ray.clone(),
                        t,
                        position,
                        normal: self.normal,
                        material: self.material.clone(),
                    });
                }
            }
            return None;
        }
    }
}
