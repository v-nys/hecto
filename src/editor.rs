use std::io::{self, Write}; // BIJHOUDEN: truukje self
use termion::event::Key;

use crate::terminal::Terminal;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

fn die(e: std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}

impl Editor {
    pub fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal."),
        }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        /* provided implementation:
        let pressed_key = read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        }
        Ok(())*/
        Terminal::read_key().map(|pressed| match pressed {
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        })
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();

        Terminal::cursor_position(0, 0);
        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(0, 0);
        }
        Terminal::cursor_show();
        io::stdout().flush()
    }

    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("Hecto editor -- version {}", VERSION);
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for row in 0..height - 1 {
            Terminal::clear_current_line();
            if row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }

    pub fn run(&mut self) {
        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }
            if self.should_quit {
                break;
            }
            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }
}
