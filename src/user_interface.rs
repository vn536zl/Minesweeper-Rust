extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::fs::File;
use std::io::Write;

use graphics::glyph_cache::rusttype::GlyphCache as Cache;
use graphics::rectangle::Border;
use graphics::*;

use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, Texture, TextureSettings};

use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, ButtonEvent, ButtonState, MouseButton, MouseCursorEvent, RenderEvent};
use piston::window::WindowSettings;

use glutin_window::GlutinWindow as Window;
use piston::{ButtonArgs, Key, RenderArgs, UpdateEvent};

use crate::minesweeper;
use crate::minesweeper::MinesweeperBoard;

const PIXEL_SIZE: f64 = 32.0;
const FONT: &[u8] = include_bytes!("mine-sweeper.ttf");

pub struct GUI<'a> {
    board: MinesweeperBoard,
    start_mine_count: i32,
    current_mine_count: i32,
    height: i32,
    width: i32,
    tiles_to_win: i32,
    world_size: [f64; 2],
    game_result: i32,
    hit_mine: [i32; 2],
    mouse_pos: [i32; 2],
    cache: Cache<'a, (), Texture>,
    window: Window,
    gl: GlGraphics,
}

impl<'a> GUI<'a> {
    pub fn new(board: MinesweeperBoard, mine_count: i32) -> Self {
        // Board information
        let height = board.len() as i32;
        let width = board[0].len() as i32;
        let tiles_to_win = (height * width) - mine_count;

        // Graphical info
        let world_size = [
            (PIXEL_SIZE * width as f64),
            (PIXEL_SIZE * height as f64) + (PIXEL_SIZE * 2.0),
        ];

        let opengl = OpenGL::V3_2;
        let window: Window = WindowSettings::new("Minesweeper", world_size)
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();
        let gl = GlGraphics::new(opengl);

        let exe_path = std::env::current_dir().unwrap();
        let path = exe_path.to_str().unwrap().to_owned() + "\\Font.ttf";
        let mut f = File::create(&path).unwrap();
        f.write_all(FONT).expect("No File Written");
        let cache = GlyphCache::new(path, (), TextureSettings::new()).unwrap();

        GUI {
            board,
            start_mine_count: mine_count,
            current_mine_count: mine_count,
            height,
            width,
            tiles_to_win,
            world_size,
            game_result: 0,
            hit_mine: [-1, -1],
            mouse_pos: [0, 0],
            cache,
            window,
            gl,
        }
    }

    fn mouse_update(&mut self, m: [f64; 2]) {
        self.mouse_pos = [
            (m[0] / PIXEL_SIZE) as i32,
            ((m[1] - PIXEL_SIZE * 2.0) / PIXEL_SIZE) as i32,
        ];
    }

    fn button_press(&mut self, b: ButtonArgs) {
        if b.state == ButtonState::Press {
            match b.button {
                Button::Mouse(MouseButton::Left) => {
                    if self.game_result == 0 {
                        let reveal_result = minesweeper::reveal_tile(
                            &mut self.board,
                            self.mouse_pos[0],
                            self.mouse_pos[1],
                        );
                        if reveal_result == 1 {
                            self.hit_mine = self.mouse_pos;
                            self.game_result = 1;
                        } else if self.board[self.mouse_pos[1] as usize][self.mouse_pos[0] as usize]
                            .get_num()
                            != 0
                        {
                            match minesweeper::cord_tile(
                                &mut self.board,
                                self.mouse_pos[0],
                                self.mouse_pos[1],
                            ) {
                                Ok(_r) => {}
                                Err(c) => {
                                    if c != [-1, -1] {
                                        self.hit_mine = c;
                                        self.game_result = 1;
                                    }
                                }
                            }
                        }
                    }
                }
                Button::Mouse(MouseButton::Right) => {
                    if self.game_result == 0 {
                        self.current_mine_count = minesweeper::flag_tile(
                            &mut self.board,
                            self.mouse_pos[0],
                            self.mouse_pos[1],
                            self.current_mine_count,
                        );
                    }
                }
                Button::Keyboard(Key::LCtrl) => {
                    if self.game_result == 0 {
                        self.current_mine_count = minesweeper::flag_tile(
                            &mut self.board,
                            self.mouse_pos[0],
                            self.mouse_pos[1],
                            self.current_mine_count,
                        );
                    }
                }
                Button::Keyboard(Key::R) => {
                    self.game_result = 0;
                    self.current_mine_count = self.start_mine_count;
                    self.board = minesweeper::build_minesweeper_board(
                        self.height,
                        self.width,
                        self.current_mine_count,
                    );
                }
                _ => {}
            }
        }
    }

