use ggez::graphics;
use ggez::{Context, ContextBuilder, GameResult, GameError};
use ggez::conf::{Conf, WindowMode};
use ggez::mint::Point2;
use ggez::mint::Vector2;
use ggez::event::{KeyCode, KeyMods};
use ggez::event;
use ggez::filesystem;
use ggez::timer;
use std::env;
use std::path;
//use assets::*;

#[derive(Clone)]
enum FigureType {

    L, J, O, S, Z, I, T
}

#[derive(Clone)]
struct Figure {

    kind: FigureType,
    /* col: u8,
    row: u8, */
    shape: [[u8; 4]; 4],
}

impl Figure {

    pub fn new(kind_: FigureType) -> Figure {

        match kind_ {

            FigureType::I => Figure {
                            kind: kind_,

                            shape: [[ 0, 1, 0, 0 ],
                                    [ 0, 1, 0, 0 ],
                                    [ 0, 1, 0, 0 ],
                                    [ 0, 1, 0, 0 ]] },
            FigureType::L => Figure {
                            kind: kind_,

                            shape: [[ 0, 0, 1, 0 ],
                                    [ 1, 1, 1, 0 ],
                                    [ 0, 0, 0, 0 ],
                                    [ 0, 0, 0, 0 ]] },

            FigureType::T => Figure {
                            kind: kind_,

                            shape: [[ 0, 1, 0, 0 ],
                                    [ 1, 1, 1, 0 ],
                                    [ 0, 0, 0, 0 ],
                                    [ 0, 0, 0, 0 ]] },
            FigureType::J => Figure {
                            kind: kind_,

                            shape: [[ 1, 0, 0, 0 ],
                                    [ 1, 1, 1, 0 ],
                                    [ 0, 0, 0, 0 ],
                                    [ 0, 0, 0, 0 ]] },
            FigureType::S => Figure {
                            kind: kind_,

                            shape: [[ 0, 1, 1, 0 ],
                                    [ 1, 1, 0, 0 ],
                                    [ 0, 0, 0, 0 ],
                                    [ 0, 0, 0, 0 ]] },
            FigureType::Z => Figure {
                            kind: kind_,

                            shape: [[ 1, 1, 0, 0 ],
                                    [ 0, 1, 1, 0 ],
                                    [ 0, 0, 0, 0 ],
                                    [ 0, 0, 0, 0 ]] },
            FigureType::O => Figure {
                            kind: kind_,

                            shape: [[ 0, 1, 1, 0 ],
                                    [ 0, 1, 1, 0 ],
                                    [ 0, 0, 0, 0 ],
                                    [ 0, 0, 0, 0 ]] }, 
        }
    }

    /* pub fn rotate(&mut self) {

        let dim = match self.kind {
            FigureType::I => 4,
            _             => 3
        };

        for row in 0..dim {

            for col in 0..dim {


            }
        }
    } */

    pub fn draw(&self, ctx: &mut Context, assets: &Assets, col: u8, row: u8) -> GameResult<()> {

        let x_coord : f32 = 35.0 * row as f32;
        let y_coord : f32 = 35.0 * col as f32;
        
        let draw_params = graphics::DrawParam::default().
                    dest(Point2::<f32>{x: x_coord, y: y_coord}).
                    offset(Point2 { x: 0.5, y: 1.0 });
                graphics::draw(ctx, &assets.orange, draw_params)?;

        Ok(())
    }
}

#[derive(Clone)]
pub struct Assets {

    pub orange: graphics::Image,
    pub black: graphics::Image,
    pub red: graphics::Image,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {

        let orange = graphics::Image::new(ctx, "\\l.png")?;
        let black = graphics::Image::new(ctx, "\\field.png")?;
        let red = graphics::Image::new(ctx, "\\z.png")?;
        Ok(Assets {orange, black, red})
    }
}

type Field = [[u8; 10]; 20];

#[derive(Clone)]
struct GameState {

    frames_until_fall: u8,
    field: Field,
    //figures: Vec<FigureType>,
    current_figure: Figure,
    //next_figure: Figure,
    screen_width: f32,
    screen_height: f32,
    //box_w_h: f32,
    col: u8,
    row: u8,
    assets: Assets,
}

impl GameState {

    fn new(ctx: &mut Context, conf: &Conf) -> GameResult<GameState> {

        let assets = Assets::new(ctx)?;

        let gs = GameState {
            frames_until_fall: 20,
            field: [[0; 10] ; 20],
            screen_width: 350.0,
            screen_height: 700.0,
            current_figure: Figure::new(FigureType::L),
            col: 7,
            row: 7,
            assets: assets,
        };

        Ok(gs)
    }

    fn move_left(&mut self) {
        if !self.collide_at_sides(-1) {
            self.row -= 1;
        }
    }

    fn move_right(&mut self) {
        if !self.collide_at_sides(1) {
            self.row += 1;
        }
    }

