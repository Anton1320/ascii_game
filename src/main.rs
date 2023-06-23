use std::io::stdin;
use std::{f64::consts::PI, sync::mpsc::channel, thread};

use std::time::{Instant};

use lyon_geom::*;
use ncurses::*;
use objects::{Map, Player};
use view::{init_view, draw};
use rdev::*;

use crate::objects::MapObject;

mod game;
mod view;
mod objects;
mod maze_generator;

fn main() {
    // spawn new thread because listen blocks
    let (schan, rchan) = channel();
    let _listener = thread::spawn(move || {
        listen(move |event| {
            if let EventType::KeyPress(_) = &event.event_type {
                schan
                .send(event)
                .unwrap_or_else(|e| println!("Could not send event {:?}", e));
            } else if let EventType::KeyRelease(_) = &event.event_type {
                schan
                .send(event)
                .unwrap_or_else(|e| println!("Could not send event {:?}", e));
            }
            
        })
        .expect("Could not listen");
    });

    init_view();
    //keypad(stdscr(), true);

    let mut player = Player::new(point(0.5, 0.5), vector(1., 0.), Angle { radians:PI*2./3. });
    let mut map = Map::new();
    player.set_camera_distances(&map);


    //println!("{:?}", player.camera_distances);
    //getch();

    draw(&player);
    //println!("{:?}", player.camera_distances);
    let left_turn_angle = Angle {radians:-PI*2./3.};
    let right_turn_angle = Angle {radians:PI*2./3.};

    let speed = 2.;

    let mut forward = false;
    let mut backward = false;
    let mut left = false;
    let mut right = false;

    let mut start;
    let mut duration;


    'main: loop {
        start = Instant::now();
        //println!("{}", player.pos.y);
       
        player.set_camera_distances(&map);
    
        map.update_main_tile(&player.pos);

        draw(&player);
        //view::print_on_top(format!("{:?}", map.get_tile_pos_xy(&player.pos)));
        

        for event in rchan.try_iter() {
            if let EventType::KeyPress(Key::UpArrow) = &event.event_type {
                forward = true;
            } else if let EventType::KeyRelease(Key::UpArrow) = &event.event_type {
                forward = false;
                
            } else if let EventType::KeyPress(Key::DownArrow) = &event.event_type {
                backward = true;
            } else if let EventType::KeyRelease(Key::DownArrow) = &event.event_type {
                backward = false;
                
            } else if let EventType::KeyPress(Key::LeftArrow) = &event.event_type {
                left = true;
            } else if let EventType::KeyRelease(Key::LeftArrow) = &event.event_type {
                left = false;
                
            } else if let EventType::KeyPress(Key::RightArrow) = &event.event_type {
                right = true;
            } else if let EventType::KeyRelease(Key::RightArrow) = &event.event_type {
                right = false;
                
            } else if let EventType::KeyPress(Key::KeyQ) = &event.event_type {
                break 'main;
            }
            
        }

        duration = start.elapsed();

        if forward {
            let new_pos = player.pos.clone() + (player.direction.clone() * duration.as_secs_f64() * 2.);
                
            match map.get(&new_pos).unwrap_or(&MapObject::Space) {
                MapObject::Wall => (),
                MapObject::Space => player.pos += player.direction*speed * duration.as_secs_f64(),
            }
        }
        if backward {
            let new_pos = player.pos.clone() - (player.direction.clone() * duration.as_secs_f64() * 2.);
                
            match map.get(&new_pos).unwrap_or(&MapObject::Space) {
                MapObject::Wall => (),
                MapObject::Space => player.pos -= player.direction*speed * duration.as_secs_f64(),
            }
        }
        if left {
            player.turn_by_angle_sin_cos((left_turn_angle * duration.as_secs_f64()).sin_cos())
        }
        if right {
            player.turn_by_angle_sin_cos((right_turn_angle * duration.as_secs_f64()).sin_cos())
        }
    }
    
    endwin();
}
