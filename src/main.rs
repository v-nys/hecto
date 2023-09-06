use std::io::{self,Read}; // BIJHOUDEN: truukje self

fn main() {
    for b in io::stdin().bytes() {
        let char = b.unwrap() as char;
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
        println!("{char}");
    }
}