    fn figure_collides(&self) -> bool {

        for i in 0..4 {

            for j in 0..4 {

                if self.current_figure.shape[j as usize][i as usize] == 1 {
                    
                    if j + self.col + 1 <= 20 && i + self.row < 10 {

                        if j + self.col == 19
                          || self.field[(j + self.col + 1) as usize][(i + self.row) as usize] == 1
                        {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }

    fn collide_at_sides(&self, dir: i8) -> bool {

        for i in 0..4 {

            for j in 0..4 {

                if self.current_figure.shape[j as usize][i as usize] == 1 {
                    
                    if (i + self.row) as i8 + dir > 9 || (i + self.row) as i8 + dir < 0 {
                        return true;
                    }

                    else if (i + self.row) as i8 + dir <= 9 && (i + self.row) as i8 + dir >= 0
                            && self.field[(j + self.col) as usize][((i + self.row) as i8 + dir) as usize] == 1
                    {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn fix_figure_to_field(&mut self) -> () {

        for i in 0..4 {

            for j in 0..4 {

                let x_coord : f32 = 35.0 * (i + self.row) as f32;
                let y_coord : f32 = 35.0 * (j + self.col) as f32;

                if self.current_figure.shape[j as usize][i as usize] == 1 {
                    
                    if j + self.col < 20 && i + self.row < 10 {
                        self.field[(j + self.col) as usize][(i + self.row) as usize] = 1;
                    }
                    
                }
            }
        }
    }
}

impl ggez::event::EventHandler<GameError> for GameState {

    fn update(&mut self, ctx: &mut Context) -> GameResult {

        const DESIRED_FPS: u32 = 30;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            if self.frames_until_fall == 0 {

                //if self.col == 18 {return Ok(())}
                if self.col < 20 {
                    

                    if self.figure_collides() {

                        self.fix_figure_to_field();
                        self.current_figure = Figure::new(FigureType::T);
                        self.col = 0;
                        self.row = 4;
                        return Ok(());
                    }
                    else {
                        self.col += 1;
                    }
                }
                self.frames_until_fall = 20;
                
            }
            else if self.frames_until_fall > 0 {
                
                self.frames_until_fall -= 1;
                continue;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let dark_blue = graphics::Color::from_rgb(26, 51, 77);
        graphics::clear(ctx, dark_blue);
        
        self.current_figure.draw(ctx, &self.assets, self.col, self.row)?;

        for i in 0..10 {

            for j in 0..20 {

                let x_coord : f32 = 35.0 * i as f32;
                let y_coord : f32 = 35.0 * j as f32;

                if self.field[j][i] == 0 {
                    let draw_params = graphics::DrawParam::default().
                                    dest(Point2::<f32>{x: x_coord, y: y_coord}).
                                    scale(Vector2 { x: 1.75, y: 1.75 });
                                    //offset(Point2 { x: 0.5, y: 1.0 });
                    graphics::draw(ctx, &self.assets.black, draw_params)?;
                }
                else {
                    let draw_params = graphics::DrawParam::default().
                                    dest(Point2::<f32>{x: x_coord, y: y_coord}).
                                    scale(Vector2 { x: 1.75, y: 1.75 });
                                    //offset(Point2 { x: 0.5, y: 1.0 });
                    graphics::draw(ctx, &self.assets.red, draw_params)?;
                }
            }
        }

        for i in 0..4 {

            for j in 0..4 {

                let x_coord : f32 = 35.0 * (i + self.row) as f32;
                let y_coord : f32 = 35.0 * (j + self.col) as f32;

                if self.current_figure.shape[j as usize][i as usize] == 1 {
                    
                    let draw_params = graphics::DrawParam::default().
                                    dest(Point2::<f32>{x: x_coord, y: y_coord}).
                                    scale(Vector2 { x: 1.75, y: 1.75 });
                                    //offset(Point2 { x: 0.5, y: 1.0 });
                    graphics::draw(ctx, &self.assets.orange, draw_params)?;
                }
            }
        }
        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, 
        ctx: &mut Context, 
        keycode: KeyCode, 
        _keymod: KeyMods, 
        _repeat: bool)
    {
        match keycode {

            KeyCode::Left  => self.move_left(),
            KeyCode::Right => self.move_right(),
            _              => ()
        }    
    }
}

fn main() {
    //println!("Hello, world!");

    let conf = Conf::new()
            .window_mode(WindowMode{
                width: 1000.0,
                height: 700.0,
                ..Default::default()
            });

    let (mut ctx, event_loop) = ContextBuilder::new("hello_ggez", "awesome_person")
                            .default_conf(conf.clone())
                            .build()
                            .unwrap();

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        filesystem::mount(&mut ctx, &path, true);
    }

    let state = GameState::new(&mut ctx, &conf).unwrap();

    event::run(ctx, event_loop, state);
}