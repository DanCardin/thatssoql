use std::iter::Peekable;

pub enum Statement {
    Insert(usize, String, String),
    Select,
    Unrecognized(String),
}

impl Statement {
    pub fn prepare(chars: &mut Peekable<impl Iterator<Item = char>>) -> Statement {
        let command: String = chars.collect();
        let command_parts: Vec<&str> = command.split(' ').collect();
        match &command_parts[..] {
            ["insert", id, username, email] => {
                let id = id.parse::<usize>().unwrap();
                let username = username.to_string();
                let email = email.to_string();
                Statement::Insert(id, username, email)
            }
            ["select"] => Statement::Select,
            _ => Statement::Unrecognized(format!("{:?}", command)),
        }
    }
}
