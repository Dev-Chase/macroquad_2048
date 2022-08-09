use macroquad::prelude::*;
extern crate rand;
use rand::Rng;

const TILE_SIZE: f32 = 125f32;
const RNG_CHOICE: [i32; 10] = [2, 2, 2, 2, 2, 2, 2, 2, 2, 4];

fn are_any_empty(row: &[i32; 4]) -> bool {
    for cell in row {
        if *cell == 0 {
            return true;
        }
    }
    false
}

fn get_index(n: i32) -> [i32; 2] {
    [n / 4, n - (n / 4 * 4)]
}

pub struct Board(pub [[i32; 4]; 4]);

impl Board {
    pub fn draw(&self) {
        // Looping through Rows in Board
        for row_i in 0..4 {
            // Looping through Columns in Board
            for col_i in 0..4 {
                // Checking if the Cell is 0
                if self.0[row_i][col_i] != 0 {
                    // Drawing the Tile Background
                    draw_rectangle(
                        col_i as f32 * TILE_SIZE,
                        row_i as f32 * TILE_SIZE,
                        TILE_SIZE,
                        TILE_SIZE,
                        WHITE,
                    );

                    // Drawing the Tile Borders
                    draw_line(
                        col_i as f32 * TILE_SIZE,
                        row_i as f32 * TILE_SIZE,
                        col_i as f32 * TILE_SIZE + TILE_SIZE,
                        row_i as f32 * TILE_SIZE,
                        1f32,
                        BLACK,
                    );
                    draw_line(
                        col_i as f32 * TILE_SIZE,
                        row_i as f32 * TILE_SIZE,
                        col_i as f32 * TILE_SIZE,
                        row_i as f32 * TILE_SIZE + TILE_SIZE,
                        1f32,
                        BLACK,
                    );
                    draw_line(
                        col_i as f32 * TILE_SIZE + TILE_SIZE,
                        row_i as f32 * TILE_SIZE,
                        col_i as f32 * TILE_SIZE + TILE_SIZE,
                        row_i as f32 * TILE_SIZE + TILE_SIZE,
                        1f32,
                        BLACK,
                    );
                    draw_line(
                        col_i as f32 * TILE_SIZE,
                        row_i as f32 * TILE_SIZE + TILE_SIZE,
                        col_i as f32 * TILE_SIZE + TILE_SIZE,
                        row_i as f32 * TILE_SIZE + TILE_SIZE,
                        1f32,
                        BLACK,
                    );

                    // Measuring the Number text that will be on the Tile
                    let n_dim = measure_text(
                        format!("{}", self.0[row_i][col_i]).as_str(),
                        Some(Font::default()),
                        25u16,
                        1f32,
                    );

                    // Drawing the Number Text on the Tile
                    draw_text(
                        format!("{}", self.0[row_i][col_i]).as_str(),
                        col_i as f32 * TILE_SIZE + TILE_SIZE * 0.5f32 - n_dim.width * 0.5f32,
                        row_i as f32 * TILE_SIZE + TILE_SIZE * 0.5f32 + n_dim.height * 0.5f32,
                        25f32,
                        BLACK,
                    );
                }
            }
        }
    }

    pub fn gen_rand_n_empty(&mut self) {
        let mut empty_rows_i: Vec<i32> = Vec::new();
        let mut empty_cell_i: Vec<Vec<i32>> = Vec::new();

        for row_i in 0..4 {
            if are_any_empty(&self.0[row_i]) {
                empty_rows_i.push(row_i as i32);
                empty_cell_i.push(Vec::new());
                let cell_ind = empty_cell_i.len() - 1;
                for cell_i in 0..4 {
                    if self.0[row_i][cell_i] == 0 {
                        empty_cell_i[cell_ind].push(cell_i as i32);
                    }
                }
            }
        }

        let random_i = rand::thread_rng().gen_range(0..empty_rows_i.len());
        let random_row_i = empty_rows_i[random_i];
        let random_cell_options = &empty_cell_i[random_i];
        let random_cell_options_len = random_cell_options.len();
        let random_cell_i =
            random_cell_options[rand::thread_rng().gen_range(0..random_cell_options_len)];
        self.0[random_row_i as usize][random_cell_i as usize] =
            RNG_CHOICE[rand::thread_rng().gen_range(0..10)];
    }

    pub fn can_move(&self) -> bool {
        for i in 0..self.0.len() * self.0.len() {
            let cell = self.0[get_index(i as i32)[0] as usize][get_index(i as i32)[1] as usize];
            if cell == 0 {
                return true;
            }
            if get_index(i as i32)[0] < 3
                && self.0[get_index(i as i32 + 4)[0] as usize][get_index(i as i32 + 4)[1] as usize]
                    == cell
            {
                return true;
            }
            if get_index(i as i32)[1] < 3
                && self.0[get_index(i as i32 + 1)[0] as usize][get_index(i as i32 + 1)[1] as usize]
                    == cell
            {
                return true;
            }
        }
        false
    }

    pub fn compress(&mut self, i_arr: &Vec<Vec<i32>>) {
        for arr in i_arr {
            for i in (1..arr.len()).rev() {
                for x in 1..=arr.len() - (arr.len() - i) {
                    let current_i_row = get_index(arr[i])[0] as usize;
                    let current_i_col = get_index(arr[i])[1] as usize;
                    let next_i_row = get_index(arr[i - x])[0] as usize;
                    let next_i_col = get_index(arr[i - x])[1] as usize;
                    if self.0[current_i_row][current_i_col] == 0
                        && self.0[next_i_row][next_i_col] != 0
                    {
                        self.0[current_i_row][current_i_col] =
                            self.0[next_i_row][next_i_col].clone();
                        self.0[next_i_row][next_i_col] = 0;
                        break;
                    }
                }
            }
        }
    }

    pub fn merge(&mut self, i_arr: &Vec<Vec<i32>>) {
        for arr in i_arr {
            for i in (1..arr.len()).rev() {
                if self.0[get_index(arr[i])[0] as usize][get_index(arr[i])[1] as usize]
                    == self.0[get_index(arr[i - 1 as usize])[0] as usize]
                        [get_index(arr[i - 1 as usize])[1] as usize]
                {
                    self.0[get_index(arr[i])[0] as usize][get_index(arr[i])[1] as usize] *= 2;
                    self.0[get_index(arr[i - 1 as usize])[0] as usize]
                        [get_index(arr[i - 1 as usize])[1] as usize] = 0;
                }
            }
        }
    }
}
