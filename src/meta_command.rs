use std::iter::Peekable;

pub enum MetaCommand {
    Exit,
    Unrecognized(String),
}

impl MetaCommand {
    pub fn parse(chars: &mut Peekable<impl Iterator<Item = char>>) -> MetaCommand {
        chars.next();
        let command: String = chars.collect();
        match command.as_ref() {
            "exit" => MetaCommand::Exit,
            _ => MetaCommand::Unrecognized(command),
        }
    }
}
