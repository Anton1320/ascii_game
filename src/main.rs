use std::{f64::consts::PI};

use lyon_geom::*;
use ncurses::*;
use objects::{Map, Player};
use view::{init_view, draw};

use crate::objects::MapObject;

mod game;
mod view;
mod objects;
mod maze_generator;

fn main() {
    init_view();
    keypad(stdscr(), true);
    let mut player = Player::new(point(500.5, 500.5), vector(1., 0.), Angle { radians:PI*2./3. });
    let map = Map::new();
    player.set_camera_distances(&map);
    draw(&player);
    //println!("{:?}", player.camera_distances);
    let left_turn_angle_sin_cos = Angle {radians:-PI/12.}.sin_cos();
    let right_turn_angle_sin_cos = Angle {radians:PI/12.}.sin_cos();

    let speed = 0.3;

    loop {
        //println!("{}", player.pos.y);
        const Q: i32 = 'q' as i32;
        match getch() {
            Q => break,
            KEY_RIGHT => player.turn_by_angle_sin_cos(right_turn_angle_sin_cos),
            KEY_LEFT => player.turn_by_angle_sin_cos(left_turn_angle_sin_cos),
            KEY_UP => {
                let new_pos = player.pos.clone() + player.direction.clone()*0.2;
                
                match map.get(&new_pos).unwrap_or(&MapObject::Space) {
                    MapObject::Wall => (),
                    MapObject::Space => player.pos += player.direction*speed,
                }
            },
            KEY_DOWN => {
                let new_pos = player.pos.clone() - player.direction.clone()*0.2;
                
                match map.get(&new_pos).unwrap_or(&MapObject::Space) {
                    MapObject::Wall => (),
                    MapObject::Space => player.pos -= player.direction*speed,
                }
            },
            _ => ()
        }
        player.set_camera_distances(&map);
        draw(&player);
    }
    
    endwin();
}
