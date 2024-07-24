use std::io::Write;

use crossterm::{
    execute, queue,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

fn main() {
    enable_raw_mode().unwrap();

    let mut stdout = std::io::stdout();

    // hide cursor
    execute!(stdout, EnterAlternateScreen, crossterm::cursor::Hide).unwrap();

    // add basic game loop with key input and screen clearing with transparent background
    loop {
        stdout.flush().unwrap();
        if crossterm::event::poll(std::time::Duration::from_millis(33)).unwrap() {
            if let crossterm::event::Event::Key(event) = crossterm::event::read().unwrap() {
                if event.code == crossterm::event::KeyCode::Char('q') {
                    break;
                }
            }
        }
        queue!(
            stdout,
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
            crossterm::cursor::MoveTo(0, 0),
            crossterm::style::Print("FPS: 30 :D")
        )
        .unwrap();
    }

    execute!(stdout, crossterm::cursor::Show, LeaveAlternateScreen).unwrap();

    disable_raw_mode().unwrap();
}
