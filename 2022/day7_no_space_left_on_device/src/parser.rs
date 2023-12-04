use std::error::Error;
use std::fs::File as FSFile;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    message: String,
}

impl ParseError {
    fn new(message: String) -> Self {
        Self { message }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct File {
    pub name: String,
    pub size: u64,
}

impl FromStr for File {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        // "14848514 b.txt"
        let (size, name) = line.split_once(" ").unwrap();
        match size.parse() {
            Ok(s) => Ok(Self {
                size: s,
                name: String::from(name),
            }),
            Err(_) => Err(ParseError::new(format!(
                "size is not a number {}; line: \"{}\"",
                size, line
            ))),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Dir {
    pub name: String,
}

impl FromStr for Dir {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, name) = s.split_once(" ").unwrap();
        if !dir.eq("dir") {
            return Err(ParseError::new("not a directory".to_string()));
        }
        Ok(Self {
            name: name.to_string(),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CommandOutputLine {
    File(File),
    Dir(Dir),
}

impl FromStr for CommandOutputLine {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        if line.starts_with("dir") {
            return Ok(CommandOutputLine::Dir(Dir::from_str(line)?));
        } else {
            return Ok(CommandOutputLine::File(File::from_str(line)?));
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    LS(),
    CD(String),
}

impl FromStr for Command {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("$ ls") {
            return Ok(Command::LS());
        }
        if s.starts_with("$ cd ") {
            return Ok(Command::CD(s.split_once(" cd ").unwrap().1.to_string()));
        }
        Err(ParseError::new(format!("command not supported: {}", s)))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CommandExecution {
    pub command: Command,
    pub output: Vec<CommandOutputLine>,
}

pub fn load_input_command_executions(
    input_file: &str,
) -> Result<Vec<CommandExecution>, Box<dyn Error>> {
    let mut command_executions = Vec::new();

    let mut command: Command = Command::LS();
    let mut command_output: Vec<CommandOutputLine> = Vec::new();
    let mut command_read = false;

    let file = FSFile::open(input_file).expect("input file should exist");
    let file_buffer = BufReader::new(file);
    for line_wrapped in file_buffer.lines() {
        let line = line_wrapped.unwrap();
        if line.starts_with("$ ") {
            if command_read {
                command_executions.push(CommandExecution {
                    command: command,
                    output: command_output,
                });
            }
            command = Command::from_str(&line).unwrap();
            command_output = Vec::new();
            command_read = true;
        } else {
            match CommandOutputLine::from_str(&line) {
                Ok(output_line) => command_output.push(output_line),
                Err(e) => panic!("error: {}", e.message),
            }
        }
    }
    command_executions.push(CommandExecution {
        command: command,
        output: command_output,
    });

    Ok(command_executions)
}
