use std::io::{stdin, stdout, Write};
use termion::{
    clear,
    cursor::*,
    event::*,
    input::{MouseTerminal, TermRead},
    raw::IntoRawMode,
};

// icon for the cursor
const CURSOR_ICON: char = '⬤';

// background color for status bar
const BG_PURP: &str = "\x1b[45m";
const WHITE: &str = "\x1b[97m";
const RESET: &str = "\x1b[0m";

// colors to draw with :
// [ ["forground", "background"], ... ]
const COLORS: [[&str; 2]; 6] = [
    ["\x1b[45m", "\x1b[95m"],
    ["\x1b[44m", "\x1b[94m"],
    ["\x1b[42m", "\x1b[92m"],
    ["\x1b[41m", "\x1b[91m"],
    ["\x1b[43m", "\x1b[93m"],
    ["\x1b[47m", "\x1b[97m"],
];

// this function prints the status bar at the bottom of the terminal
// parameters:
// - drawing : whether the drawing mode is on or off
fn update_status_bar(drawing: &bool) {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let (cols, rows) = termion::terminal_size().unwrap();
    // save cursor position
    let (cur_x, cur_y) = DetectCursorPos::cursor_pos(&mut stdout).unwrap();

    // the characters to print [the statusbar]
    let mut status_bar = String::new();
    status_bar.push_str(WHITE);
    status_bar.push_str(BG_PURP);

    // the text inside the status bar
    let mut text = String::from(if *drawing { " Drawing" } else { " Not drawing" });

    text.push_str(" - ");
    text.push_str("[d]raw");
    text.push_str(" - ");
    text.push_str("[q]uit");
    text.push_str(" - ");
    text.push_str("[c]lear");
    text.push_str(" - ");
    text.push_str("[n]ext_color");
    text.push_str(" - ");
    text.push_str("[r]eload");

    status_bar.push_str(text.as_str());

    // fill the remaining space with spaces
    status_bar.push_str(
        " ".repeat(
            if cols < text.len() as u16 {
                cols as usize * 2
            } else {
                cols as usize
            } - text.len(),
        )
        .as_str(),
    );

    status_bar.push_str(RESET);

    // goto and clear the last line of the terminal
    write!(
        stdout,
        "{}{}",
        Goto(
            1,
            if cols < text.len() as u16 {
                rows - 1
            } else {
                rows
            }
        ),
        clear::CurrentLine
    )
    .unwrap();
    // write the statusbar to stdout
    write!(stdout, "{}", status_bar).unwrap();
    // restore cursor position
    write!(stdout, "{}", Goto(cur_x, cur_y)).unwrap();
    stdout.flush().unwrap();
}

// this function draws every "pixel" in the matrix vector at his position and with its color
fn update_matrix(matrix: &mut Vec<(u16, u16, &str)>) {
    let mut stdout = stdout().into_raw_mode().unwrap();

    // save cursor position
    let (cur_x, cur_y) = DetectCursorPos::cursor_pos(&mut stdout).unwrap();

    for pixel in matrix.iter_mut() {
        // print only if not the position of the cursor
        if pixel.0 != cur_x || pixel.1 != cur_y {
            write!(stdout, "{}", Goto(pixel.0, pixel.1)).unwrap();
            write!(stdout, "{} {}", pixel.2, RESET).unwrap();
        }
    }

    // restore cursor position
    write!(stdout, "{}", Goto(cur_x, cur_y)).unwrap();
}

// put the cursor at the center of the terminal
fn position_cursor(cursor: &String) {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let (cols, rows) = termion::terminal_size().unwrap();
    write!(stdout, "{}{}{}", Goto(cols / 2, rows / 2), *cursor, Left(1)).unwrap();
    stdout.flush().unwrap();
}

