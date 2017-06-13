#![feature(inclusive_range_syntax)] 
extern crate piston_window;
extern crate rand;
extern crate find_folder;

use piston_window::*;
use std::default::Default;
use rand::Rng;
use std::{thread, time};

// Constants
const BLACK: [f32; 4]  = [0.0, 0.0, 0.0, 1.0];
const BLUE: [f32; 4] = [0.0, 0.82, 0.96, 1.0];
const SCREEN_WIDTH: usize = 70;
const SCREEN_HEIGHT: usize = 67;
const VERSION: &'static str = "v: 0.1.0";

#[derive(Debug, Copy, Clone)]
struct block {
    state: u8, // on or off
    neighbors: u8 // number of neighbors
}


impl Default for block {
    fn default() -> block {
        block { state: 0, neighbors: 0 }
    }
}

// Not used right now, will eventually implement ability to reset and pause game
#[derive(Copy, Clone, PartialEq)]
enum game {
    Menu,
    Running,
    Custom,
    Paused
}

struct menu {
    title: &'static str,
    option1: &'static str,
    option1_color: [f32; 4],
    option2: &'static str,
    option2_color: [f32; 4],
    option3: &'static str,
    option3_color: [f32; 4],
    option4: &'static str,
    option4_color: [f32; 4],
    selection: u8,
    home_help: &'static str,
    custom_help: &'static str,
    running_help: &'static str
}

impl menu {
    fn new() -> menu {
        menu { title: "CONWAY-RS", option1: "RANDOM GAME", option1_color: BLUE,
               option2: "CUSTOM GAME", option2_color: BLACK,
               option3: "ABOUT", option3_color: BLACK,
               option4: "QUIT", option4_color: BLACK,
               selection: 0, home_help: "UP/DOWN: MOVE - ENTER: SELECT",
               custom_help: "LEFT/RIGHT MOUSE: DRAW/ERASE - ENTER: START - ESC: MENU",
               running_help: "GAME IS RUNNING - PRESS ESC TO OPEN MENU"}
    }
    fn selection_change(&mut self) -> &mut Self {
        match self.selection {
            0 => { self.option1_color = BLUE; self.option2_color = BLACK; self.option3_color = BLACK; self.option4_color = BLACK; },
            1 => { self.option1_color = BLACK; self.option2_color = BLUE; self.option3_color = BLACK; self.option4_color = BLACK; },
            2 => { self.option1_color = BLACK; self.option2_color = BLACK; self.option3_color = BLUE; self.option4_color = BLACK; },
            3 => { self.option1_color = BLACK; self.option2_color = BLACK; self.option3_color = BLACK; self.option4_color = BLUE; },
            _ => { self.option1_color = BLACK; self.option2_color = BLACK; self.option3_color = BLACK; self.option4_color = BLACK; }
        }
        self
    }
}

#[derive(Copy)]
struct screen {
    buffer: [[block; SCREEN_HEIGHT]; SCREEN_WIDTH],
    game_state: game,
    menu_open: bool
}

impl Clone for screen {
    fn clone(&self) -> screen { *self }
}

impl screen {
    fn new() -> screen { // Create new, blank screen object
        let pixel: block = Default::default();
        screen { buffer: [[pixel; SCREEN_HEIGHT]; SCREEN_WIDTH],
                 game_state: game::Menu, menu_open: false }
    }

    fn random(&mut self) -> &mut Self { // Populate game board with random seed
        let mut rng = rand::thread_rng();
        for i in self.buffer.iter_mut() {
            for j in i.iter_mut() {
                j.state = rng.gen_range(0,2);
            }
        }
        self
    }

    fn clear_buffer(&mut self) -> &mut Self { // Clear the game board while keeping screen state
        let pixel: block = Default::default();
        self.buffer = [[pixel; SCREEN_HEIGHT]; SCREEN_WIDTH];
        self
    }

    fn check_neighbors(&mut self, x: usize, y: usize) -> &mut Self { // Counts neighbors for block at (x,y)
        let mut neighbors = 0;
        for i in 0...2 {
            let k = (x + i + SCREEN_WIDTH - 1) % SCREEN_WIDTH;          
            for j in 0...2 {
                let h = (y + j + SCREEN_HEIGHT - 1) % SCREEN_HEIGHT;
                neighbors += self.buffer[k][h].state;
            }
        }
        if self.buffer[x][y].state == 1 {
            neighbors -= 1;
        }
        self.buffer[x][y].neighbors = neighbors;
        self
    }

