use glam::{vec2, Vec2};
use sdl2::keyboard::Keycode;
use sdl2::rect::FPoint;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{pixels::Color, rect::FRect};
use sdl2::event::Event;
use std::time::{Duration, Instant};

const BACKGROUND: Color = Color::RGB(15, 15, 15);
const FOREGROUND: Color = Color::RGB(127, 127, 127);

const SPEED: f32 = 500.0;
const BALL_SPEED: f32 = 700.0;

const NUMS: &'static [&'static [Vec2]] = &[
    &[vec2(-0.5, -1.0), vec2(0.5, -1.0), vec2(0.5, 1.0), vec2(-0.5, 1.0), vec2(-0.5, -1.0)],
    &[vec2(0.5, -1.0), vec2(0.5, 1.0)],
    &[vec2(-0.5, -1.0), vec2(0.5, -1.0), vec2(0.5, 0.0), vec2(-0.5, 0.0), vec2(-0.5, 1.0), vec2(0.5, 1.0)],
    &[vec2(-0.5, -1.0), vec2(0.5, -1.0), vec2(0.5, 0.0), vec2(-0.5, 0.0), vec2(0.5, 0.0), vec2(0.5, 1.0), vec2(-0.5, 1.0)],
    &[vec2(-0.5, -1.0), vec2(-0.5, 0.0), vec2(0.5, 0.0), vec2(0.5, -1.0), vec2(0.5, 1.0)],
    &[vec2(0.5, -1.0), vec2(-0.5, -1.0), vec2(-0.5, 0.0), vec2(0.5, 0.0), vec2(0.5, 1.0), vec2(-0.5, 1.0)],
    &[vec2(0.5, -1.0), vec2(-0.5, -1.0), vec2(-0.5, 0.0), vec2(0.5, 0.0), vec2(0.5, 1.0), vec2(-0.5, 1.0), vec2(-0.5, 0.0)],
    &[vec2(-0.5, -1.0), vec2(0.5, -1.0), vec2(0.5, 1.0)],
    &[vec2(-0.5, -1.0), vec2(0.5, -1.0), vec2(0.5, 0.0), vec2(-0.5, 0.0), vec2(0.5, 0.0), vec2(0.5, 1.0), vec2(-0.5, 1.0), vec2(-0.5, -1.0)],
    &[vec2(0.5, 0.0), vec2(-0.5, 0.0), vec2(-0.5, -1.0), vec2(0.5, -1.0), vec2(0.5, 1.0), vec2(-0.5, 1.0)],
    ];

struct Keys {
    w: bool,
    s: bool,
    a_u: bool,
    a_d: bool,
}

