mod clock;

use std::io::{stdout, Write};
use std::time::{Duration, Instant};

use anyhow::Result;
use chrono::Local;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{execute, queue};

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    execute!(stdout, Clear(ClearType::All))?;

    let exit_result = run_event_loop(&mut stdout);

    execute!(stdout, LeaveAlternateScreen)?;
    disable_raw_mode()?;

    exit_result
}

fn run_event_loop<W: Write>(stdout: &mut W) -> Result<()> {
    let mut last_render = Instant::now() - Duration::from_secs(10);

    loop {
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    _ => {}
                }
            }
        }

        if last_render.elapsed() >= Duration::from_millis(200) {
            let now = Local::now();
            let time_string = now.format("%H:%M:%S").to_string();
            let (width, height) = size()?;
            let frame = clock::render(&time_string, width, height);

            queue!(stdout, Clear(ClearType::All))?;
            for (row, line) in frame.iter().enumerate() {
                queue!(stdout, crossterm::cursor::MoveTo(0, row as u16))?;
                write!(stdout, "{line}")?;
            }
            stdout.flush()?;
            last_render = Instant::now();
        }
    }

    Ok(())
}
