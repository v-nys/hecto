use std::io::{self, stdout};

use termion::{
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};

pub struct Terminal {
    size: Size,
    _stdout: RawTerminal<std::io::Stdout>,
}

pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let (width, height) = termion::terminal_size()?;
        Ok(Self {
            size: Size { width, height },
            _stdout: stdout().into_raw_mode()?,
        })
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    pub fn cursor_position(x: u16, y: u16) {
        let x = x.saturating_add(1);
        let y = y.saturating_add(1);
        print!("{}", termion::cursor::Goto(x, y));
    }

    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            /* dit lijkt me vreemd
             * gebruiken Option (Some) omdat iterator ten einde kan zijn
             * maar als dat gebeurt, komen we hiermee in oneindige lus
             * al zou kunnen dat itereren over stdin ... keys nooit eindigt, dan is het geen probleem
             */
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }
}
