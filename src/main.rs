extern crate ggez;

use ggez::*;
use ggez::graphics::{self, DrawMode, Point2, MeshBuilder};
use ggez::event::MouseButton;

struct MainState {
    pos_x: f32,
    pos_y: f32,
    grid: graphics::Image,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let image = graphics::Image::new(ctx, "/grid.png")?;

        let s = MainState {
            pos_x: 0.0,
            pos_y: 380.0,
            grid: image,
        };

        Ok(s)
    }
}

fn point(x: i32, y: i32) -> Point2 {
    Point2::new(x as f32, y as f32)
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        let mesh = MeshBuilder::new()
            .circle(DrawMode::Fill, Point2::new(self.pos_x, self.pos_y), 100.0, 2.0)
            // horizontal axes of the grid
            .line(&[point(500, 100), point(500, 500)], 1.0)
            .line(&[point(300, 100), point(300, 500)], 1.0)
            // vertical axes of the grid
            .line(&[point(200, 200), point(600, 200)], 1.0)
            .line(&[point(200, 400), point(600, 400)], 1.0)
            .build(ctx)?;

        graphics::draw(ctx, &mesh, Point2::new(0.0, 0.0), 0.0).unwrap();
        graphics::draw(ctx, &self.grid, point(100, 0), 0.0).unwrap();

        graphics::present(ctx);

        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: i32, _y: i32) {
        self.pos_y += 20.0;
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: i32, _y: i32) {
        self.pos_y -= 20.0;
    }
}

fn main() {
    let c = conf::Conf::new();

    let ctx = &mut Context::load_from_conf("tic-tac-toe", "sn99", c).unwrap();

    if let Ok(manifest_fir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_fir);
        path.push("resources");
        ctx.filesystem.mount(&path, true);
    }

    println!("{}", graphics::get_renderer_info(ctx).unwrap());

    let state = &mut MainState::new(ctx).unwrap();

    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered : {}", e);
    } else {
        println!("Game exit cleanly");
    }
}