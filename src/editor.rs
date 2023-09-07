use std::io::{self, stdout}; // BIJHOUDEN: truukje self
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {}

fn die(e: std::io::Error) {
    panic!("{}", e);
}

fn read_key() -> Result<Key, std::io::Error> {
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

impl Editor {
    pub fn default() -> Self {
        Self {}
    }

    fn process_keypress(&self) -> Result<(), std::io::Error> {
        let pressed_key = read_key()?;
        match pressed_key {
            Key::Ctrl('q') => panic!("Program end"),
            _ => (),
        }
        Ok(())
    }

    pub fn run(&self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        loop {
            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }
}
