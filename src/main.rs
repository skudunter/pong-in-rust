use minifb::{Key, Window, WindowOptions};
use rand::Rng;

const WIDTH: usize = 1400;
const HEIGHT: usize = 800;
const PADDLE_WIDTH: usize = 30;
const PADDLE_HEIGHT: usize = 140;
const BALL_WIDTH: usize = 40;
const PADDLE_SPEED: f64 = 1.5;
const DRAG: f64 = 0.85; // scales from 0 - 1
const MAX_SPEED: f64 = 10.0;
const SPEED_INCREASE: f64 = 1.15;

#[derive(Debug)]
struct Player {
    position: Vector2,
    dimensions: Vector2,
    velocity: Vector2,
    acceleration: Vector2,
    color: u32,
}

impl Player {
    fn new(x: f64, y: f64, w: f64, h: f64, vx: f64, vy: f64, ax: f64, ay: f64, color: u32) -> Self {
        Self {
            position: Vector2::new(x, y),
            dimensions: Vector2::new(w, h),
            velocity: Vector2::new(vx, vy),
            acceleration: Vector2::new(ax, ay),
            color,
        }
    }

    // eulerian physics update thing
    fn update(&mut self) {
        self.velocity.x += self.acceleration.x;
        self.velocity.y += self.acceleration.y;
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        //drag
        self.velocity.x *= DRAG;
        self.velocity.y *= DRAG;
        // clamp
        self.velocity.y = if self.velocity.y >= MAX_SPEED {
            MAX_SPEED
        } else {
            self.velocity.y
        };
        self.velocity.x = if self.velocity.x >= MAX_SPEED {
            MAX_SPEED
        } else {
            self.velocity.x
        };
        self.acceleration.x = 0.0;
        self.acceleration.y = 0.0;
        self.constrain();
    }

    // draws a rectangle starting in the top left think cartesian but y axis flipped
    // could bug when coords are negative ,usize?
    fn draw_rectangle(&self, buffer: &mut Vec<u32>) {
        for i in self.position.x as usize..self.position.x as usize + self.dimensions.x as usize {
            for j in self.position.y as usize..self.position.y as usize + self.dimensions.y as usize
            {
                let index = j * WIDTH + i;
                if index >= WIDTH * HEIGHT {
                    dbg!("error with rec pos");
                    break;
                }
                buffer[index] = self.color;
            }
        }
    }
    fn constrain(&mut self) {
        self.position.y = if self.position.y > (HEIGHT - PADDLE_HEIGHT) as f64 {
            (HEIGHT - PADDLE_HEIGHT) as f64
        } else {
            self.position.y
        };
        self.position.y = if self.position.y <= 0 as f64 {
            0.0
        } else {
            self.position.y
        };
    }
}

#[derive(Debug, Copy, Clone)]
struct Vector2 {
    x: f64,
    y: f64,
}

