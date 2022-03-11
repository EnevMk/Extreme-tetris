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
use rand::Rng;
//use assets::*;

#[derive(Clone)]
pub struct Assets {

    pub orange: graphics::Image,
    pub black: graphics::Image,
    pub red: graphics::Image,
    pub purple: graphics::Image,
    pub blue: graphics::Image,
    pub green: graphics::Image,
    pub yellow: graphics::Image,
    pub cyan: graphics::Image,
    
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {

        let orange = graphics::Image::new(ctx, "\\orange.png")?;
        let black  = graphics::Image::new(ctx, "\\field.png")?;
        let red    = graphics::Image::new(ctx, "\\red.png")?;
        let cyan   = graphics::Image::new(ctx, "\\cyan.png")?;
        let purple = graphics::Image::new(ctx, "\\purple.png")?;
        let blue   = graphics::Image::new(ctx, "\\blue.png")?;
        let green  = graphics::Image::new(ctx, "\\green.png")?;
        let yellow = graphics::Image::new(ctx, "\\yellow.png")?;

        Ok(Assets {orange, black, red, cyan, purple, blue, green, yellow})
    }
}

#[derive(Clone, PartialEq)]
enum FigureType {

    L, J, O, S, Z, I, T, Bomb
}

#[derive(Clone)]
struct Figure {
    kind: FigureType,
    shape: [[u8; 4]; 4],
    color: graphics::Image,
}

impl Figure {

    pub fn new(kind_: FigureType, assets: &Assets) -> Figure {

        match kind_ {

            FigureType::I => Figure {
                            kind: kind_,

                            shape: [[ 0, 1, 0, 0 ],
                                    [ 0, 1, 0, 0 ],
                                    [ 0, 1, 0, 0 ],
                                    [ 0, 1, 0, 0 ]],
                            
                            color: assets.orange.clone()},

            FigureType::L => Figure {
                            kind: kind_,
                            shape: [[ 0, 0, 1, 0 ],
                                    [ 1, 1, 1, 0 ],
                                    [ 0, 0, 0, 0 ],
                                    [ 0, 0, 0, 0 ]],
                            
                            color: assets.blue.clone()},

            FigureType::T => Figure {
                            kind: kind_,
                            shape: [[ 0, 1, 0, 0 ],
                                    [ 1, 1, 1, 0 ],
                                    [ 0, 0, 0, 0 ],
                                    [ 0, 0, 0, 0 ]],

                            color: assets.purple.clone()},
            FigureType::J => Figure {
                            kind: kind_,
                            shape: [[ 1, 0, 0, 0 ],
                                    [ 1, 1, 1, 0 ],
                                    [ 0, 0, 0, 0 ],
                                    [ 0, 0, 0, 0 ]],

                            color: assets.yellow.clone()},
            FigureType::S => Figure {
                            kind: kind_,
                            shape: [[ 0, 1, 1, 0 ],
                                    [ 1, 1, 0, 0 ],
                                    [ 0, 0, 0, 0 ],
                                    [ 0, 0, 0, 0 ]],

                            color: assets.cyan.clone()},
            FigureType::Z => Figure {
                            kind: kind_,
                            shape: [[ 1, 1, 0, 0 ],
                                    [ 0, 1, 1, 0 ],
                                    [ 0, 0, 0, 0 ],
                                    [ 0, 0, 0, 0 ]],

                            color: assets.green.clone()},
            FigureType::O => Figure {
                            kind: kind_,
                            shape: [[ 0, 1, 1, 0 ],
                                    [ 0, 1, 1, 0 ],
                                    [ 0, 0, 0, 0 ],
                                    [ 0, 0, 0, 0 ]],

                            color: assets.red.clone()},
            
            FigureType::Bomb => Figure {

                            kind: kind_,
                            shape: [[ 0, 1, 0, 0 ],
                                    [ 0, 0, 0, 0 ],
                                    [ 0, 0, 0, 0 ],
                                    [ 0, 0, 0, 0 ]],
                                    
                            color: assets.red.clone()}
        }
    }

    pub fn is_first_column_zero(&self) -> bool {

        for col in 0..4 {
            if self.shape[col][0] == 1 {return false;}
        }

        true
    }

    fn left_shift(&mut self) {

        for row in 0..4 {

            for col in 0..3 {

                self.shape[row][col] = self.shape[row][col + 1];
            }
        }

        for row in 0..4 {

            self.shape[row][3] = 0;
        }
    }

