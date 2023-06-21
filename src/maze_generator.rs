use std::collections::HashMap;
use rand::Rng;

pub fn generate_maze(height: usize, width: usize) -> (Vec<Vec<bool>>, Vec<Vec<bool>>) {
    let mut row = vec![0; width];
    let mut right_wall = vec![vec![false; width]; height];
    let mut down_wall = vec![vec![false; width]; height];
    let mut color = 1;

    let mut cnt_of_colors = HashMap::new();

    for i in 0..height {
        for j in 0..width {
            if row[j] == 0 {
                row[j] = color;
                *cnt_of_colors.entry(color).or_insert(0) += 1;
                color += 1;
            }
        }
        for j in 0..width-1 {
            if row[j] == row[j+1] {
                right_wall[i][j] = true;
            } else if rand::thread_rng().gen_bool(0.5) { // if there is no right wall
                *cnt_of_colors.entry(row[j+1]).or_insert(0) -= 1;
                *cnt_of_colors.entry(row[j]).or_insert(0) += 1;
                row[j+1] = row[j];
                right_wall[i][j] = false;

            } else {
                right_wall[i][j] = true;
            }
        }
        for j in 0..width {
            if cnt_of_colors.get(&row[j]).copied().unwrap_or(0) > 1 { // if there can be a down wall
                if rand::thread_rng().gen_bool(0.5) { // if there is a down wall
                    down_wall[i][j] = true;
                    *cnt_of_colors.entry(row[j]).or_insert(0) -= 1;
                    if i < height-1 {
                        row[j] = 0;
                    }
                }
            }
        }
    }

    for j in 0..width {
        down_wall[height-1][j] = true;
    }
    for j in 0..width-1 {
        if row[j] != row[j+1] {
            right_wall[height-1][j] = false;
        }
    }
    for i in 0..height {
        right_wall[i][width-1] = true;
    }

    (right_wall, down_wall)
}

pub fn thin_to_thick_maze(right_wall: &Vec<Vec<bool>>, down_wall: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut maze = vec![vec![false; right_wall[0].len()*2]; right_wall.len()*2];
    for i in 0..right_wall.len() {
        for j in 0..right_wall[i].len() {
            if right_wall[i][j] {
                if i > 0 {
                    maze[2*i-1][2*j+1] = true;
                }
                maze[2*i][2*j+1] = true;
                maze[2*i+1][2*j+1] = true;
                
            }
            if down_wall[i][j] {
                if j > 0 {
                    maze[2*i+1][2*j-1] = true;
                }
                maze[2*i+1][2*j] = true;
                maze[2*i+1][2*j+1] = true;
                
            }
        }
    }
    maze
}

pub fn matrix_to_vector<T>(m: Vec<Vec<T>>) -> Vec<T> {
    let mut a: Vec<T> = Vec::new();
    for row in m {
        for elem in row {
            a.push(elem);
        }
    }
    a
}