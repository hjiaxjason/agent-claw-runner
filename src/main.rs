mod game;

use game::Game;
use crossterm::{
    cursor::{Hide, Show},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::{stdout, stdin, Read, Write},
    sync::mpsc,
    thread,
    time::Duration,
};

fn main() -> std::io::Result<()> {
    let mut out = stdout();
    let (cols, rows) = size()?;
    let mut game = Game::new(cols);

    enable_raw_mode()?;
    execute!(out, EnterAlternateScreen, Hide)?;

    let (tx, rx) = mpsc::channel::<u8>();
    thread::spawn(move || {
        let mut buf = [0u8; 32];
        while let Ok(n) = stdin().read(&mut buf) {
            for &b in &buf[..n] {
                if tx.send(b).is_err() { return; }
            }
        }
    });

    let dt = 1.0 / 30.0;

    'running : loop {  
        while let Ok(b) = rx.try_recv() {
            match b {
                b' ' => game.jump(),
                b'q' => break 'running,
                _ => {}
            }

        }               // ~5 seconds, then quit
        game.update(dt);
        game.draw(&mut out, 0, 0, cols, rows)?;
        out.flush()?;
        thread::sleep(Duration::from_millis(33));
    }

    execute!(out, Show, LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}