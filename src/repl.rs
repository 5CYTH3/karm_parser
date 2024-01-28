/// The structure representing the REPL's state
struct Repl {
    /// The string printed before, the first line of the command.
    /// **This string must not contain newlines.**
    prompt1: String,
    /// The string printed before, the following lines of the command.
    /// **This string must not contain newlines.**
    prompt2: String,
    /// The list of all previously typed lines.
    /// **This vector must not contain empty strings.**
    history: Vec<String>,
    /// The index in history of the first line of the command, if the current
    /// command has no lines in history, this should be equal to *history.len()*.
    first_command_line: usize,
    /// The index of the current line as found in *history*, if the line is not
    /// yet in history, this should be equal to *history.len()*.
    hist_idx: usize,
}
