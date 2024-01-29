use crate::parser;
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
    command_result: Option<Result<String, String>>,
    /// true if the last event resulted in a newline
    was_newline: bool,
    /// true if the previous line has been entered without a ';' at the end of it,
    /// false otherwise
    tbc: bool,
    /// false if the repl should exit, true otherwise
    running: bool,
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
            tbc: false,
            running: true,
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
                        self.cursor_idx = 0;
                        self.history.push(self.current_line.clone());
                        self.hist_idx = self.history.len();
                        self.tbc = true;
                        // The current line ends a command
                        if self.current_line.ends_with(";") {
                            let full_command = self.history[self.first_command_line..].join("\n");
                            let ast = parser::Parser::new(full_command)
                                .parse()
                                .map(|x| format!("{:#?}", x))
                                .map_err(|err| format!("{err}"));
                            self.command_result = Some(ast);
                            self.first_command_line = self.hist_idx;
                            self.tbc = false;
                        }
                        self.current_line.clear()
                    }
                } else {
                    self.current_line.insert(self.cursor_idx, c);
                    self.cursor_idx += 1;
                }
            }
            Key::Backspace => {
                if self.cursor_idx != 0 {
                    self.cursor_idx -= 1;
                    self.current_line.remove(self.cursor_idx);
                }
            }
            Key::Left => {
                self.cursor_idx = self.cursor_idx.checked_sub(1).unwrap_or(self.cursor_idx)
            }
            Key::Right => {
                if self.cursor_idx < self.current_line.len() {
                    self.cursor_idx += 1;
                }
            }
            Key::Up => {
                if let Some(nhi) = self.hist_idx.checked_sub(1) {
                    self.hist_idx = nhi;
                    self.current_line = self.history[self.hist_idx].clone();
                }
            }
            Key::Down => {
                if self.hist_idx + 1 < self.history.len() {
                    self.hist_idx += 1;
                    self.current_line = self.history[self.hist_idx].clone();
                }
            }
            Key::Ctrl('c') | Key::Ctrl('d') => self.running = false,
            _ => (),
        }
    }
    fn show(&self, stdout: &mut Stdout) {
        if self.was_newline {
            newline(stdout);
        }
        if let Some(result) = &self.command_result {
            let result = result
                .as_ref()
                .map(|x| x.clone())
                .unwrap_or_else(|x| format!("Error: {x}"));
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
        let prompt = if self.tbc {
            &self.prompt2
        } else {
            &self.prompt1
        };
        write!(
            stdout,
            "{prompt}{}{}",
            self.current_line,
            termion::cursor::Goto((prompt.len() + self.cursor_idx + 1) as u16, cursor_y)
        )
        .unwrap();
        stdout.flush().unwrap();
    }
    pub fn run(mut self) {
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();
        self.show(&mut stdout);
        for c in stdin.keys() {
            self.update(c.unwrap());
            if !self.running {
                break;
            }
            self.show(&mut stdout);
        }
    }
}
