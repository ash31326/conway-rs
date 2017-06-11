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
const SCREEN_HEIGHT: usize = 70;
const MENU_STRING: &'static str = "ENTER: RANDOM GAME ESC: EXIT";


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
    Stopped,
    Started,
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
    selection: u8 // may be a better way to do this but just going to use a match on the u8
}

impl menu {
    fn new() -> menu {
        menu { title: "CONWAY-RS", option1: "RANDOM GAME", option1_color: BLUE,
               option2: "CUSTOM GAME", option2_color: BLACK,
               option3: "ABOUT", option3_color: BLACK,
               selection: 0 }
    }
    fn selection_change(&mut self) -> &mut Self {
        match self.selection {
            0 => { self.option1_color = BLUE; self.option2_color = BLACK; self.option3_color = BLACK; },
            1 => { self.option1_color = BLACK; self.option2_color = BLUE; self.option3_color = BLACK; },
            2 => { self.option1_color = BLACK; self.option2_color = BLACK; self.option3_color = BLUE; },
            _ => panic!("Wrong selection value happened somehow")
        }
        self
    }
}

#[derive(Copy)]
struct screen {
    buffer: [[block; SCREEN_WIDTH]; SCREEN_HEIGHT],
    game_state: game
}

impl Clone for screen {
    fn clone(&self) -> screen { *self }
}

impl screen {
    fn new() -> screen { // Create new, blank screen object
        let pixel: block = Default::default();
        screen { buffer: [[pixel; SCREEN_WIDTH]; SCREEN_HEIGHT], game_state: game::Stopped }
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
        .resizable(false).exit_on_esc(true).build().unwrap();

    let mut board = screen::new();
    //board.random();
    
    EventLoop::set_ups(&mut window, 1);
    EventLoop::set_max_fps(&mut window, 10);

    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    let ref font = assets.join("PressStart2P.ttf");
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font, factory).unwrap();
    let mut main_menu: menu = menu::new();
    
    while let Some(e) = window.next() {


        if board.game_state == game::Stopped {

            if let Some(Button::Keyboard(key)) = e.press_args() {
               // println!("{:?}",key);
               // println!("{:?}", main_menu.selection);
                match key {
                    Key::Up => { if main_menu.selection == 0 { main_menu.selection = 0; }
                                   else { main_menu.selection -= 1; } },
                    Key::Down => { if main_menu.selection == 2 { main_menu.selection = 2;}
                                   else {main_menu.selection += 1; } },
                    Key::Return => match main_menu.selection { 0 => {board.game_state = game::Started; board.random();},
                                                              1 => board.game_state = game::Stopped,
                                                              2 => board.game_state = game::Stopped,
                                                              _ => panic!("Game in wrong state")},
                    _ => {}
                }
            }
            
            main_menu.selection_change();
            
            window.draw_2d(&e, |c, g| {
                let transform = c.transform.trans(140.0, 100.0);
                // Set a white background
                clear([1.0; 4], g);
                text::Text::new_color(BLACK, 32).draw(
                    main_menu.title,
                    &mut glyphs,
                    &c.draw_state,
                    transform, g);
            });
            window.draw_2d(&e, |c, g| {
                let transform = c.transform.trans(190.0, 350.0);
                text::Text::new_color(main_menu.option1_color, 20).draw(
                    main_menu.option1,
                    &mut glyphs,
                    &c.draw_state,
                    transform, g);
            });
            window.draw_2d(&e, |c, g| {
                let transform = c.transform.trans(190.0, 450.0);
                text::Text::new_color(main_menu.option2_color, 20).draw(
                    main_menu.option2,
                    &mut glyphs,
                    &c.draw_state,
                    transform, g);
            });
            window.draw_2d(&e, |c, g| {
                let transform = c.transform.trans(280.0, 550.0);
                text::Text::new_color(main_menu.option3_color, 20).draw(
                    main_menu.option3,
                    &mut glyphs,
                    &c.draw_state,
                    transform, g);
            });
            
        } else if board.game_state == game::Started {

            let board_temp = board;
        
            window.draw_2d(&e, |c, g| { clear([1.0; 4], g) });
        
            for (x, i) in board_temp.buffer.iter().enumerate() {
                for (y, j) in i.iter().enumerate() {
                
                    board.check_neighbors(x, y);

                    if j.state != 0 {
                        window.draw_2d(&e, |c, g| {rectangle(BLACK, [(x as f64)*10.0, (y as f64)*10.0, 10.0, 10.0], c.transform, g);});  
                    } 
                }
            }
        
            board.update_board();
        } else if board.game_state == game::Paused {
            //TODO
        }
    }   
}

