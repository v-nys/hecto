use std::io::{self, stdout}; // BIJHOUDEN: truukje self
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn die(e: std::io::Error) {
    panic!("{}", e);
}

fn main() {
    /* BIJHOUDEN:
    However, by default your terminal starts in canonical mode, also called cooked mode.
    In this mode, keyboard input is only sent to your program when the user presses ENTER.
    This is useful for many programs: it lets the user type in a line of text,
    use BACKSPACE to fix errors until they get their input exactly the way they want it,
    and finally press ENTER to send it to the program.
    But it does not work well for programs with more complex user interfaces, like text editors.
    We want to process each key press as it comes in, so we can respond to it immediately.
    What we want is raw mode.
     */
    // BIJHOUDEN: moet op stdout, niet stdin, omdat terminals gecontroleerd worden door hun writer
    // BIJHOUDEN: toekenning is nodig om de raw terminal niet te droppen
    let _stdout = stdout().into_raw_mode().unwrap();
    for key in io::stdin().keys() {
        match key {
            Ok(key) => match key {
                Key::Char(c) => {
                    if c.is_control() {
                        println!("{:?} \r", c as u8);
                    } else {
                        println!("{:?} ({})\r", c as u8, c);
                    }
                }
                Key::Ctrl('q') => break,
                _ => println!("{:?}\r", key),
            },
            Err(e) => die(e),
        }
    }
}
