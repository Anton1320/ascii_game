use lyon_geom::{*};
use ncurses::*;
use crate::maze_generator::*;

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
    map: Vec<MapObject>,
    height: i32,
    width: i32,
}

impl Map {
    pub fn new() -> Map {
        let height = 1000;
        let width = 1000;

        let thin_map = generate_maze(height/2, width/2);
        let thick_map = thin_to_thick_maze(&thin_map.0, &thin_map.1);
        let vector_map = matrix_to_vector(thick_map);


        let mut map = Vec::new();
        
        for elem in vector_map {
            if elem {
                map.push(MapObject::Wall);
            } else {
                map.push(MapObject::Space);
            }

        }

        Map {
            map,
            height: height as i32,
            width: height as i32,
        }
    }

    pub fn get(&self, pos: &Point<f64>) -> Option<&MapObject> {
        if pos.y < 0. || pos.x < 0. || pos.x as i32 >= self.width {
            return None
        }
        self.map.get(((pos.y as i32)*self.width + pos.x as i32) as usize)
    }
}


pub type IntersectionInfo<'a> = (f64, Point<f64>, &'a MapObject);
pub struct Player<'a> {
    pub pos: Point<f64>,        // position of player
    left_direction: Vector<f64>, // normilized vector of direction of left side of camera angle
    pub direction: Vector<f64>, // normalized vector of direction of view
    pub camera_dist: f64, // distance of view
    pub camera_angle: Angle<f64>,    // angle of view
    camera_resolution_angle_sin_cos: (f64, f64), // sin and cos of (camera_angle / camera_resolution.1)
    half_camera_angle_sin_cos: (f64, f64), // sin and cos of (camera_angle / 2.)
    pub camera_resolution: (usize, usize), // (height, width)
    pub camera_distances: Vec<Option<IntersectionInfo<'a>>>, // distances from each camera ray to it's intersection with map object and that object's type
}
//cos(a+b) = cos(a) * cos(b) - sin(a) * sin(b)
//sin(a+b) = sin(a) * cos(b) + sin(b) * cos(a)
impl<'a> Player<'a> {
    pub fn new (
        pos: Point<f64>,
        direction: Vector<f64>,
        camera_angle: Angle<f64>,
    ) -> Player<'a> {
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

    fn cast_ray(&self, dir: &Vector<f64>, map: &'a Map) -> Option<IntersectionInfo<'a>> {
        let mut dist = 0.; //distance to intersection
        let mut ray_point = self.pos.clone(); // point on a ray with dist distance from its origin
        ray_point += *dir * 0.2;
        while dist < self.camera_dist {
            if let Some(i) = map.get(&ray_point) {
                
                match *i {
                    MapObject::Wall => { return Some((dist, ray_point.clone(), i)) },
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