fn main() {
    let window_size = vec2(1000.0, 600.0);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Plong", window_size.x as u32, window_size.y as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut paddle_left = FRect::from_center((50.0, window_size.y as f32 / 2.0), 30.0, 200.0);
    let mut paddle_right = FRect::from_center((window_size.x - 50.0, window_size.y as f32 / 2.0), 30.0, 200.0);
    let mut ball = FRect::from_center((window_size.x as f32 / 2.0, window_size.y as f32 / 2.0), 25.0, 25.0);
    let mut ball_dir = vec2(1.0, 1.0);

    let mut score_left = 0;
    let mut score_right = 0;

    let mut keys = Keys {
        w: false,
        s: false,
        a_u: false,
        a_d: false,
    };

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(BACKGROUND);
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut last_frame = Instant::now();
    let mut delta = 0.0;
    'running: loop {
        canvas.set_draw_color(BACKGROUND);
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running
                }
                Event::KeyDown { keycode, ..} => {
                    match keycode {
                        Some(Keycode::W) => {
                            keys.w = true;
                        }
                        Some(Keycode::S) => {
                            keys.s = true;
                        }
                        Some(Keycode::Up) => {
                            keys.a_u = true;
                        }
                        Some(Keycode::Down) => {
                            keys.a_d = true;
                        }
                        _ => {}
                    }
                }
                Event::KeyUp {keycode, ..} => {
                    match keycode {
                        Some(Keycode::W) => {
                            keys.w = false;
                        }
                        Some(Keycode::S) => {
                            keys.s = false;
                        }
                        Some(Keycode::Up) => {
                            keys.a_u = false;
                        }
                        Some(Keycode::Down) => {
                            keys.a_d = false;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        paddle_left.set_y((paddle_left.y() + axis(keys.s, keys.w) * SPEED * delta).max(0.0).min(window_size.y as f32 - paddle_left.height()));
        paddle_right.set_y((paddle_right.y() + axis(keys.a_d, keys.a_u) * SPEED * delta).max(0.0).min(window_size.y as f32 - paddle_right.height()));

        ball.set_x(ball.x() + ball_dir.x * BALL_SPEED * delta);
        ball.set_y(ball.y() + ball_dir.y * BALL_SPEED * delta);

        if ball.right() > window_size.x {
            ball_dir.x = -1.0;
            ball.set_right(window_size.x);
            score_left += 1;
        } else if ball.x() < 0.0 {
            ball_dir.x = 1.0;
            ball.set_x(0.0);
            score_right += 1;
        }
        if ball.bottom() > window_size.y {
            ball_dir.y = -1.0;
            ball.set_bottom(window_size.y);
        } else if ball.y() < 0.0 {
            ball_dir.y = 1.0;
            ball.set_y(0.0);
        }

        if paddle_left.has_intersection(ball) {
            if ball.x() < paddle_left.right() - 20.0 {
                if paddle_left.y() + paddle_left.height() / 2.0 > ball.y() + ball.height() / 2.0 {
                    ball_dir.y = -1.0;
                    ball.set_bottom(paddle_left.y());
                } else {
                    ball_dir.y = 1.0;
                    ball.set_y(paddle_right.bottom());
                }
            } else {
                ball_dir.x = 1.0;
                ball.set_x(paddle_left.right());
            }
        } else if paddle_right.has_intersection(ball) {
            if ball.right() > paddle_right.x() + 20.0 {
                if paddle_right.y() + paddle_right.height() / 2.0 > ball.y() + ball.height() / 2.0 {
                    ball_dir.y = -1.0;
                    ball.set_bottom(paddle_right.y());
                } else {
                    ball_dir.y = 1.0;
                    ball.set_y(paddle_right.bottom());
                }
            } else {
                ball_dir.x = -1.0;
                ball.set_right(paddle_right.x());
            }
        }

        canvas.set_draw_color(FOREGROUND);
        canvas.fill_frect(ball).unwrap();
        canvas.fill_frect(paddle_left).unwrap();
        canvas.fill_frect(paddle_right).unwrap();

        render_num(&mut canvas, score_left, 30.0, vec2(window_size.x / 2.0 - 50.0, 50.0), -1.0);
        render_num(&mut canvas, score_right, 30.0, vec2(window_size.x / 2.0 + 50.0, 50.0), 1.0);

        canvas.present();
        let diff = Instant::now() - last_frame;
        delta = diff.as_secs_f32();
        if delta < 1.0/120.0 {
            delta = 1.0/120.0;
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 120) - diff);
        }
        last_frame = Instant::now();
    }
}

fn axis(positive: bool, negative: bool) -> f32 {
    match (positive, negative) {
        (true, true) => 0.0,
        (true, false) => 1.0,
        (false, true) => -1.0,
        (false, false) => 0.0,
    }
}

fn render_digit(canvas: &mut Canvas<Window>, digit: usize, scale: f32, offset: Vec2) {
    let mut points = vec![];
    for p in NUMS[digit] {
        points.push(FPoint::new(p.x * scale + offset.x, p.y * scale + offset.y));
    }
    canvas.draw_flines(points.as_slice()).unwrap();
}

fn get_digits(n: usize) -> Vec<usize> {
    fn x_inner(n: usize, xs: &mut Vec<usize>) {
        if n >= 10 {
            x_inner(n / 10, xs);
        }
        xs.push(n % 10);
    }
    let mut xs = Vec::new();
    x_inner(n, &mut xs);
    xs
}

fn render_num(canvas: &mut Canvas<Window>, num: usize, scale: f32, offset: Vec2, direction: f32) {
    let digits = get_digits(num);
    for (i, digit) in digits.iter().enumerate() {
        render_digit(canvas, *digit, scale, offset + vec2(i as f32 * scale * 2.0 - if direction < 0.0 { digits.len() as f32 * scale * 2.0 } else { 0.0 }, 0.0));
    }
}