    pub fn draw(&self, ctx: &mut Context, col: u8, row: u8) -> GameResult<()> {

        for i in 0..4 {

            for j in 0..4 {

                let x_coord : f32 = 35.0 * (i + row) as f32;
                let y_coord : f32 = 35.0 * (j + col) as f32;

                if self.shape[j as usize][i as usize] == 1 {
                    
                    let draw_params = graphics::DrawParam::default().
                                    dest(Point2::<f32>{x: x_coord, y: y_coord}).
                                    scale(Vector2 { x: 1.75, y: 1.75 });
                                    //offset(Point2 { x: 0.5, y: 1.0 });
                    graphics::draw(ctx, &self.color, draw_params)?;
                }
            }
        }
        Ok(())
    }
}

type Field = [[u8; 10]; 20];

#[derive(Clone)]
struct GameState {

    frames_until_fall: u8,
    field: Field,
    figures: Vec<FigureType>,
    current_figure: Figure,
    next_figure: Figure,
    //box_w_h: f32,
    col: u8,
    row: u8,
    assets: Assets,

    game_over: bool,
    score: u16,
    level: u8,
    current_level_fps: u32,

    BOTTOM_LINE_INDEX: u8,
    RIGHTEST_INDEX: u8,
}

impl GameState {

    fn new(ctx: &mut Context, _conf: &Conf) -> GameResult<GameState> {

        let mut rng = rand::thread_rng();
        let rand_index = rng.gen_range(0, 8);

        let assets = Assets::new(ctx)?;

        let mut gs = GameState {
            frames_until_fall: 20,
            field: [[0; 10] ; 20],
            current_figure: Figure::new(FigureType::L, &assets),
            next_figure: Figure::new(FigureType::L, &assets),
            //current_figure: Figure::new(self.figures[rand_index].clone(), &self.assets),
            col: 0,
            row: 4,
            assets: assets,
            figures: vec![FigureType::I, FigureType::J, FigureType::L, FigureType::O, FigureType::S, FigureType::T, FigureType::Z, FigureType::Bomb],

            game_over: false,
            score: 0,
            level: 1,
            current_level_fps: 30,

            BOTTOM_LINE_INDEX: 19,
            RIGHTEST_INDEX: 9
        };
        gs.current_figure = Figure::new(gs.figures[rand_index].clone(), &gs.assets);
        
        let rand_index = rng.gen_range(0, 8);
        gs.next_figure = Figure::new(gs.figures[rand_index].clone(), &gs.assets);
        Ok(gs)
    }

