use std::io::{self, Write}; // BIJHOUDEN: truukje self
use termion::event::Key;

use crate::terminal::Terminal;

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
        Terminal::clear_screen();
        Terminal::cursor_position(0, 0);
        if self.should_quit {
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(0, 0);
        }
        io::stdout().flush()
    }

    fn draw_rows(&self) {
        for _ in 0..self.terminal.size().height {
            println!("~\r");
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
