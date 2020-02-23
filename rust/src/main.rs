extern crate piston_window;
mod rect;

use piston_window::*;
use rand::prelude::*;
use rayon::prelude::*;
use rect::Rect;

fn overlap(a: &Rect, b: &Rect) -> bool {
    return a.bottom() > b.top()
        && a.top() < b.bottom()
        && a.right() > b.left()
        && a.left() < b.right();
}

fn intersection<'a>(obstacles: &'a [Rect], follower: Rect) -> impl Iterator<Item = &'a Rect> {
    obstacles
        .iter()
        .filter(move |rect| overlap(rect, &follower))
}

pub fn move_rect(obstacles: &[Rect], subject: &mut Rect, x: f64, y: f64) {
    subject.x += x;
    if x > 0.0 {
        intersection(obstacles, subject.clone()).for_each(|overlap| {
            if subject.right() > overlap.left() {
                subject.set_right(overlap.left());
            }
        });
    } else if x < 0.0 {
        intersection(obstacles, subject.clone()).for_each(|overlap| {
            if subject.left() < overlap.right() {
                subject.set_left(overlap.right());
            }
        });
    }

    subject.y += y;
    if y > 0.0 {
        intersection(obstacles, subject.clone()).for_each(|overlap| {
            if subject.bottom() < overlap.top() {
                subject.set_bottom(overlap.top());
            }
        });
    } else if y < 0.0 {
        intersection(obstacles, subject.clone()).for_each(|overlap| {
            if subject.top() < overlap.bottom() {
                subject.set_top(overlap.bottom());
            }
        });
    }
}

fn distance(v: [f64; 2]) -> f64 {
    (v[0] * v[0] + v[1] * v[1]).sqrt()
}

fn main() {
    let win = [1280, 720];
    let builder = WindowSettings::new("Hello Piston!", win);
    let mut window: PistonWindow = builder.exit_on_esc(true).build().unwrap();

    let mut rng = rand::thread_rng();
    let mut followers: Vec<Rect> = (0..1000)
        .map(|_| {
            let size = 10.0 + rng.gen::<f64>() * 50.0;
            Rect::new(
                rng.gen::<f64>() * win[0] as f64,
                rng.gen::<f64>() * win[1] as f64,
                size,
                size,
            )
        })
        .collect();

    let obstacles: Vec<Rect> = (0..30)
        .map(|_| {
            let size = 32.0 + rng.gen::<f64>() * 32.0;
            Rect::new(
                rng.gen::<f64>() * win[0] as f64,
                rng.gen::<f64>() * win[1] as f64,
                size,
                size,
            )
        })
        .collect();

    let mut desired = [0.0, 0.0];
    while let Some(event) = window.next() {
        event.mouse_cursor(|pos| {
            desired = pos;
        });

        followers.par_iter_mut().for_each(|subject| {
            let mut move_to = [desired[0] - subject.x, desired[1] - subject.y];
            if distance(move_to) > 5.0 {
                let slow_down = subject.width * subject.height * 0.2;
                move_to[0] /= slow_down;
                move_to[1] /= slow_down;
                move_rect(&obstacles, subject, move_to[0], move_to[1]);
            }
        });

        window.draw_2d(&event, |context, graphics, _device| {
            clear([0.5, 0.5, 0.5, 1.0], graphics);
            followers.iter().for_each(|rect| {
                rectangle(
                    [1.0, 0.0, 0.0, 1.0], // red
                    [rect.x, rect.y, rect.width, rect.height],
                    context.transform,
                    graphics,
                );
            });

            obstacles.iter().for_each(|rect| {
                rectangle(
                    [0.0, 0.0, 0.0, 1.0], // black
                    [rect.x, rect.y, rect.width, rect.height],
                    context.transform,
                    graphics,
                );
            });
        });
    }
}
