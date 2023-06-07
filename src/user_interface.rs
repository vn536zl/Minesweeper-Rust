extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::fs::File;
use std::io::Write;

use graphics::*;
use graphics::rectangle::Border;
use graphics::glyph_cache::rusttype::GlyphCache as Cache;

use opengl_graphics::{GlGraphics, OpenGL, GlyphCache, TextureSettings, Texture};

use piston::event_loop::{ EventSettings, Events };
use piston::input::{ ButtonEvent, RenderEvent, Button, ButtonState, MouseButton, MouseCursorEvent };
use piston::window::{ WindowSettings };

use glutin_window::GlutinWindow as Window;
use piston::{Key, UpdateEvent};

use crate::minesweeper;
use crate::minesweeper::MinesweeperBoard;

const PIXEL_SIZE: f64 = 32.0;
const FONT: &[u8] = include_bytes!("mine-sweeper.ttf");

fn get_text_size(str: &str, cache: &mut Cache<(), Texture>, font_size: u32) -> (f64, f64) {

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

pub fn run(mut board: MinesweeperBoard, mut mine_count: i32) {
    let opengl = OpenGL::V3_2;
    let mut game_results: i32 = 0;
    let default_mines = mine_count;

    let mut height = board.len() as i32;
    let mut width = board[0].len() as i32;
    let tiles_to_win = (height * width) - mine_count;
    let world_size = [(PIXEL_SIZE * width as f64), (PIXEL_SIZE * height as f64) + (PIXEL_SIZE * 2.0)];

    let mut window: Window = WindowSettings::new("Spinning Square", world_size)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut gl = GlGraphics::new(opengl);


    let exe_path = std::env::current_dir().unwrap();
    let path = exe_path.to_str().unwrap().to_owned() + "\\Font.ttf";

    let mut f = File::create(&path).unwrap();
    f.write_all(FONT).expect("No File Written");

    let ref mut cache = GlyphCache::new(path, (), TextureSettings::new()).unwrap();

    let mut hit_mine_pos: [i32; 2] = [0, 0];

    let mut mouse_pos: [i32; 2] = [0, 0];
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            gl.draw(
                args.viewport(),
                |c, gl| {
                    clear(color::WHITE, gl);

                    let rect = Rectangle::new(color::grey(0.73)).border( Border {
                        color: color::WHITE,
                        radius: 1.5,
                    });
                    let dims = [0.0, 0.0, world_size[0], PIXEL_SIZE * 2.0];

                    rect.draw(
                        dims,
                        &c.draw_state,
                        c.transform,
                        gl,
                    );

                    let (mine_count_width, mine_count_height) = get_text_size(mine_count.to_string().as_str(), cache, 64);

                    let width_offset = ((world_size[0]/3.0) - mine_count_width) / 2.0;
                    let height_offset = ((PIXEL_SIZE * 2.0) - mine_count_height) / 2.0;

                    Text::new_color(color::RED, 64)
                        .draw(
                            mine_count.to_string().as_str(),
                            cache,
                            &c.draw_state,
                            c.transform.trans(width_offset, (PIXEL_SIZE * 2.0) - height_offset),
                            gl
                        ).unwrap();

                    for j in 0..height {
                        for i in 0..width {
                            let mut tile_color = color::grey(0.73);
                            let mut border_color = color::WHITE;
                            let mut border_radius = 1.5;


                            let pos_x = i as f64 * PIXEL_SIZE;
                            let pos_y = (2 + j) as f64 * PIXEL_SIZE;
                            let current_tile = board[j as usize][i as usize];


                            if current_tile.is_revealed() {
                                border_color = color::grey(0.48);
                                border_radius = 0.5;
                            } else if current_tile.is_flagged() {
                                tile_color = color::LIME;
                                border_color = color::GREEN;
                                border_radius = 1.0;
                            }

                            if current_tile.has_mine() && game_results == 1 {
                                if [i, j] == hit_mine_pos {
                                    tile_color = color::RED
                                } else {
                                    tile_color = color::BLACK;
                                }
                                border_color = color::RED;
                                border_radius = 1.0;
                            }

                            let rect = Rectangle::new(tile_color).border(Border {
                                color: border_color,
                                radius: border_radius
                            });
                            let dims = rectangle::square(0.0, 0.0, PIXEL_SIZE);

                            rect.draw(dims, &c.draw_state, c.transform.trans(pos_x, pos_y), gl);
                        }
                    }
                    for j in 0..height {
                        for i in 0..width {
                            let current_tile = board[j as usize][i as usize];

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
                                    _ => color::grey(0.73)
                                };

                                let (num_width, num_height) = get_text_size(&tile_number, cache, 20);

                                let width_offset = (PIXEL_SIZE - num_width) / 2.0;
                                let height_offset = (PIXEL_SIZE - num_height) / 2.0;

                                let pos_x = (i as f64 * PIXEL_SIZE) + width_offset;
                                let pos_y = ((j as f64 * PIXEL_SIZE) + (PIXEL_SIZE*3.0)) - height_offset;
                                Text::new_color(number_color, 20)
                                    .draw(&*tile_number, cache, &c.draw_state, c.transform.trans(pos_x, pos_y), gl)
                                    .unwrap();
                            }
                        }
                    }
                }
            )
        }

        if let Some(m) = e.mouse_cursor_args() {
            mouse_pos = [(m[0]/PIXEL_SIZE) as i32, ((m[1] - PIXEL_SIZE*2.0) / PIXEL_SIZE) as i32];
        }

        if let Some(b) = e.button_args() {
            if b.state == ButtonState::Press {
                match b.button {
                    Button::Mouse(MouseButton::Left) => {
                        if game_results == 0 {
                            if !(board[mouse_pos[1] as usize][mouse_pos[0] as usize].is_flagged()) {
                                if board[mouse_pos[1] as usize][mouse_pos[0] as usize].has_mine() {
                                    hit_mine_pos = mouse_pos;
                                    game_results = 1;
                                }
                                minesweeper::reveal_tile(&mut board, mouse_pos[0], mouse_pos[1])
                            }
                        }
                    }
                    Button::Mouse(MouseButton::Right) => {
                        if game_results == 0 {
                            if !(board[mouse_pos[1] as usize][mouse_pos[0] as usize].is_revealed()) {
                                let mut flagged_tile = board[mouse_pos[1] as usize][mouse_pos[0] as usize];
                                if flagged_tile.is_flagged() {
                                    flagged_tile.set_flagged(false);
                                    mine_count += 1
                                } else {
                                    flagged_tile.set_flagged(true);
                                    mine_count -= 1
                                }
                                board[mouse_pos[1] as usize][mouse_pos[0] as usize] = flagged_tile;
                            }
                        }
                    }
                    Button::Keyboard(Key::LCtrl) => {
                        if game_results == 0 {
                            if !(board[mouse_pos[1] as usize][mouse_pos[0] as usize].is_revealed()) {
                                let mut flagged_tile = board[mouse_pos[1] as usize][mouse_pos[0] as usize];
                                if flagged_tile.is_flagged() {
                                    flagged_tile.set_flagged(false);
                                    mine_count += 1
                                } else {
                                    flagged_tile.set_flagged(true);
                                    mine_count -= 1
                                }
                                board[mouse_pos[1] as usize][mouse_pos[0] as usize] = flagged_tile;
                            }
                        }
                    }
                    Button::Keyboard(Key::R) => {
                        game_results = 0;
                        mine_count = default_mines;
                        board = minesweeper::build_minesweeper_board(height, width, mine_count);
                    }
                    _ => {}
                }
            }
        }

        if let Some(_u) = e.update_args() {
            height = board.len() as i32;
            width = board[0].len() as i32;

            let mut revealed_tiles = 0;
            for j in 0..height {
                for i in 0..width {
                    if !(board[j as usize][i as usize].has_mine()) && board[j as usize][i as usize].is_revealed() {
                        revealed_tiles += 1;
                    }
                }
            }
            if tiles_to_win - revealed_tiles == 0 {
                for i in 0..height {
                    for j in 0..width {
                        let mut piece = board[j as usize][i as usize];
                        piece.set_flagged(true);
                        board[j as usize][i as usize] = piece;
                    }
                }
                game_results = 2;
            }
        }
    }
}