impl Vector2 {
    fn new(x: f64, y: f64) -> Self {
        Vector2 { x, y }
    }
}
fn main() {
    // main vars
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let game_color: u32 = from_u8_rgb(255, 255, 255);
    let background_color: u32 = from_u8_rgb(0, 0, 0);
    let mut _player1_score = 0;
    let mut _player2_score = 0;

    let mut player1 = Player::new(
        0.0,
        (HEIGHT / 2 - PADDLE_HEIGHT / 2) as f64,
        PADDLE_WIDTH as f64,
        PADDLE_HEIGHT as f64,
        0.0,
        0.0,
        0.0,
        0.0,
        game_color,
    );

    let mut player2 = Player::new(
        (WIDTH - PADDLE_WIDTH) as f64,
        (HEIGHT / 2 - PADDLE_HEIGHT / 2) as f64,
        PADDLE_WIDTH as f64,
        PADDLE_HEIGHT as f64,
        0.0,
        0.0,
        0.0,
        0.0,
        game_color,
    );

    let mut ball = Player::new(
        (WIDTH / 2 - BALL_WIDTH / 2) as f64,
        (HEIGHT / 2 - BALL_WIDTH / 2) as f64,
        BALL_WIDTH as f64,
        BALL_WIDTH as f64,
        rand_between_0_and_1(10.0),
        rand_between_0_and_1(5.0),
        0.0,
        0.0,
        game_color,
    );

    let mut window = Window::new("Pong Game", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    //main game loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        background(&mut buffer, background_color);

        // handle all keypress events
        window.get_keys().iter().for_each(|key| match key {
            Key::W => player1.acceleration.y = -PADDLE_SPEED,
            Key::S => player1.acceleration.y = PADDLE_SPEED,
            Key::Up => player2.acceleration.y = -PADDLE_SPEED,
            Key::Down => player2.acceleration.y = PADDLE_SPEED,
            _ => (),
        });

        player1.draw_rectangle(&mut buffer);
        player2.draw_rectangle(&mut buffer);
        ball.draw_rectangle(&mut buffer);

        player1.update();
        player2.update();

        ball.velocity.x += ball.acceleration.x;
        ball.velocity.y += ball.acceleration.y;
        ball.position.x += ball.velocity.x;
        ball.position.y += ball.velocity.y;

        //handle collisons w walls
        ball.velocity.y = if ball.position.y > (HEIGHT - BALL_WIDTH) as f64 {
            ball.position.y = (HEIGHT - BALL_WIDTH) as f64;
            -ball.velocity.y
        } else {
            ball.velocity.y
        };
        ball.velocity.y = if ball.position.y <= 0 as f64 {
            ball.position.y = 0.0;
            -ball.velocity.y
        } else {
            ball.velocity.y
        };

        // handle collions w paddles
        ball.velocity.x = if (ball.position.y >= player1.position.y
            && ball.position.y <= player1.position.y + player1.dimensions.y)
            && player1.position.x + player1.dimensions.x >= ball.position.x
        {
            ball.position.x = player1.position.x + player1.dimensions.x;
            -ball.velocity.x * SPEED_INCREASE
        } else {
            ball.velocity.x
        };

        ball.velocity.x = if (ball.position.y >= player2.position.y
            && ball.position.y <= player2.position.y + player1.dimensions.y)
            && player2.position.x <= ball.position.x + ball.dimensions.x
        {
            ball.position.x = player2.position.x - ball.dimensions.x;
            -ball.velocity.x * SPEED_INCREASE
        } else {
            ball.velocity.x
        };

        // score points
        _player1_score += if ball.position.x <= 0.0 {
            ball.position = Vector2::new(
                (WIDTH / 2 - BALL_WIDTH / 2) as f64,
                (HEIGHT / 2 - BALL_WIDTH / 2) as f64,
            );
            ball.velocity = Vector2::new(rand_between_0_and_1(10.0), rand_between_0_and_1(5.0));
            1
        } else {
            0
        };
        _player2_score += if ball.position.x + ball.dimensions.x >= WIDTH as f64 {
            ball.position = Vector2::new(
                (WIDTH / 2 - BALL_WIDTH / 2) as f64,
                (HEIGHT / 2 - BALL_WIDTH / 2) as f64,
            );
            ball.velocity = Vector2::new(rand_between_0_and_1(10.0), rand_between_0_and_1(5.0));
            1
        } else {
            0
        };

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

// converts from rgb to u32 for the buffer
fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

// draws background
fn background(buffer: &mut Vec<u32>, color: u32) {
    for i in buffer.iter_mut() {
        *i = color;
    }
}

fn rand_between_0_and_1(scale: f64) -> f64 {
    let mut rng = rand::thread_rng();
    let num1: f64 = rng.gen();
    let num2: f64 = rng.gen();
    if num1 >= 0.5 {
        num2 * scale
    } else {
        -1.0 * num2 * scale
    }
}
