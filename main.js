import Rect from './Rect.js';
import Vec from './Vec.js';

function rand(max) {
    return Math.random() * max;
}

const SPEED = 5;

canvas.width = document.body.offsetWidth;
canvas.height = document.body.offsetHeight;

const context = canvas.getContext('2d');
const desired = new Vec(0, 0);

const followers = new Array(10).fill(0).map(() => {
    const s = 5 + rand(32);
    const rect = new Rect(
        rand(canvas.width),
        rand(canvas.height),
        s,
        s);
    rect.weight = rect.w * rect.h;
    return rect;
});

const obstacles = new Array(30).fill(0).map(() => new Rect(
    rand(canvas.width),
    rand(canvas.height),
    32 + rand(32),
    32 + rand(32)));

function overlap(subject, rect) {
    return subject.b > rect.t
        && subject.t < rect.b
        && subject.r > rect.l
        && subject.l < rect.r;
}

/* Iterate over all obstables that overlap subject. */
function intersection(subject, fn) {
    obstacles.filter(obstacle => overlap(subject, obstacle)).forEach(fn);
}

function move(subject, x, y) {
    subject.x += x;

    if (x > 0) {
        intersection(subject, rect => {
            if (subject.r > rect.l) {
                subject.r = rect.l;
            }
        });
    } else if (x < 0) {
        intersection(subject, rect => {
            if (subject.l < rect.r) {
                subject.l = rect.r;
            }
        });
    }

    subject.y += y;
    if (y > 0) {
        intersection(subject, rect => {
            if (subject.b > rect.t) {
                subject.b = rect.t;
            }
        });
    } else if (y < 0) {
        intersection(subject, rect => {
            if (subject.t < rect.b) {
                subject.t = rect.b;
            }
        });
    }

    draw();
}

function drawRect(rect, color = '#000') {
    context.fillStyle = color;
    context.fillRect(rect.x, rect.y, rect.w, rect.h);
}

function draw() {
    context.clearRect(0, 0, canvas.width, canvas.height);

    followers.forEach(subject => {
        drawRect(subject, 'red');
    });

    obstacles.forEach(rect => {
        drawRect(rect);
    });
}

function update() {
    followers.forEach(subject => {
        const moveTo = new Vec(desired.x - subject.x, desired.y - subject.y);
        if (moveTo.len > 5) {
            moveTo.len /= subject.weight * 0.2;
            move(subject, moveTo.x, moveTo.y);
        }
    });

    draw();
    requestAnimationFrame(update);
}

canvas.addEventListener('mousemove', ({offsetX, offsetY}) => {
    desired.x = offsetX;
    desired.y = offsetY;
});

update();