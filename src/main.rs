use event::KeyCode;
use ggez::{
    conf::WindowMode,
    graphics::{self, Rect},
    nalgebra::Vector3,
    Context, ContextBuilder, GameResult,
};
use ggez::{
    event::{self, EventHandler},
    graphics::Color,
};
use graphics::DrawParam;
use material::Material;
use scene::{camera::Camera, model, scene::Scene};

pub const WIDTH: usize = 640;
pub const HEIGHT: usize = 480;
pub const SHADOW: f32 = 0.05f32;
pub const REFLECTION_DEPTH: usize = 5;
mod material;

mod scene;

fn main() {
    // Make a Context.

    let (mut ctx, mut event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .window_mode(
            WindowMode::default()
                .resizable(false)
                .dimensions(WIDTH as f32, HEIGHT as f32),
        )
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = MyGame::new(&mut ctx);

    // Run!
    event::run(&mut ctx, &mut event_loop, &mut my_game).expect("Uh, could not run game");
}

struct MyGame {
    scene: Scene,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        MyGame {
            scene: basic_scene(),
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::A) {
            self.scene.camera.move_left(0.1f32);
        }

        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::D) {
            self.scene.camera.move_right(0.1f32);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        // Draw code here...

        let image = self.scene.camera.render(
            Rect::new(0f32, 0f32, WIDTH as f32, HEIGHT as f32),
            &self.scene,
            ctx,
        )?;

        graphics::draw(
            ctx,
            &image,
            DrawParam {
                ..DrawParam::default()
            },
        )?;
        graphics::present(ctx)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: event::KeyMods,
        _repeat: bool,
    ) {
        let speed = 0.5f32;
        match keycode {
            KeyCode::Escape => event::quit(ctx),
            KeyCode::W => self.scene.camera.move_forward(speed),
            KeyCode::A => self.scene.camera.move_left(speed),
            KeyCode::S => self.scene.camera.move_back(speed),
            KeyCode::D => self.scene.camera.move_right(speed),
            _ => {}
        }
    }
}

fn basic_scene() -> Scene {
    let camera = Camera::new([10f32, 1f32, 0f32].into(), [-700f32, 0f32, 0f32].into());
    let mut scene = Scene::new(camera);

    scene.add(Box::new(model::basic::Plane::new(
        Vector3::from([0f32, 0f32, 0f32]),
        Vector3::from([0f32, 1f32, 0f32]),
        // Material::color(Color::new(1.9f32, 1.9f32, 1.9f32, 1f32)),
        Material::color_ref(Color::new(0.9f32, 0.9f32, 0.9f32, 1f32), 0.7f32),
    )));

    for i in -3..=-3 {
        for j in 0..4 {
            for k in 0..2 {
                scene.add(Box::new(model::basic::Circle::new(
                    Vector3::from([i as f32 * 10f32, j as f32 * 10f32, 20f32 + k as f32 * 10f32]),
                    5f32,
                    Material::color_ref(Color::new(0.6f32, 0.3f32, 0.0f32, 1f32), 0.5f32),
                )));
            }
        }
    }

    scene.add(Box::new(model::basic::Circle::new(
        Vector3::from([-10f32, 1f32, 0f32]),
        5f32,
        Material::color_ref(Color::new(0.6f32, 0.3f32, 0.0f32, 1f32), 0.9f32),
    )));

    scene.add(Box::new(model::basic::Circle::new(
        Vector3::from([-10f32, 11f32, 0f32]),
        5f32,
        Material::color_ref(Color::new(0.6f32, 0.3f32, 0.0f32, 1f32), 0.9f32),
    )));

    scene.add(Box::new(model::basic::Circle::new(
        Vector3::from([0f32, 1f32, 0f32]),
        1f32,
        Material::color_ref(Color::new(0.3f32, 0.3f32, 0.9f32, 1f32), 0.3f32),
    )));

    scene.add(Box::new(model::basic::Circle::new(
        Vector3::from([0f32, 1f32, -2f32]),
        1f32,
        Material::color_ref(Color::new(0.5f32, 0.9f32, 0.5f32, 1f32), 0.4f32),
    )));

    scene.add(Box::new(model::basic::Circle::new(
        Vector3::from([0f32, 1f32, 2f32]),
        1f32,
        Material::color(Color::new(0.9f32, 0.9f32, 0.5f32, 1f32)),
    )));

    scene
}
