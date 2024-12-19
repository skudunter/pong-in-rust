use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 1400;
const HEIGHT: usize = 800;
const PADDLE_WIDTH: usize = 50;
const PADDLE_HEIGHT: usize = 120;
struct Rect {
    position: Vec2,
    dimensions: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    color: u32,
}

struct Vec2 {
    x: f64,
    y: f64,
}
fn main() {
    // main vars
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let game_color: u32 = from_u8_rgb(144, 233, 60);
    let background_color: u32 = from_u8_rgb(0, 0, 0);

    // let mut player1 = make_rect(
    //     0.0,
    //     (HEIGHT / 2) as f64,
    //     PADDLE_WIDTH as f64,
    //     PADDLE_HEIGHT as f64,
    //     0.0,
    //     0.0,
    //     0.0,
    //     0.0,
    //     game_color,
    // );

    let mut window = Window::new("Pong Game", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    //main game loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        background(&mut buffer, background_color);

        draw_rect(&mut buffer, 1350, 750, 50, 50, game_color);

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

// converts from rgb to u32 for the buffer
fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

// draws a draw_rectangle starting in the top left
fn draw_rect(buffer: &mut Vec<u32>, x: usize, y: usize, w: usize, h: usize, color: u32) {
    for i in x..x + w {
        for j in y..y + h {
            let index = j * WIDTH + i;
            if index > WIDTH * HEIGHT {
                println!("error with rec pos");
                break;
            }
            buffer[index] = color;
        }
    }
}
// draws background
fn background(buffer: &mut Vec<u32>, color: u32) {
    for i in buffer.iter_mut() {
        *i = color;
    }
}

// fn make_rect(
//     x: f64,
//     y: f64,
//     w: f64,
//     h: f64,
//     vx: f64,
//     vy: f64,
//     ax: f64,
//     ay: f64,
//     color: u32,
// ) -> Rect {
//     Rect {
//         x,
//         y,
//         w,
//         h,
//         vx,
//         vy,
//         ax,
//         ay,
//         color,
//     }
// }
