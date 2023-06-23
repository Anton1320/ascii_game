use ncurses::*;
use crate::objects::*;

pub fn init_view() {
    initscr();
}

const ASCII_CHARS: &str = "``.-':_,^=;><+!rc*/z?sLTv)J7(|Fi{C}fI31tlu[neoZ5Yxjya]2ESwqkP6h9d4VpOGbUAKXHm8RD#$Bg0MNWQ%&@";
const ASCII_BRIGHTNESS: [f64; 92] = [0., 0.0751, 0.0829, 0.0848, 0.1227, 0.1403, 0.1559, 0.185, 0.2183, 0.2417, 0.2571, 0.2852,
 0.2902, 0.2919, 0.3099, 0.3192, 0.3232, 0.3294, 0.3384, 0.3609, 0.3619, 0.3667, 0.3737, 0.3747, 0.3838, 0.3921, 0.396, 0.3984,
  0.3993, 0.4075, 0.4091, 0.4101, 0.42, 0.423, 0.4247, 0.4274, 0.4293, 0.4328, 0.4382, 0.4385, 0.442, 0.4473, 0.4477, 0.4503, 0.4562,
   0.458, 0.461, 0.4638, 0.4667, 0.4686, 0.4693, 0.4703, 0.4833, 0.4881, 0.4944, 0.4953, 0.4992, 0.5509, 0.5567, 0.5569, 0.5591, 0.5602,
    0.5602, 0.565, 0.5776, 0.5777, 0.5818, 0.587, 0.5972, 0.5999, 0.6043, 0.6049, 0.6093, 0.6099, 0.6465, 0.6561, 0.6595, 0.6631, 0.6714,
     0.6759, 0.6809, 0.6816, 0.6925, 0.7039, 0.7086, 0.7235, 0.7302, 0.7332, 0.7602, 0.7834, 0.8037, 1.1];

fn get_char_by_dist(dist: f64, max_dist: f64) -> char {
    let darkness = dist / max_dist;
    let i = ASCII_BRIGHTNESS.partition_point(|&x| x < 1.-darkness);
    ASCII_CHARS.as_bytes()[i] as char
}

pub fn print_on_top<T: ToString>(s: T) {
    mvaddstr(0, 0, &s.to_string());
    refresh();
}

pub fn draw(camera: &Player) {
    let (height, width): (i32, i32) = (camera.camera_resolution.0 as i32, camera.camera_resolution.1 as i32);
    let distances = &camera.camera_distances;
    let angle_of_view = &camera.camera_angle;
    let max_dist = camera.camera_dist;

    clear();

    let mut line_height;
    for (j, dist_and_object) in distances.iter().enumerate() {
        if let Some((dist, intersection_point, _)) = dist_and_object {
            let cos = ((*angle_of_view) / width as f64 * (j as i32 - width / 2) as f64).radians.cos();
            let not_fish_dist = dist * cos;
            line_height = (0.33 * (height as f64) / not_fish_dist) as i32;

            if line_height > height/2 {
                line_height = height;
            }

            for i in (height / 2 - line_height)..(height / 2 + line_height) {
                
                if intersection_point.distance_to(intersection_point.round()) < 0.1 {
                    mvaddch(i, j as i32, '|' as u32);
                }
                else {
                    mvaddch(i, j as i32, get_char_by_dist(*dist, max_dist) as u32);
                }
            }
            mvaddch(height / 2 - line_height, j as i32, '=' as u32);
            mvaddch(height / 2 + line_height, j as i32, '=' as u32);
        }
    }
    refresh();
}