    fn update_board(&mut self) -> &mut Self {
        for i in self.buffer.iter_mut() {
            for j in i.iter_mut() {
                if j.state != 0 {
                    match j.neighbors {
                       0...1 => j.state = 0,
                       2...3 => j.state = 1,
                       4...8 => j.state = 0,
                       _ => panic!("Something horribly wrong happened!")
                    }
                } else {
                    match j.neighbors {
                        3 => j.state = 1,
                        _ => j.state = 0
                    }
                }
            }
        }
        self
    }
}

  
fn main() {

   let mut window: PistonWindow =
        WindowSettings::new("Conway's Game of Life", [700, 700])
        .resizable(false).exit_on_esc(false).build().unwrap();

    let mut board = screen::new();
    //board.random();
    
    EventLoop::set_ups(&mut window, 1);
    EventLoop::set_max_fps(&mut window, 10);

    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    //let ref menu_font = assets.join("PressStart2P.ttf");
    let ref menu_font = assets.join("Retro Computer_DEMO.ttf");
    let ref help_font = assets.join("VCR_OSD_MONO_1.001.ttf");
    let factory_menu = window.factory.clone();
    let factory_help = window.factory.clone();
    let mut menu_glyphs = Glyphs::new(menu_font, factory_menu).unwrap();
    let mut help_glyphs = Glyphs::new(help_font, factory_help).unwrap();
    let mut main_menu: menu = menu::new();

    // Game flags / variables
    let mut cursor = [0.0, 0.0];
    let mut draw_flag = false;
    let mut erase_flag = false;
    let mut generation: u64 = 0; // generation counter
    let mut xloc: usize = 0;
    let mut yloc: usize = 0;
    
    while let Some(e) = window.next() {

        if board.game_state == game::Menu {

            generation = 0;
            board.clear_buffer();
            
            if let Some(Button::Keyboard(key)) = e.press_args() {
               // println!("{:?}",key);
               // println!("{:?}", main_menu.selection);
                match key {
                    Key::Up => { if main_menu.selection == 0 { main_menu.selection = 0; }
                                   else { main_menu.selection -= 1; } },
                    Key::Down => { if main_menu.selection == 3 { main_menu.selection = 3;}
                                   else {main_menu.selection += 1; } },
                    Key::Return => match main_menu.selection { 0 => {board.game_state = game::Running; board.random();},
                                                               1 => {board.game_state = game::Custom; board.clear_buffer();
                                                                     window.draw_2d(&e, |c, g| { clear([1.0; 4], g)});},
                                                               2 => board.game_state = game::Menu,
                                                               3 => window.set_should_close(true),
                                                               _ => panic!("Game in wrong state")},
                    Key::Escape => window.set_should_close(true),
                    _ => {}
                }
            }
            
            main_menu.selection_change();
            
            window.draw_2d(&e, |c, g| {
                let transform = c.transform.trans(170.0, 100.0);
                // Set a white background
                clear([1.0; 4], g);
                text::Text::new_color(BLACK, 46).round().draw(
                    main_menu.title,
                    &mut menu_glyphs,
                    &c.draw_state,
                    transform, g);
            });
            window.draw_2d(&e, |c, g| {
                let transform = c.transform.trans(210.0, 270.0);
                text::Text::new_color(main_menu.option1_color, 30).round().draw(
                    main_menu.option1,
                    &mut menu_glyphs,
                    &c.draw_state,
                    transform, g);
            });
            window.draw_2d(&e, |c, g| {
                let transform = c.transform.trans(210.0, 370.0);
                text::Text::new_color(main_menu.option2_color, 30).round().draw(
                    main_menu.option2,
                    &mut menu_glyphs,
                    &c.draw_state,
                    transform, g);
            });
            window.draw_2d(&e, |c, g| {
                let transform = c.transform.trans(275.0, 470.0);
                text::Text::new_color(main_menu.option3_color, 30).round().draw(
                    main_menu.option3,
                    &mut menu_glyphs,
                    &c.draw_state,
                    transform, g);
            });
            window.draw_2d(&e, |c, g| {
                let transform = c.transform.trans(290.0, 570.0);
                text::Text::new_color(main_menu.option4_color, 30).round().draw(
                    main_menu.option4,
                    &mut menu_glyphs,
                    &c.draw_state,
                    transform, g);
            });            
            window.draw_2d(&e, |c, g| {
                let transform = c.transform.trans(30.0, 690.0);
                text::Text::new_color(BLACK, 9).round().draw(
                    main_menu.home_help,
                    &mut help_glyphs,
                    &c.draw_state,
                    transform, g);});
            window.draw_2d(&e, |c, g| {
                let transform = c.transform.trans(610.0, 690.0);
                text::Text::new_color(BLACK, 9).round().draw(
                    VERSION,
                    &mut help_glyphs,
                    &c.draw_state,
                    transform, g);});
                
        } else if board.game_state == game::Running {
            
            let board_temp = board;
        
            window.draw_2d(&e, |c, g| { clear([1.0; 4], g);
                let transform = c.transform.trans(30.0, 690.0);
                text::Text::new_color(BLACK, 9).round().draw(
                    main_menu.running_help,
                    &mut help_glyphs,
                    &c.draw_state,
                    transform, g);});

            // draw generation counter
            let generation_counter = format!("Generation: {:}", generation);
            window.draw_2d(&e, |c, g| {
                let transform = c.transform.trans(530.0, 690.0);
                text::Text::new_color(BLACK, 9).round().draw(
                    &generation_counter,
                    &mut help_glyphs,
                    &c.draw_state,
                    transform, g);});
            
            for (x, i) in board_temp.buffer.iter().enumerate() {
                for (y, j) in i.iter().enumerate() {
                
                    board.check_neighbors(x, y);

                    if j.state != 0 {
                        window.draw_2d(&e, |c, g| {rectangle(BLACK, [(x as f64)*10.0, (y as f64)*10.0, 10.0, 10.0], c.transform, g);});  
                    } 
                }
            }

            if let Some(Button::Keyboard(key)) = e.press_args() {
                match key {
                   Key::Escape => board.game_state = game::Menu,
                    _ => {}
                }
             }
            
            board.update_board();
            generation += 1;
            
        } else if board.game_state == game::Custom {
            
            window.draw_2d(&e, |c, g| { clear([1.0; 4], g);
                let transform = c.transform.trans(30.0, 690.0);
                text::Text::new_color(BLACK, 9).round().draw(
                    main_menu.custom_help,
                    &mut help_glyphs,
                    &c.draw_state,
                    transform, g);});
            
            e.mouse_cursor(|x, y| {cursor = [x, y];});
            let mut xloc_string = format!("X: {:}", xloc);
            let mut yloc_string = format!("Y: {:}", yloc);
            
            window.draw_2d(&e, |c, g| {
                let transform = c.transform.trans(590.0, 690.0);
                text::Text::new_color(BLACK, 9).round().draw(
                    &xloc_string,
                    &mut help_glyphs,
                    &c.draw_state,
                    transform, g);});
            
            window.draw_2d(&e, |c, g| {
                let transform = c.transform.trans(640.0, 690.0);
                text::Text::new_color(BLACK, 9).round().draw(
                    &yloc_string,
                    &mut help_glyphs,
                    &c.draw_state,
                    transform, g);});
            
            if let Some(Button::Mouse(button)) = e.press_args() {
                match button {
                    MouseButton::Left => {
                        draw_flag = true;
                        println!("Something {:?}", cursor);
                    },
                    MouseButton::Right => {
                        erase_flag = true;
                    },
                    _ => {println!("nothing");}
                }
            }
            if let Some(Button::Mouse(button)) = e.release_args() {
                match button {
                    MouseButton::Left => draw_flag = false,
                    MouseButton::Right => erase_flag = false,
                    _ => {}
                }
            }

            // fixes float to usize cast when float is negative
            cursor[0] = if cursor[0] < 0.0 { 0.0 } else { cursor[0] };
            cursor[1] = if cursor[1] < 0.0 { 0.0 } else { cursor[1] };
            
            xloc = cursor[0] as usize / 10;
            yloc = cursor[1] as usize / 10;
            
            
            // prevent cursor loc from indexing screen buffer out of bounds when drawing/erasing
            if xloc >= SCREEN_WIDTH {
                xloc = SCREEN_WIDTH - 1
            }
            
            if yloc >= SCREEN_HEIGHT {
                yloc = SCREEN_HEIGHT - 1
            }
            
            if draw_flag && erase_flag {
                erase_flag = false;
            }
            
            if draw_flag {
                board.buffer[xloc][yloc].state = 1;
            } else if erase_flag {
                board.buffer[xloc][yloc].state = 0;
            }
            
            for (x, i) in board.buffer.iter().enumerate() {
                for (y, j) in i.iter().enumerate() {
                
                     if j.state != 0 {
                        window.draw_2d(&e, |c, g| {rectangle(BLACK, [(x as f64)*10.0, (y as f64)*10.0, 10.0, 10.0], c.transform, g);});  
                    } 
                }
            }

            if let Some(Button::Keyboard(key)) = e.press_args() {
                match key {
                    Key::Return => { board.game_state = game::Running; let board_save = board.clone(); },
                    Key::Escape => board.game_state = game::Menu,
                    _ => {}
                }
            }
            
            println!("{:?}", cursor);
        } else if board.game_state == game::Paused {
            //TODO
        }
    }   
}

