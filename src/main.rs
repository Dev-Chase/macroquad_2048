use macroquad::prelude::*;

// Including Information from Other Files Here
mod board;
use board::Board;

enum GameState {
    Setup,
    Start,
    Play,
    Pause,
    Gameover,
}

// Creating a Function to Configurate the Window
fn create_window_conf() -> Conf {
    Conf {
        window_title: String::from("2048"),
        fullscreen: false,
        window_width: 500,
        window_height: 500,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(create_window_conf())]
async fn main() {
    // Creating Board
    let mut board = Board([[0;4];4]);

    // Creating GameState Variable
    let mut game_state = GameState::Start;

    // Creating Variables to Control Frame Rate
    let mut minimum_frame_time: f32;
    let mut frame_time: f32;
    let mut time_to_sleep = 0f32;

    loop {
        // Clearing the Window with a Colour
        clear_background(BLACK);

        // Calling Corresponding GameState Function
        game_state = match game_state {
            GameState::Start => start(),
            GameState::Pause => pause(),
            GameState::Play => play(&mut board),
            GameState::Gameover => gameover(),
            GameState::Setup => setup(&mut board),
        };

        // Capping the Frame Rate
        minimum_frame_time = 1. / 60.; // 60 FPS
        frame_time = get_frame_time()-time_to_sleep*0.001f32;
        time_to_sleep = (minimum_frame_time - frame_time) * 1000.;
        std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));

        next_frame().await
    }
}

// Creating a Function to Get the List of Indexes for Everymovement
fn get_i_arr_for_movement(dir: (i32, i32)) -> Vec<Vec<i32>>{
    // Creating the Vector that will hold the indexes
    let mut i_arr: Vec<Vec<i32>> = Vec::new();
    if dir.0 == 1{
        for i in 0..4 {
            i_arr.push(Vec::new());
            for x in 0..4 {
                i_arr[i].push(i as i32*4+x as i32);
            }
        }
    } else if dir.0 == -1 {
        for i in 0..4 {
            i_arr.push(Vec::new());
            for x in 0..4 {
                i_arr[i].push(i as i32*4+x as i32);
            }
            i_arr[i].reverse();
        }
    } else if dir.1 == 1 {
        for i in 0..4 {
            i_arr.push(Vec::new());
            for x in 0..4 {
                i_arr[i].push(x as i32*4+i as i32);
            }
        }
    } else {
        for i in 0..4 {
            i_arr.push(Vec::new());
            for x in 0..4 {
                i_arr[i].push(x as i32*4+i as i32);
            }
            i_arr[i].reverse();
        }
    }
    i_arr
}

// Creating a Function to Run if the Game State is Setup
fn setup(board: &mut Board) -> GameState {
    *board = Board([[0;4];4]);
    
    // Generating 2 Random Numbers on the board
    board.gen_rand_n_empty();
    board.gen_rand_n_empty();

    GameState::Play
}

// Creating a Function to Run if the Game State is Start
fn start() -> GameState {
    // Getting Text Section Dimensions
    let start_text_dimensions = measure_text("Press Space or Click to Start", Some(Font::default()), 25u16, 1f32);

    // Drawing Text Sections
    draw_text("Press Space or Click to Start", screen_width()*0.5f32-start_text_dimensions.width*0.5f32, screen_height()*0.5f32-start_text_dimensions.height*0.5f32, 25f32, WHITE);

    // Checking to see if Space Key Pressed or Left mouse button clicked
    if is_key_released(KeyCode::Space) || is_mouse_button_down(MouseButton::Left) {
        return GameState::Setup
    }
    GameState::Start
}


// Creating a Function to Run if the Game State is Gameover
fn gameover() -> GameState {
    // Getting Text Section Dimensions
    let gameover_text_dimensions = measure_text("Gameover.", Some(Font::default()), 30u16, 1f32);
    let restart_text_dimensions = measure_text("Press Space or Click to Restart", Some(Font::default()), 30u16, 1f32);
    
    // Drawing Text Sections
    draw_text("Gameover.", screen_width()*0.5f32-gameover_text_dimensions.width*0.5f32, screen_height()*0.5f32-restart_text_dimensions.height*0.5f32-gameover_text_dimensions.height-5f32, 30f32, WHITE);
    draw_text("Press Space or Click to Restart", screen_width()*0.5f32-restart_text_dimensions.width*0.5f32, screen_height()*0.5f32-restart_text_dimensions.height*0.5f32, 30f32, WHITE);


    // Checking to See if Space key pressed or Left mouse button clicked
    if is_key_released(KeyCode::Space) || is_mouse_button_down(MouseButton::Left) {
        return GameState::Setup
    }
    GameState::Gameover
}

// Creating a Function to Run if the Game State is Play
fn play(board: &mut Board) -> GameState {
    // Checking to see if the Space Key was Released and if so Pausing the Game
    if is_key_released(KeyCode::Space){
        return GameState::Pause;
    }

    if is_key_pressed(KeyCode::Right) && board.can_move(){
        let i_arr = get_i_arr_for_movement((1, 0));
        board.compress(&i_arr);
        board.merge(&i_arr);
        board.compress(&i_arr);
        board.gen_rand_n_empty();
    }
    if is_key_pressed(KeyCode::Left) && board.can_move() {
        let i_arr = get_i_arr_for_movement((-1, 0));
        board.compress(&i_arr);
        board.merge(&i_arr);
        board.compress(&i_arr);
        board.gen_rand_n_empty();
    }
    if is_key_pressed(KeyCode::Up) && board.can_move() {
        let i_arr = get_i_arr_for_movement((0, -1));
        board.compress(&i_arr);
        board.merge(&i_arr);
        board.compress(&i_arr);
        board.gen_rand_n_empty();
    }
    if is_key_pressed(KeyCode::Down) && board.can_move() {
        let i_arr = get_i_arr_for_movement((0, 1));
        board.compress(&i_arr);
        board.merge(&i_arr);
        board.compress(&i_arr);
        board.gen_rand_n_empty();
    }

    //Most of the Game Code Goes here
    board.draw();


    if !board.can_move() {
        return GameState::Gameover;
    }


    GameState::Play
}

// Creating a Function to Run if the Game State is Pause
fn pause() -> GameState {
    // Getting Text Section Dimensions
    let pause_text_dimensions = measure_text("Game Paused", Some(Font::default()), 30u16, 1f32);
    let unpause_text_dimensions = measure_text("Press Space or Click to Unpause", Some(Font::default()), 30u16, 1f32);
    
    // Drawing Text Sections
    draw_text("Game Paused", screen_width()*0.5f32-pause_text_dimensions.width*0.5f32, screen_height()*0.5f32-unpause_text_dimensions.height*0.5f32-pause_text_dimensions.height-5f32, 30f32, WHITE);
    draw_text("Press Space or Click to Unpause", screen_width()*0.5f32-unpause_text_dimensions.width*0.5f32, screen_height()*0.5f32-unpause_text_dimensions.height*0.5f32, 30f32, WHITE);

    // Checking to see if the Space Key was Released or If the Left Mouse Button was Clicked and if so Unpausing the Game
    if is_key_released(KeyCode::Space) || is_mouse_button_down(MouseButton::Left) {
        return GameState::Play;
    }

    GameState::Pause
}


