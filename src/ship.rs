use std::error;
use std::fmt;

#[derive(Debug)]
pub struct CommandTypeParseError(String);

impl fmt::Display for CommandTypeParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "command type parse error ({})", self.0)
    }
}

impl error::Error for CommandTypeParseError {}

#[derive(Debug)]
pub enum CommandType {
    Forward,
    Down,
    Up,
}

impl std::str::FromStr for CommandType {
    type Err = CommandTypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(CommandType::Forward),
            "down" => Ok(CommandType::Down),
            "up" => Ok(CommandType::Up),
            _ => Err(CommandTypeParseError(format!(
                "`{}` unidentifiable command type.",
                s
            ))),
        }
    }
}

#[derive(Debug)]
pub struct CommandParseError(String);

impl fmt::Display for CommandParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "command parse error ({})", self.0)
    }
}

impl std::error::Error for CommandParseError {}

pub struct Command {
    command: CommandType,
    distance: u64,
}

impl std::str::FromStr for Command {
    type Err = CommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split(' ').take(2);
        let command = tokens
            .next()
            .ok_or(CommandParseError(format!("missing first command token")))?
            .parse::<CommandType>()
            .map_err(|parse_cmd_err| CommandParseError(parse_cmd_err.to_string()))?;
        let distance = tokens
            .next()
            .ok_or(CommandParseError(format!("missing second command token")))?
            .parse::<u64>()
            .map_err(|parse_int_err| CommandParseError(parse_int_err.to_string()))?;
        Ok(Command { command, distance })
    }
}

pub struct Pilot {
    forward: u64,
    depth: u64,
}

impl Pilot {
    pub fn new() -> Self {
        Pilot {
            forward: 0,
            depth: 0,
        }
    }

    pub fn process(&mut self, cmd: &Command) {
        match cmd.command {
            CommandType::Forward => {
                self.forward += cmd.distance;
            }
            CommandType::Down => {
                self.depth += cmd.distance;
            }
            CommandType::Up => {
                self.depth = self.depth.saturating_sub(cmd.distance);
            }
        }
    }

    pub fn get_forward(&self) -> u64 {
        self.forward
    }

    pub fn get_depth(&self) -> u64 {
        self.depth
    }
}
