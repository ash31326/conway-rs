#![feature(inclusive_range_syntax)] 
extern crate piston_window;
extern crate rand;

use piston_window::*;
use std::default::Default;
use rand::Rng;
use std::{thread, time};

// Constants
const BLACK: [f32; 4]  = [0.0, 0.0, 0.0, 1.0];
const SCREEN_WIDTH: usize = 70;
const SCREEN_HEIGHT: usize = 70;


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
#[derive(Copy, Clone)]
enum game {
    Stopped,
    Started,
    Paused
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
    board.random();
    
    EventLoop::set_ups(&mut window, 1);
    EventLoop::set_max_fps(&mut window, 10);

    while let Some(e) = window.next() {
        
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
    }    
}

