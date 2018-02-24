use ncurses;

crate struct Ncurses { }

impl Ncurses {
    crate fn new() -> Self {
        ncurses::initscr();
        ncurses::raw();
        ncurses::noecho();
        Ncurses { }
    }
}

impl Drop for Ncurses {
    fn drop(&mut self) {
        ncurses::endwin();
    }
}
