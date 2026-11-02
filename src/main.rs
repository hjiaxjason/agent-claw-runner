mod game;

use game::Game;
use crossterm::{
    cursor::{Hide, Show},
    execute, queue,
    terminal::{BeginSynchronizedUpdate, EndSynchronizedUpdate, disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::{stdout, stdin, Read, Write},
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

// High enough that the world never moves more than one cell per frame,
// otherwise motion turns into uneven 1-cell / 2-cell jumps and looks shaky.
const FPS: u32 = 120;

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

    let frame = Duration::from_secs(1) / FPS;
    let mut last = Instant::now();
    let mut next_frame = last;

    'running : loop {  
        while let Ok(b) = rx.try_recv() {
            match b {
                b' ' => game.jump(),
                b'q' => break 'running,
                _ => {}
            }

        }               // ~5 seconds, then quit
        let now = Instant::now();
        let dt = (now - last).as_secs_f32().min(0.1);   // real time passed; cap it after a stall
        last = now;
        game.update(dt);
        queue!(out, BeginSynchronizedUpdate)?;
        game.draw(&mut out, 0, 0, cols, rows)?;
        queue!(out, EndSynchronizedUpdate)?;
        out.flush()?;

        next_frame += frame;                            // fixed schedule, not "sleep after work"
        match next_frame.checked_duration_since(Instant::now()) {
            Some(wait) => thread::sleep(wait),
            None => next_frame = Instant::now(),        // fell behind: don't try to catch up
        }
    }

    execute!(out, Show, LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}