fn main() {
    // the index (in COLORS vector) of the colors to use (foreground and background)
    let mut color_index = 0;

    // the foreground colors to use
    let mut color: &str = COLORS[color_index][0];

    // the cursor string (with foreground color)
    let mut cursor = format!("{}{}{}", COLORS[color_index][1], CURSOR_ICON, RESET);
    // whether the drawing mode is on or off
    let mut drawing = false;

    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());
    let stdin = stdin();

    // clear the terminal and hide the terminal cursor
    write!(stdout, "{}{}", clear::All, Hide).unwrap();

    // prints the status bar
    update_status_bar(&drawing);

    let (mut cols, mut rows);
    // the matrix of every "pixels" to print on the terminal
    let mut matrix: Vec<(u16, u16, &str)> = vec![];

    // clear the terminal and put the cursor to the center
    position_cursor(&cursor);
    stdout.flush().unwrap();

    // listen for keypress
    for c in stdin.events() {
        let mut temp_disable_drawing = false;
        (cols, rows) = termion::terminal_size().unwrap();
        match c.unwrap() {
            Event::Key(Key::Char('q')) => break,
            Event::Key(Key::Ctrl('c')) => break,
            Event::Key(Key::Esc) => break,
            // toggle drawing mode
            Event::Key(Key::Char('d')) => drawing = !drawing,
            Event::Key(Key::Char('r')) => write!(stdout, "{}{}", clear::All, Hide).unwrap(),
            Event::Key(Key::Char('c')) => {
                // remove every pixel from the matrix
                matrix.clear();
                // clear the terminal
                write!(stdout, "{}", clear::All).unwrap();
                // reposition the cursor at the center of the screen
                position_cursor(&cursor);
                // disable the drawing mode
                drawing = false;
            }
            Event::Key(Key::Char('n')) => {
                // change the color to use
                color_index = (color_index + 1) % COLORS.len();
                color = COLORS[color_index][0];
                cursor = format!("{}{}{}", COLORS[color_index][1], CURSOR_ICON, RESET);
                write!(stdout, "{}{}{}", cursor, RESET, Left(1)).unwrap();
            }
            Event::Key(Key::Left) => write!(stdout, " {}{}{}", Left(2), cursor, Left(1)).unwrap(),
            Event::Key(Key::Right) => {
                if DetectCursorPos::cursor_pos(&mut stdout).unwrap().0 < cols - 1 {
                    write!(stdout, " {}{}", cursor, Left(1)).unwrap()
                }
            }
            Event::Key(Key::Down) => {
                // dont overwrite the status bar
                if DetectCursorPos::cursor_pos(&mut stdout).unwrap().1 < rows - 1 {
                    write!(stdout, " {}{}{}{}", Left(1), Down(1), cursor, Left(1)).unwrap();
                }
            }
            Event::Key(Key::Up) => {
                write!(stdout, " {}{}{}{}", Left(1), Up(1), cursor, Left(1)).unwrap()
            }
            Event::Mouse(me) => match me {
                MouseEvent::Hold(x, y) => {
                    if y != rows && x != cols {
                        write!(stdout, " {}{}{}{}", Goto(x, y), cursor, Left(1), RESET).unwrap();
                        temp_disable_drawing = true;
                    }
                }
                MouseEvent::Press(b, x, y) => {
                    // check if the position is on the status bar
                    if y != rows && x != cols && b == MouseButton::Left {
                        write!(stdout, " {}{}{}{}", Goto(x, y), cursor, Left(1), RESET).unwrap();
                        temp_disable_drawing = true;
                    }
                }
                MouseEvent::Release(y, x) => temp_disable_drawing = true,
            },
            _ => {}
        }
        // update the status bar
        update_status_bar(&drawing);

        if drawing && !temp_disable_drawing {
            // if drawing mode is on, add the current position and color to the matrix
            let (x, y) = DetectCursorPos::cursor_pos(&mut stdout).unwrap();
            matrix.push((x, y, color));
        }
        // update the matrix
        update_matrix(&mut matrix);
        stdout.flush().unwrap();
    }

    // restore terminal
    write!(stdout, "{}{}", clear::All, Show).unwrap();
    stdout.flush().unwrap();
    std::process::exit(0);
}
