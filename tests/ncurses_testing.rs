fn base_windows() {

    let mw = pancurses::initscr();
    mw.draw_box(0, 0);

    let col = 1;
    let line = 1;

    for i in 0..10 {

        mw.mvprintw(line + i, col, format!("Line {}", i));

    }

    pancurses::noecho();

    mw.getch();
    
    mw.mv(5, 2);
    
    mw.insertln();

    mw.printw("xxx");

    mw.getch();

    pancurses::endwin();

}

#[cfg(test)]
mod tests {
    
    use super::*;
    
    #[test]
    #[ignore]
    fn windows_test1() {
        base_windows()
    }

}