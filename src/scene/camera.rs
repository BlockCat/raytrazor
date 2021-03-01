use ggez::{
    graphics::{Color, Image, Rect},
    nalgebra::Vector3,
    Context, GameResult,
};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

use super::scene::{Ray, Scene};

pub struct Camera {
    position: Vector3<f32>,
    direction: Vector3<f32>,
    up: Vector3<f32>,
}

impl Camera {
    pub fn new(position: Vector3<f32>, direction: Vector3<f32>) -> Camera {
        Camera::new_up(position, direction, Vector3::from([0f32, 1f32, 0f32]))
    }

    pub fn new_up(position: Vector3<f32>, direction: Vector3<f32>, up: Vector3<f32>) -> Camera {
        Camera {
            position,
            direction,
            up: up.normalize(),
        }
    }

    pub fn move_left(&mut self, f: f32) {
        let direction: Vector3<f32> = self.direction;
        let up: Vector3<f32> = self.up.normalize();
        let left: Vector3<f32> = Vector3::cross(&direction, &up).normalize();

        self.position = self.position + left * -f;
    }

    pub fn move_right(&mut self, f: f32) {
        let direction: Vector3<f32> = self.direction;
        let up: Vector3<f32> = self.up.normalize();
        let left: Vector3<f32> = Vector3::cross(&direction, &up).normalize();

        self.position = self.position + left * f;
    }

    pub fn move_forward(&mut self, f: f32) {
        let direction = self.direction.normalize();
        self.position = self.position + direction * f;
    }

    pub fn move_back(&mut self, f: f32) {
        let direction = self.direction.normalize();
        self.position = self.position + direction * -f;
    }

    pub fn render(
        &self,
        mut viewport: Rect,
        scene: &Scene,
        ctx: &mut Context,
    ) -> GameResult<Image> {
        viewport.move_to([-viewport.w / 2f32, -viewport.h / 2f32]);

        let direction: Vector3<f32> = self.direction;
        let up: Vector3<f32> = self.up.normalize();
        let left: Vector3<f32> = Vector3::cross(&direction, &up).normalize();

        println!("dir: {:?}", direction);
        println!("up: {:?}", up);
        println!("lef: {:?}", left);

        let rays = (viewport.y as isize..(viewport.y + viewport.h) as isize)
            .flat_map(|y| {
                (viewport.x as isize..(viewport.x + viewport.w) as isize).map(move |x| (x, -y))
            })
            .map(|(x, y)| {
                let dir: Vector3<f32> = self.direction + (left * x as f32) + (up * y as f32);
                Ray::new(self.position, dir)
            })
            .collect::<Vec<_>>();

        let colors = rays
            .into_par_iter()
            .map(|x| scene.shoot_ray(x))
            .collect::<Vec<_>>();

        let rgba = colors
            .into_iter()
            .flat_map(|x: Color| {
                let (r, g, b, a) = x.to_rgba();
                return vec![r, g, b, a].into_iter();
            })
            .collect::<Vec<u8>>();

        Image::from_rgba8(ctx, viewport.w as u16, viewport.h as u16, &rgba)
    }
}
