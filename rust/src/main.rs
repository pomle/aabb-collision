extern crate piston_window;
mod rect;

use piston_window::*;
use rand::prelude::*;
use rect::Rect;

fn overlap(a: &Rect, b: &Rect) -> bool {
    return a.bottom() > b.top()
        && a.top() < b.bottom()
        && a.right() > b.left()
        && a.left() < b.right();
}

fn intersection<'a>(obstacles: &'a [Rect], follower: &'a Rect) -> impl Iterator<Item = &'a Rect> {
    obstacles.iter().filter(move |rect| overlap(rect, follower))
}


fn move(subject: &Rect, x: f64, y: f64) {
    subject.x += x;
    subject.y += y;
}

fn main() {
    let win = [1280, 720];

    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", win)
        .exit_on_esc(true).build().unwrap();

    let mut rng = rand::thread_rng();

    let followers: Vec<Rect> = (0..10).map(| index | {
        let size = 10.0 + rng.gen::<f64>() * 50.0;
        Rect::new(
            rng.gen::<f64>() * win[0] as f64,
            rng.gen::<f64>() * win[1] as f64,
            size, size
        )
    }).collect();

    let obstacles: Vec<Rect> = (0..30).map(| index | {
        let size = 32.0 + rng.gen::<f64>() * 32.0;
        Rect::new(
            rng.gen::<f64>() * win[0] as f64,
            rng.gen::<f64>() * win[1] as f64,
            size, size
        )
    }).collect();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            for rect in &followers {
                rectangle([1.0, 0.0, 0.0, 1.0], // red
                        [rect.x, rect.y, rect.width, rect.height],
                        context.transform,
                        graphics);
            }

            for rect in &obstacles {
                rectangle([0.0, 0.0, 0.0, 1.0], // black
                        [rect.x, rect.y, rect.width, rect.height],
                        context.transform,
                        graphics);
            }
        });
    }
}
