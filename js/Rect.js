export default class Rect {
    constructor(x, y, w, h) {
        this.x = x;
        this.y = y;
        this.w = w;
        this.h = h;
    }

    get t() {
        return this.y;
    }

    get l() {
        return this.x;
    }

    get r() {
        return this.x + this.w;
    }

    get b() {
        return this.y + this.h;
    }

    set t(v) {
        this.y = v;
    }

    set l(v) {
        this.x = v;
    }

    set r(v) {
        this.x = v - this.w;
    }

    set b(v) {
        this.y = v - this.h;
    }
}
