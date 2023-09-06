use std::io::{self, stdout, Read}; // BIJHOUDEN: truukje self
use termion::raw::IntoRawMode;

fn to_ctrl_byte(c: char) -> u8 {
    let b = c as u8;
    b & 0b0001_1111
}

fn die(e: std::io::Error) {
    panic!("{}", e);
}

fn main() {
    // BIJHOUDEN: moet op stdout, niet stdin, omdat terminals gecontroleerd worden door hun writer
    // BIJHOUDEN: toekenning is nodig om de raw terminal niet te droppen
    let _stdout = stdout().into_raw_mode().unwrap();
    for b in io::stdin().bytes() {
        match b {
            Ok(b) => {
                let c = b as char;
                if c.is_control() {
                    println!("{:?} \r", b);
                } else {
                    println!("{:?} ({})\r", b, c);
                }
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
                if b == to_ctrl_byte('q') {
                    break;
                }
            }
            Err(e) => die(e),
        }
    }
}
