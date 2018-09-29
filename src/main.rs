extern crate ggez;

use ggez::*;
use ggez::graphics::{self, DrawMode, Point2};
use ggez::event::MouseButton;

//TODO optimizations and error handling

#[derive(Clone)]
struct Cell {
    option: Option<bool>,
    position: Option<Point2>,
}

impl Cell {
    fn new() -> Self {
        Cell {
            option: None,
            position: None,
        }
    }

    fn make(&mut self, x: i32, y: i32, state: bool) {
        self.position = Some(point(x, y));
        self.option = Some(state);
    }
}

fn winner_declare(winner: i32, ctx: &mut Context) -> GameResult<()> {
    if winner == 1 || winner == 2 {
        println!("Winner is player {}", winner);
    } else {
        println!("It is a draw");
    }

    let exit_image = graphics::Image::new(ctx, "/game_over.png")?;

    graphics::draw(ctx, &exit_image, point(150, 50), 0.0).unwrap();
    graphics::present(ctx);


    std::thread::sleep(std::time::Duration::from_secs(1));

    ctx.quit();

    Ok(())
}

fn check(value: &MainState, ctx: &mut Context) {
    let grid = &value.grid_values;

    // check for horizontal
    for row in grid {
        let mut player_one = 0;
        let mut player_two = 0;
        for column in row {
            match column.option {
                Some(t) => {
                    if t {
                        player_one += 1;
                    } else {
                        player_two += 1;
                    }
                }
                None => continue,
            }
        }
        if player_one == 3 {
            winner_declare(1, ctx);
        } else if player_two == 3 {
            winner_declare(2, ctx);
        } else {
            continue;
        }
    }

    // check for vertical
    for i in 0..3 {
        let mut player_one = 0;
        let mut player_two = 0;
        for j in 0..3 {
            match grid[j][i].option {
                Some(t) => {
                    if t {
                        player_one += 1;
                    } else {
                        player_two += 1;
                    }
                }
                None => continue,
            }
        }
        if player_one == 3 {
            winner_declare(1, ctx);
        } else if player_two == 3 {
            winner_declare(2, ctx);
        } else {
            continue;
        }
    }

    // check for diagonals
    if grid[0][0].option == Some(true) && grid[1][1].option == Some(true) && grid[2][2].option == Some(true) {
        winner_declare(1, ctx);
    }
    if grid[0][0].option == Some(false) && grid[1][1].option == Some(false) && grid[2][2].option == Some(false) {
        winner_declare(2, ctx);
    }

    // check for draw
    let mut draw_check = 0;
    for row in grid {
        for column in row {
            if column.option != None {
                draw_check += 1;
            }
        }
    }
    if draw_check == 9 {
        winner_declare(9, ctx);
    }
}


struct MainState {
    grid: graphics::Image,
    grid_values: Vec<Vec<Cell>>,
    state: bool,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let grid = graphics::Image::new(ctx, "/grid.png")?;
        let grid_values = vec![vec![Cell::new(); 3]; 3];

        let s = MainState {
            grid,
            grid_values,
            state: true,
        };

        Ok(s)
    }
}

fn point(x: i32, y: i32) -> Point2 {
    Point2::new(x as f32, y as f32)
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        for row in &self.grid_values {
            for column in row {
                match column.option {
                    Some(t) => {
                        if t {
                            graphics::circle(ctx, DrawMode::Fill, column.position.unwrap(), 50.0, 1.0)?;
                        } else {
                            graphics::circle(ctx, DrawMode::Line(5.0), column.position.unwrap(), 50.0, 2.0)?;
                        }
                    }

                    None => continue,
                }
            }
        }

        graphics::draw(ctx, &self.grid, point(100, 0), 0.0).unwrap();

        graphics::present(ctx);

        check(self, ctx);

        Ok(())
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, _button: MouseButton, x: i32, y: i32) {
        if y < 200 {
            let y = 100;
            if x < 300 {
                let x = 200;
                self.grid_values[0][0].make(x, y, self.state);
            } else if x <= 500 && x >= 300 {
                let x = 400;
                self.grid_values[0][1].make(x, y, self.state);
            } else {
                let x = 600;
                self.grid_values[0][2].make(x, y, self.state);
            }
        } else if y >= 200 && y <= 400 {
            let y = 300;
            if x < 300 {
                let x = 200;
                self.grid_values[1][0].make(x, y, self.state);
            } else if x <= 500 && x >= 300 {
                let x = 400;
                self.grid_values[1][1].make(x, y, self.state);
            } else {
                let x = 600;
                self.grid_values[1][2].make(x, y, self.state);
            }
        } else {
            let y = 500;
            if x < 300 {
                let x = 200;
                self.grid_values[2][0].make(x, y, self.state);
            } else if x <= 500 && x >= 300 {
                let x = 400;
                self.grid_values[2][1].make(x, y, self.state);
            } else {
                let x = 600;
                self.grid_values[2][2].make(x, y, self.state);
            }
        }
        self.state = !self.state;
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