    fn render(&mut self, args: RenderArgs) {
        self.gl.draw(args.viewport(), |c, gl| {
            clear(color::WHITE, gl);

            let rect = Rectangle::new(color::grey(0.73)).border(Border {
                color: color::WHITE,
                radius: 1.5,
            });
            let dims = [0.0, 0.0, self.world_size[0], PIXEL_SIZE * 2.0];

            rect.draw(dims, &c.draw_state, c.transform, gl);

            let temp = self.current_mine_count.to_string();
            let mine_num_string = temp.as_str();

            let (mine_count_width, mine_count_height) =
                get_text_size(&mut self.cache, mine_num_string, 64);

            let width_offset = ((self.world_size[0] / 3.0) - mine_count_width) / 2.0;
            let height_offset = ((PIXEL_SIZE * 2.0) - mine_count_height) / 2.0;

            Text::new_color(color::RED, 64)
                .draw(
                    mine_num_string,
                    &mut self.cache,
                    &c.draw_state,
                    c.transform
                        .trans(width_offset, (PIXEL_SIZE * 2.0) - height_offset),
                    gl,
                )
                .unwrap();

            for j in 0..self.height {
                for i in 0..self.width {
                    let mut tile_color = color::grey(0.73);
                    let mut border_color = color::WHITE;
                    let mut border_radius = 1.5;

                    let pos_x = i as f64 * PIXEL_SIZE;
                    let pos_y = (2 + j) as f64 * PIXEL_SIZE;
                    let current_tile = self.board[j as usize][i as usize];

                    if current_tile.is_revealed() {
                        border_color = color::grey(0.48);
                        border_radius = 0.5;
                    } else if current_tile.is_flagged() {
                        tile_color = color::LIME;
                        border_color = color::GREEN;
                        border_radius = 1.0;
                    }

                    if current_tile.has_mine() && self.game_result == 1 {
                        if [i, j] == self.hit_mine {
                            tile_color = color::RED
                        } else {
                            tile_color = color::BLACK;
                        }
                        border_color = color::RED;
                        border_radius = 1.0;
                    }

                    let rect = Rectangle::new(tile_color).border(Border {
                        color: border_color,
                        radius: border_radius,
                    });
                    let dims = rectangle::square(0.0, 0.0, PIXEL_SIZE);

                    rect.draw(dims, &c.draw_state, c.transform.trans(pos_x, pos_y), gl);
                }
            }
            for j in 0..self.height {
                for i in 0..self.width {
                    let current_tile = self.board[j as usize][i as usize];

                    if current_tile.is_revealed() {
                        let tile_number = current_tile.get_num().to_string();
                        let number_color = match tile_number.as_str() {
                            "1" => color::BLUE,
                            "2" => color::GREEN,
                            "3" => color::RED,
                            "4" => color::NAVY,
                            "5" => color::MAROON,
                            "6" => color::CYAN,
                            "7" => color::PURPLE,
                            "8" => color::grey(0.48),
                            _ => color::grey(0.73),
                        };

                        let (num_width, num_height) =
                            get_text_size(&mut self.cache, &tile_number, 20);

                        let width_offset = (PIXEL_SIZE - num_width) / 2.0;
                        let height_offset = (PIXEL_SIZE - num_height) / 2.0;

                        let pos_x = (i as f64 * PIXEL_SIZE) + width_offset;
                        let pos_y = ((j as f64 * PIXEL_SIZE) + (PIXEL_SIZE * 3.0)) - height_offset;
                        Text::new_color(number_color, 20)
                            .draw(
                                &*tile_number,
                                &mut self.cache,
                                &c.draw_state,
                                c.transform.trans(pos_x, pos_y),
                                gl,
                            )
                            .unwrap();
                    }
                }
            }
        })
    }

    fn update(&mut self) {
        let mut revealed_tiles = 0;

        for j in 0..self.height {
            for i in 0..self.width {
                if !(self.board[j as usize][i as usize].has_mine())
                    && self.board[j as usize][i as usize].is_revealed()
                {
                    revealed_tiles += 1;
                }
            }
        }
        if self.tiles_to_win == revealed_tiles {
            for i in 0..self.height {
                for j in 0..self.width {
                    self.board[i as usize][j as usize].set_flagged(true);
                }
            }
            self.game_result = 2;
        }
    }

    pub fn run(&mut self) {
        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut self.window) {
            if let Some(args) = e.render_args() {
                self.render(args);
            }

            if let Some(b) = e.button_args() {
                self.button_press(b);
            }

            if let Some(m) = e.mouse_cursor_args() {
                self.mouse_update(m);
            }

            if let Some(_u) = e.update_args() {
                self.update();
            }
        }
    }
}

fn get_text_size(cache: &mut Cache<(), Texture>, str: &str, font_size: u32) -> (f64, f64) {
    let mut width = 0.0;
    let mut height = 0.0;
    for ch in str.chars() {
        let character = cache.character(font_size, ch).ok().unwrap();
        width += (character.advance_width() + character.left()) as f64;

        if (character.advance_height() + character.top()) as f64 > height {
            height = (character.advance_height() + character.top()) as f64;
        }
    }

    (width, height)
}