    fn move_left(&mut self) {
        if !self.collide_at_sides(-1) {

            if self.row == 0 && self.current_figure.is_first_column_zero() {
                self.current_figure.left_shift();
            }
            else { 
                self.row -= 1;
            }
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

                if self.current_figure.shape[j as usize][i as usize] != 0 {
                    
                    if j + self.col <= self.BOTTOM_LINE_INDEX && i + self.row < 10 {

                        if j + self.col == 19
                          || self.field[(j + self.col + 1) as usize][(i + self.row) as usize] != 0
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
        //println!("colision check");
        for i in 0..4 {

            for j in 0..4 {

                if self.current_figure.shape[j as usize][i as usize] != 0 {
                    
                    if (i + self.row) as i8 + dir > 9 || (i + self.row) as i8 + dir < 0 {
                        return true;
                    }

                    else if (i + self.row) as i8 + dir <= 9 && (i + self.row) as i8 + dir >= 0
                            && self.field[(j + self.col) as usize][((i + self.row) as i8 + dir) as usize] != 0
                    {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn bomb_collision(&mut self) {
        
        for i in 0..4 {
            for j in 0..4 {
                // randomly destroying three boxes below the bomb figure
                if self.current_figure.shape[j][i] == 1 {

                    for x in (i as i8 - 1)..(i+2) as i8 { // strange looking range needed for validation
                        let mut rng = rand::thread_rng();
                        let will_destroy_box = rng.gen_range(0, 2);
                        
                        if will_destroy_box == 1 
                        && (self.col + j as u8) < self.BOTTOM_LINE_INDEX
                        && x + self.row as i8 <= self.RIGHTEST_INDEX as i8 && x >= 0 {

                            self.field[j + 1 + self.col as usize][x as usize + self.row as usize] = 0;
                        }
                    }
                }

            }
        }
    }

    fn fix_figure_to_field(&mut self) -> () {

        if self.current_figure.kind == FigureType::Bomb {
            //println!("Bomb colision");
            self.bomb_collision()
        }

        for i in 0..4 {
            for j in 0..4 {

                if self.current_figure.shape[j as usize][i as usize] == 1 {
                    
                    if j + self.col <= self.BOTTOM_LINE_INDEX && i + self.row < 10 {
                        self.field[(j + self.col) as usize][(i + self.row) as usize] = match self.current_figure.kind {

                            //FigureType::Bomb => 0,
                            FigureType::I => 1,
                            FigureType::J => 2,
                            FigureType::L => 3,
                            FigureType::O => 4,
                            FigureType::S => 5,
                            FigureType::T => 6,
                            FigureType::Z => 7,
                            _             => 0
                        };
                    }
                }
            }
        }
    }

    pub fn rotate_figure(&mut self) {

        let dim = match self.current_figure.kind {
            FigureType::I => 4,
            _             => 3
        };

        let mut new_shape : [[u8; 4]; 4] = self.current_figure.shape;

        for row in 0..dim {
            let mut ind = dim - 1;
            for col in 0..dim {
                
                //println!("{}, {}", self.col, col);
                if col + self.col > self.BOTTOM_LINE_INDEX || row + self.row >= 10 
                    || self.field[(col + self.col) as usize][(row + self.row) as usize] != 0 {return;}

                new_shape[row as usize][col as usize] = self.current_figure.shape[ind as usize][row as usize];
                if ind > 0 {ind -= 1;}
            }
        }

        self.current_figure.shape = new_shape;
    }

    fn hard_drop(&mut self) {

        if !self.figure_collides() {
            self.col += 1;
        }
    }

    fn match_color_code(&self, code: u8) -> &graphics::Image {

        match code {
            1 => &self.assets.orange,
            2 => &self.assets.yellow,
            3 => &self.assets.blue,
            4 => &self.assets.red,
            5 => &self.assets.cyan,
            6 => &self.assets.purple,
            7 => &self.assets.green,
            _ => &self.assets.black
        }
    }

    fn clear_complete_rows(&mut self) -> u8 {

        let mut new_field : Field = [[0; 10]; 20];
        let mut new_field_row = 19;
        let mut rows_cleared : u8 = 0;

        for row in (0..20).rev() {

            let mut slots_count = 0;

            for col in 0..10 {
                if self.field[row][col] != 0 { slots_count += 1; }
            }

            if slots_count == 10 {
                rows_cleared += 1;
                continue;
            }

            if self.field[row].iter().sum::<u8>() > 0 {
                new_field[new_field_row] = self.field[row];
                new_field_row -= 1;
            }
        }

        self.field = new_field;
        rows_cleared
    }

    fn update_score(&mut self, rows_cleared: u16) {
        self.score += rows_cleared * 80 * self.level as u16;
    }

    fn check_level_update(&mut self) {

        if (self.score / self.level as u16) >= 1000 * self.level as u16 { 
            self.current_level_fps += (0.4 * self.current_level_fps as f32) as u32;
            self.level += 1
        }
    }

    fn display_score(&mut self, dest: Point2<f32>, ctx: &mut Context) -> GameResult {

        let text: String = "Score: ".to_string() + &self.score.to_string();
        let text_graphic = graphics::Text::new(text);

        graphics::draw(ctx, &text_graphic, graphics::DrawParam::default()
                        .dest(dest)
                        .scale(Vector2 { x: 2.0, y: 2.0 }))
    }

    fn draw_next_figure(&mut self, ctx: &mut Context) -> GameResult {

        self.next_figure.draw(ctx, 4, 15)
    }
}

impl ggez::event::EventHandler<GameError> for GameState {

    fn update(&mut self, ctx: &mut Context) -> GameResult {

        let fps;
        if self.current_figure.kind == FigureType::Bomb { fps = 260; }
        else {
            fps = self.current_level_fps;
        }

        while timer::check_update_time(ctx, fps) && !self.game_over {
            if self.frames_until_fall == 0 {

                if self.col <= self.BOTTOM_LINE_INDEX {

                    if self.figure_collides() {

                        self.fix_figure_to_field();
                        if self.col == 0 {
                            self.game_over = true;
                            break;
                        }
                        let rows_cleared = self.clear_complete_rows();
                        self.update_score(rows_cleared as u16);
                        self.check_level_update();

                        let mut rng = rand::thread_rng();
                        let rand_index = rng.gen_range(0, 8);

                        self.current_figure = self.next_figure.clone();
                        self.next_figure = Figure::new(self.figures[rand_index].clone(), &self.assets);

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
                    graphics::draw(ctx, self.match_color_code(self.field[j][i]), draw_params)?;
                }
            }
        }

        self.current_figure.draw(ctx, self.col, self.row)?;
        self.display_score(Point2::<f32>{x: 500.0, y: 100.0}, ctx)?;
        self.draw_next_figure(ctx)?;

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

            KeyCode::Left   => self.move_left(),
            KeyCode::Right  => self.move_right(),
            KeyCode::Up     => self.rotate_figure(),
            KeyCode::Down   => self.hard_drop(),
            KeyCode::Escape => event::quit(ctx),
            _               => ()
        }    
    }
}

fn main() {
    
    let conf = Conf::new()
            .window_mode(WindowMode{
                width: 1000.0,
                height: 700.0,
                ..Default::default()
            });

    let (mut ctx, event_loop) = ContextBuilder::new("extreme-tetris", "Mihail")
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
