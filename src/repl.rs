use std::io::{stdin, stdout, Stdout, Write};
use termion::cursor::DetectCursorPos;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
/// The structure representing the REPL's state
pub struct Repl {
    /// The string printed before, the first line of the command.
    /// **This string must not contain newlines.**
    prompt1: String,
    /// The string printed before, the following lines of the command.
    /// **This string must not contain newlines.**
    prompt2: String,
    /// The command currently being typed
    current_line: String,
    /// The list of all previously typed lines.
    /// **This vector must not contain empty strings.**
    history: Vec<String>,
    /// The index in history of the first line of the command, if the current
    /// command has no lines in history, this should be equal to *history.len()*.
    first_command_line: usize,
    /// The index of the current line as found in *history*, if the line is not
    /// yet in history, this should be equal to *history.len()*.
    hist_idx: usize,
    /// The index of the cursor in *current_line*
    cursor_idx: usize,
    /// The value will be *Some(x)*, where x is the result of the previous command,
    /// **if the last line was the end of a command**
    command_result: Option<String>,
    /// true if the last event resulted in a newline
    was_newline: bool,
}

fn newline(stdout: &mut Stdout) {
    let height = termion::terminal_size().unwrap().1;
    let cursor_y = stdout.cursor_pos().unwrap().1;
    if cursor_y >= height {
        write!(stdout, "{}", termion::scroll::Up(1)).unwrap();
    }
    let cursor_y = stdout.cursor_pos().unwrap().1;
    write!(stdout, "{}", termion::cursor::Goto(1, cursor_y + 1)).unwrap();
}

impl Repl {
    pub fn new(prompt1: String, prompt2: String, history: Vec<String>) -> Self {
        Repl {
            current_line: String::new(),
            first_command_line: history.len(),
            hist_idx: history.len(),
            cursor_idx: 0,
            command_result: None,
            was_newline: false,
            prompt1,
            prompt2,
            history,
        }
    }
    fn update(&mut self, c: Key) {
        self.command_result = None;
        self.was_newline = false;
        match c {
            Key::Char(c) => {
                if c == '\n' {
                    if !self.current_line.is_empty() {
                        self.was_newline = true;
                        self.hist_idx += 1;
                        self.cursor_idx = 0;
                        self.history.push(self.current_line.clone());
                        // The current line ends a command
                        if self.current_line.ends_with(";") {
                            let full_command =
                                self.history[self.first_command_line..self.hist_idx].join("\n");
                            self.command_result = Some(format!("TODO: execute {full_command}"));
                            self.first_command_line = self.hist_idx;
                        }
                        self.current_line.clear()
                    }
                } else {
                    self.current_line.insert(self.cursor_idx, c);
                    self.cursor_idx += 1;
                }
            }
            _ => todo!(),
        }
    }
    fn show(&self, stdout: &mut Stdout) {
        if self.was_newline {
            newline(stdout);
        }
        if let Some(result) = &self.command_result {
            for c in result.chars() {
                if c == '\n' {
                    newline(stdout);
                } else {
                    write!(stdout, "{c}").unwrap();
                }
            }
            newline(stdout);
        }
        let cursor_y = stdout.cursor_pos().unwrap().1;
        write!(
            stdout,
            "{}{}",
            termion::clear::CurrentLine,
            termion::cursor::Goto(1, cursor_y)
        )
        .unwrap();
        if self.hist_idx == self.first_command_line {
            write!(stdout, "{}", self.prompt1).unwrap();
        } else {
            write!(stdout, "{}", self.prompt2).unwrap();
        }
        write!(stdout, "{}", self.current_line).unwrap();
        stdout.flush().unwrap();
    }
    pub fn run(mut self) {
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();
        self.show(&mut stdout);
        for c in stdin.keys() {
            self.update(c.unwrap());
            self.show(&mut stdout);
        }
    }
}
