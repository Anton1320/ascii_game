use std::mem::replace;

use lyon_geom::{*};
use ncurses::*;
use crate::{maze_generator::*, view::print_on_top};

pub fn turn_vector_by_angles_sin_cos(v: &mut Vector<f64>, sin_cos: (f64, f64)) {
    (v.y, v.x) = (
        v.y * sin_cos.1 + v.x * sin_cos.0,
        v.x * sin_cos.1 - v.y * sin_cos.0
    );
}


#[derive(Clone, Debug)]

pub enum MapObject {
    Space,
    Wall,
}

#[derive(Debug)]
pub struct Map {
    map: Vec<Vec<MapObject>>,
    new_map: Vec<Vec<MapObject>>,
    tile_height: f64,
    tile_width: f64,
    main_tile_pos: Point<f64>, // main tile is the top left tile. global pos of top left angle 
}

impl Map {
    pub fn new() -> Map {
        let tile_height = 16;
        let tile_width = 16;
        let mut map = vec![];

        for tile in 0..9 {
            map.push(vec![]);
            let thin_map = generate_maze(tile_height/2, tile_width/2);
            let thick_map = thin_to_thick_maze(&thin_map.0, &thin_map.1);
            let vector_map = matrix_to_vector(thick_map);
            
            for elem in vector_map {
                if elem {
                    map[tile].push(MapObject::Wall);
                } else {
                    map[tile].push(MapObject::Space);
                }

            }
        }

        Map {
            map,
            new_map: vec![],
            tile_height: tile_height as f64,
            tile_width: tile_width as f64,
            main_tile_pos: point(-(tile_width as f64), -(tile_height as f64)),
        }
    }

    fn generate_tile(&self) -> Vec<MapObject> {
        let thin_map = generate_maze(self.tile_height as usize / 2, self.tile_width as usize / 2);
        let thick_map = thin_to_thick_maze(&thin_map.0, &thin_map.1);
        let tile_bool = matrix_to_vector(thick_map);
        let mut out_tile = Vec::new();
        for i in tile_bool {
            if i {
                out_tile.push(MapObject::Wall);
            } else {
                out_tile.push(MapObject::Space);
            }
        }
        out_tile
    }

    fn generate_empty_tile(&self) -> Vec<MapObject> {
        let mut out = Vec::new();
        for i in 0..(self.tile_height*self.tile_width) as i32 {
            out.push(MapObject::Space);
        }
        out
    } 

    pub fn get(&self, pos: &Point<f64>) -> Option<&MapObject> {
        let rel_pos = *pos - self.main_tile_pos;
        let (tile_x, tile_y) = self.get_tile_pos_xy(pos);

        if tile_x < 0 || tile_x > 2 || tile_y < 0 || tile_y > 2 {
            return None;
        }
        
        let pos_in_tile = rel_pos - vector((tile_x as f64)*self.tile_width, (tile_y as f64)*self.tile_height as f64);
        self.map[(tile_y*3+tile_x) as usize].get(((pos_in_tile.y as i32)*(self.tile_width as i32) + pos_in_tile.x as i32) as usize)
    }

    pub fn get_tile_pos_xy(&self, pos: &Point<f64>) -> (i32, i32) { // pos in tiles related to the main one
        let rel_pos = *pos - self.main_tile_pos;
        ((rel_pos.x / self.tile_width).floor() as i32, (rel_pos.y / self.tile_height).floor() as i32)
    }

    pub fn update_main_tile(&mut self, pos: &Point<f64>) {
        let (tile_x, tile_y) = self.get_tile_pos_xy(pos);
        //println!("{tile_x}, {tile_y}");
        self.new_map.clear();
        for i in tile_y-1..=tile_y+1 {
            for j in tile_x-1..=tile_x+1 {
                if i >= 0 && i < 3 && j >= 0 && j < 3 {
                    self.new_map.push(replace(&mut self.map[(i*3+j) as usize], Vec::new()));
                }
                else  {
                    self.new_map.push(self.generate_tile());
                    //self.new_map.push(self.generate_empty_tile());
                }
            }
        }
        self.main_tile_pos += vector((tile_x-1) as f64 * self.tile_width, (tile_y-1) as f64 * self.tile_height);
        //print_on_top(format!("{tile_x}, {tile_y}"));
        self.map = self.new_map.clone();
    }
}


pub type IntersectionInfo = (f64, Point<f64>, MapObject);
pub struct Player {
    pub pos: Point<f64>,        // position of player
    left_direction: Vector<f64>, // normilized vector of direction of left side of camera angle
    pub direction: Vector<f64>, // normalized vector of direction of view
    pub camera_dist: f64, // distance of view
    pub camera_angle: Angle<f64>,    // angle of view
    camera_resolution_angle_sin_cos: (f64, f64), // sin and cos of (camera_angle / camera_resolution.1)
    half_camera_angle_sin_cos: (f64, f64), // sin and cos of (camera_angle / 2.)
    pub camera_resolution: (usize, usize), // (height, width)
    pub camera_distances: Vec<Option<IntersectionInfo>>, // distances from each camera ray to it's intersection with map object and that object's type
}
//cos(a+b) = cos(a) * cos(b) - sin(a) * sin(b)
//sin(a+b) = sin(a) * cos(b) + sin(b) * cos(a)
impl<'a> Player {
    pub fn new (
        pos: Point<f64>,
        direction: Vector<f64>,
        camera_angle: Angle<f64>,
    ) -> Player {
        let mut max_x = 0;
        let mut max_y = 0;
        getmaxyx(stdscr(), &mut max_y, &mut max_x);
        let half_camera_angle_sin_cos = (-camera_angle / 2.).sin_cos();
        let mut left_direction = direction.clone();
        turn_vector_by_angles_sin_cos(&mut left_direction, half_camera_angle_sin_cos);
        Player {
            pos,
            half_camera_angle_sin_cos,
            left_direction,
            camera_dist: 10.,
            direction,
            camera_angle,
            camera_resolution: (max_y as usize, max_x as usize),
            camera_resolution_angle_sin_cos: (camera_angle / (max_x as f64)).sin_cos(),
            camera_distances: vec![None; max_x as usize],
        }
    }

    pub fn turn_by_angle_sin_cos(&mut self, sin_cos: (f64, f64)) {
        turn_vector_by_angles_sin_cos(&mut self.direction, sin_cos);
        turn_vector_by_angles_sin_cos(&mut self.left_direction, sin_cos);
    }

    fn cast_ray(&self, dir: &Vector<f64>, map: &'a Map) -> Option<IntersectionInfo> {
        let mut dist = 0.; //distance to intersection
        let mut ray_point = self.pos.clone(); // point on a ray with dist distance from its origin
        ray_point += *dir * 0.2;
        while dist < self.camera_dist {
            if let Some(i) = map.get(&ray_point) {
                
                match *i {
                    MapObject::Wall => { return Some((dist, ray_point.clone(), i.clone())) },
                    _ => (),
                }
            }
            dist += 0.05;
            ray_point += *dir * 0.05;
        }
        None
    }

    pub fn set_camera_distances(&mut self, map: &'a Map) {
        let mut ray_dir = self.left_direction.clone();
        for i in 0..self.camera_resolution.1 {
            self.camera_distances[i] = self.cast_ray(&ray_dir, map);
            turn_vector_by_angles_sin_cos(&mut ray_dir, self.camera_resolution_angle_sin_cos);
        }
    }


}

