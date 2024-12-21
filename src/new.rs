use minifb::{Key, Window, WindowOptions};
use std::f64::consts::PI;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const PADDLE_WIDTH: usize = 20;
const PADDLE_HEIGHT: usize = 100;
const BALL_SIZE: usize = 10;
const PADDLE_SPEED: f64 = 5.0;
const BALL_SPEED: f64 = 4.0;
const BACKGROUND_COLOR: u32 = 0x000000;
const PADDLE_COLOR: u32 = 0xFFFFFF;
const BALL_COLOR: u32 = 0xFF0000;

#[derive(Debug, Default, Clone, Copy)]
struct Vector2 {
    x: f64,
    y: f64,
}

impl Vector2 {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn clamp(&mut self, min: f64, max: f64) {
        self.x = self.x.clamp(min, max);
        self.y = self.y.clamp(min, max);
    }
}

#[derive(Debug, Clone)]
struct Paddle {
    position: Vector2,
    velocity: Vector2,
    dimensions: Vector2,
}

impl Paddle {
    fn new(x: f64, y: f64) -> Self {
        Self {
            position: Vector2::new(x, y),
            velocity: Vector2::default(),
            dimensions: Vector2::new(PADDLE_WIDTH as f64, PADDLE_HEIGHT as f64),
        }
    }

    fn update(&mut self, direction: f64) {
        self.velocity.y = direction * PADDLE_SPEED;
        self.position.y += self.velocity.y;
        self.position.y = self.position.y.clamp(0.0, (HEIGHT - PADDLE_HEIGHT) as f64);
    }

    fn draw(&self, buffer: &mut [u32]) {
        let x_start = self.position.x as usize;
        let y_start = self.position.y as usize;
        for y in y_start..y_start + PADDLE_HEIGHT {
            for x in x_start..x_start + PADDLE_WIDTH {
                if let Some(pixel) = buffer.get_mut(y * WIDTH + x) {
                    *pixel = PADDLE_COLOR;
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Ball {
    position: Vector2,
    velocity: Vector2,
    dimensions: Vector2,
}

impl Ball {
    fn new(x: f64, y: f64) -> Self {
        Self {
            position: Vector2::new(x, y),
            velocity: Vector2::new(BALL_SPEED, BALL_SPEED),
            dimensions: Vector2::new(BALL_SIZE as f64, BALL_SIZE as f64),
        }
    }

    fn update(&mut self, player1: &Paddle, player2: &Paddle) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;

        // Wall collision
        if self.position.y <= 0.0 || self.position.y >= (HEIGHT - BALL_SIZE) as f64 {
            self.velocity.y = -self.velocity.y;
        }

        // Paddle collision
        if self.check_collision(player1) {
            self.velocity.x = BALL_SPEED;
        } else if self.check_collision(player2) {
            self.velocity.x = -BALL_SPEED;
        }
    }

    fn check_collision(&self, paddle: &Paddle) -> bool {
        self.position.x < paddle.position.x + paddle.dimensions.x
            && self.position.x + self.dimensions.x > paddle.position.x
            && self.position.y < paddle.position.y + paddle.dimensions.y
            && self.position.y + self.dimensions.y > paddle.position.y
    }

    fn draw(&self, buffer: &mut [u32]) {
        let x_start = self.position.x as usize;
        let y_start = self.position.y as usize;
        for y in y_start..y_start + BALL_SIZE {
            for x in x_start..x_start + BALL_SIZE {
                if let Some(pixel) = buffer.get_mut(y * WIDTH + x) {
                    *pixel = BALL_COLOR;
                }
            }
        }
    }
}

fn main() {
    let mut window = Window::new(
        "Pong Game",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .expect("Unable to open window");

    let mut buffer = vec![BACKGROUND_COLOR; WIDTH * HEIGHT];

    let mut player1 = Paddle::new(30.0, (HEIGHT / 2 - PADDLE_HEIGHT / 2) as f64);
    let mut player2 = Paddle::new((WIDTH - 50) as f64, (HEIGHT / 2 - PADDLE_HEIGHT / 2) as f64);
    let mut ball = Ball::new((WIDTH / 2) as f64, (HEIGHT / 2) as f64);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        buffer.fill(BACKGROUND_COLOR);

        // Player 1 movement
        let p1_direction = if window.is_key_down(Key::W) {
            -1.0
        } else if window.is_key_down(Key::S) {
            1.0
        } else {
            0.0
        };
        player1.update(p1_direction);

        // Player 2 movement
        let p2_direction = if window.is_key_down(Key::Up) {
            -1.0
        } else if window.is_key_down(Key::Down) {
            1.0
        } else {
            0.0
        };
        player2.update(p2_direction);

        // Ball movement
        ball.update(&player1, &player2);

        // Draw entities
        player1.draw(&mut buffer);
        player2.draw(&mut buffer);
        ball.draw(&mut buffer);

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
