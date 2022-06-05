use std::fs;
use std::env;
use std::io::{stdin, stdout};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

//use termsize;

fn main() {
    // Getting command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        // Didn't provide arguments; quitting
        eprintln!("ERROR: Please provide a filename to read.");
        eprintln!("Quitting...");
        return;
    }
    // Filename to read
    let query = &args[1];

    // Checking if the file exists
    if fs::metadata(query).is_err() {
        eprintln!("ERROR: Please provide a valid path to a file.");
        eprintln!("Quitting...");
        return;
    }
    // Getting string from file
    let file_string = fs::read_to_string(query).expect("Failed to read data.");
    // Getting a Vector of tuples of where the newlines are, to see how long the file is
    let file_length_vec: Vec<_> = file_string.match_indices('\n').collect();
    let file_length = file_length_vec.len() + 1;

    // Getting terminal size
    let rows: u16 = termsize::get().map(|size| {size.rows}).expect("Failed to parse terminal size.") - 2;
    let rows = rows as usize;

    // If the text is more than the screen
    if rows < file_length {

        let (line, _newline) = file_length_vec[rows];
        println!("{}", &file_string[..line]);
        let mut current_row = rows;
    
        let stdin = stdin();
        let stdout = stdout().into_raw_mode().unwrap();

        // Detecting keyboard events
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Down => {
                    // Arrow down
                    if file_length-2 == current_row {
                        break;
                    }
                    stdout.suspend_raw_mode().unwrap();
                    println!("{}", &file_string[..file_length_vec[current_row+1].0]);
                    current_row += 1;
                    stdout.activate_raw_mode().unwrap();
                },
                Key::Up => {
                    // Arrow up
                    if current_row < rows+1 {
                        break;
                    }
                    stdout.suspend_raw_mode().unwrap();
                    println!("{}", &file_string[..file_length_vec[current_row-1].0]);
                    current_row -= 1;
                    stdout.activate_raw_mode().unwrap();
                },
                Key::Home => {
                    stdout.suspend_raw_mode().unwrap();
                    println!("{}", &file_string[..file_length_vec[rows].0]);
                    current_row = rows;
                    stdout.activate_raw_mode().unwrap();
                },
                Key::End => {
                    stdout.suspend_raw_mode().unwrap();
                    println!("{}", &file_string[..file_length_vec[file_length-2].0]);
                    current_row = file_length-2;
                    stdout.activate_raw_mode().unwrap();
                },
                // Quitting
                Key::Esc => break,
                Key::Char('q') => break,
                Key::Ctrl('c') => break,
                _ => (),
            }
        }
    // Just print the file        
    } else {
        println!("{}", file_string);
    }
}