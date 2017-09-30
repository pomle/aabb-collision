import Rect from './Rect.js';
import Vec from './Vec.js';

const SPEED = 5;

const context = canvas.getContext('2d');
const desired = new Vec(0, 0);

const subject = new Rect(0, 0, 32, 32);
const obstacles = new Array(30).fill(0).map(x => new Rect(
    Math.random() * canvas.width,
    Math.random() * canvas.height,
    32 + Math.random() * 32,
    32 + Math.random() * 32));

function overlap(rect) {
    return subject.b > rect.t
        && subject.t < rect.b
        && subject.r > rect.l
        && subject.l < rect.r;
}

/* Iterate over all obstables that overlap subject. */
function intersection(fn) {
    obstacles.filter(overlap).forEach(fn);
}

function move(x, y) {
    subject.x += x;
    if (x > 0) {
        intersection(rect => {
            subject.r = rect.l;
        });
    } else if (x < 0) {
        intersection(rect => {
            subject.l = rect.r;
        });
    }

    subject.y += y;
    if (y > 0) {
        intersection(rect => {
            subject.b = rect.t;
        });
    } else if (y < 0) {
        intersection(rect => {
            subject.t = rect.b;
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

    drawRect(subject, 'red');

    obstacles.forEach(rect => {
        drawRect(rect);
    });
}

function update() {
    const moveTo = new Vec(desired.x - subject.x, desired.y - subject.y);
    if (moveTo.len > 5) {
        moveTo.len = SPEED;
        move(moveTo.x, moveTo.y);
    }

    draw();
    requestAnimationFrame(update);
}

canvas.addEventListener('mousemove', ({offsetX, offsetY}) => {
    desired.x = offsetX - subject.w / 2;
    desired.y = offsetY - subject.h / 2;
});

update();