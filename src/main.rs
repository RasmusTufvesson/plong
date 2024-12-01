use glam::{vec2, Vec2};
use rand::{thread_rng, Rng};
use sdl2::keyboard::Keycode;
use sdl2::rect::FPoint;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{pixels::Color, rect::FRect};
use sdl2::event::Event;
use std::collections::HashMap;
use std::time::{Duration, Instant};

const BACKGROUND: Color = Color::RGB(15, 15, 15);
const FOREGROUND: Color = Color::RGB(127, 127, 127);

const SPEED: f32 = 800.0;
const BALL_SPEED: f32 = 600.0;

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
    inner: HashMap<Keycode, bool>,
}

impl Keys {
    fn new(keys: Vec<Keycode>) -> Self {
        let mut inner = HashMap::new();
        for key in keys {
            inner.insert(key, false);
        }
        Self { inner }
    }

    fn key_down(&mut self, keycode: Keycode) {
        if self.inner.contains_key(&keycode) {
            self.inner.insert(keycode, true);
        }
    }

    fn key_up(&mut self, keycode: Keycode) {
        if self.inner.contains_key(&keycode) {
            self.inner.insert(keycode, false);
        }
    }

    fn pressed(&self, keycode: Keycode) -> bool {
        *self.inner.get(&keycode).unwrap()
    }
}

fn main() {
    let mut rng = thread_rng();

    let window_size = vec2(1000.0, 600.0);

    let mut center_lines = [(FPoint::new(window_size.x / 2.0, 0.0), FPoint::new(window_size.x / 2.0, 0.0)); 10];
    for (i, (start, end)) in center_lines.iter_mut().enumerate() {
        start.y = window_size.y / 10.0 * i as f32;
        end.y = start.y + window_size.y / 20.0;
    }

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Plong", window_size.x as u32, window_size.y as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut paddle_left = FRect::from_center((50.0, window_size.y as f32 / 2.0), 30.0, 200.0);
    let mut paddle_right = FRect::from_center((window_size.x - 50.0, window_size.y as f32 / 2.0), 30.0, 200.0);
    let mut ball = FRect::from_center((window_size.x as f32 / 2.0, window_size.y as f32 / 2.0), 25.0, 25.0);
    let mut ball_speed = vec2(BALL_SPEED, BALL_SPEED);

    let mut score_left = 0;
    let mut score_right = 0;

    let mut keys = Keys::new(vec![Keycode::W, Keycode::S, Keycode::Up, Keycode::Down]);

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
                Event::KeyDown { keycode, ..} => if let Some(keycode) = keycode { keys.key_down(keycode) },
                Event::KeyUp {keycode, ..} => if let Some(keycode) = keycode { keys.key_up(keycode) },
                _ => {}
            }
        }

        paddle_left.set_y((paddle_left.y() + axis(keys.pressed(Keycode::S), keys.pressed(Keycode::W)) * SPEED * delta)
            .max(0.0).min(window_size.y as f32 - paddle_left.height()));
        paddle_right.set_y((paddle_right.y() + axis(keys.pressed(Keycode::Down), keys.pressed(Keycode::Up)) * SPEED * delta)
            .max(0.0).min(window_size.y as f32 - paddle_right.height()));

        ball.set_x(ball.x() + ball_speed.x * delta);
        ball.set_y(ball.y() + ball_speed.y * delta);

        if ball.x > window_size.x {
            ball.x = window_size.x / 2.0 - ball.h / 2.0;
            ball.y = rng.gen_range((window_size.y * 0.1)..(window_size.y * 0.9));
            ball_speed = vec2(BALL_SPEED * rng.gen_range(0.7..1.9), BALL_SPEED * rng.gen_range(0.7..1.9) * if rng.gen::<bool>() { 1.0 } else { -1.0 });
            score_left += 1;
        } else if ball.right() < 0.0 {
            ball.x = window_size.x / 2.0 - ball.h / 2.0;
            ball.y = rng.gen_range((window_size.y * 0.1)..(window_size.y * 0.9));
            ball_speed = vec2(-BALL_SPEED * rng.gen_range(0.7..1.9), BALL_SPEED * rng.gen_range(0.7..1.9) * if rng.gen::<bool>() { 1.0 } else { -1.0 });
            score_right += 1;
        }
        if ball.bottom() > window_size.y {
            ball_speed.y = -ball_speed.y.abs();
            ball.set_bottom(window_size.y);
        } else if ball.y < 0.0 {
            ball_speed.y = ball_speed.y.abs();
            ball.set_y(0.0);
        }

        if paddle_left.has_intersection(ball) {
            if ball.x() < paddle_left.right() - 20.0 {
                deflect_y(&mut ball, &mut ball_speed, paddle_left);
            } else {
                ball_speed.x = ball_speed.x.abs() * 1.005;
                ball_speed.y += axis(keys.pressed(Keycode::S), keys.pressed(Keycode::W)) * SPEED * 0.1;
                ball.set_x(paddle_left.right());
            }
        } else if paddle_right.has_intersection(ball) {
            if ball.right() > paddle_right.x() + 20.0 {
                deflect_y(&mut ball, &mut ball_speed, paddle_right);
            } else {
                ball_speed.x = -ball_speed.x.abs() * 1.005;
                ball_speed.y += axis(keys.pressed(Keycode::Down), keys.pressed(Keycode::Up)) * SPEED * 0.1;
                ball.set_right(paddle_right.x());
            }
        }

        canvas.set_draw_color(FOREGROUND);
        for line in &center_lines {
            canvas.draw_fline(line.0, line.1).unwrap();
        }

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

fn deflect_y(ball: &mut FRect, ball_speed: &mut Vec2, paddle: FRect) {
    if paddle.y() + paddle.height() / 2.0 > ball.y() + ball.height() / 2.0 {
        ball_speed.y = -ball_speed.y.abs();
        ball.set_bottom(paddle.y());
    } else {
        ball_speed.y = ball_speed.y.abs();
        ball.set_y(paddle.bottom());
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
        render_digit(canvas, *digit, scale, offset + vec2(i as f32 * scale * 2.0 - if direction < 0.0 { (digits.len() - 1) as f32 * scale * 2.0 } else { 0.0 }, 0.0));
    }
}