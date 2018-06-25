extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

fn main() {
    // Get the standard input stream.
    let stdin = stdin();
    // Get the standard output stream and go to raw mode.
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut text = Vec::new();

    write!(stdout, "{}{}q to exit. Type stuff, use alt, and so on.",
           // Clear the screen.
           termion::clear::All,
           // Goto (1,1).
           termion::cursor::Goto(1, 1)).unwrap();
    // Flush stdout (i.e. make the output appear).
    stdout.flush().unwrap();

    for c in stdin.keys() {
        // Clear the current line.
        write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1),).unwrap();

        // Print the key we type...
        match c.unwrap() {
            // Exit.
            Key::Ctrl(c)   => break,
            Key::Char(c)   => text.push(c),
            Key::Backspace => {
                text.pop();
                ()
            },
            _              => ()
        }

        for ch in &text {
            write!(stdout, "{}", ch);
        }

        // Flush again.
        stdout.flush().unwrap();
    }

    // Show the cursor again before we exit.
    write!(stdout, "{}", termion::clear::All).unwrap();
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
