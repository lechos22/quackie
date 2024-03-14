use pancurses::initscr;

mod geometry;

fn main() {
    let window = initscr();
    window.erase();
    window.printw("Hello, world!");
    window.getch();
}
