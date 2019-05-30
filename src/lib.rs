#![feature(slice_patterns)]
#![feature(async_await)]

use futures::StreamExt;
use futures::executor::{self, ThreadPool};
use futures::io::{AsyncReadExt, AsyncWriteExt};
use futures::task::{SpawnExt};

use romio::{TcpListener, TcpStream};

mod cursor;
mod meta_command;
mod pager;
mod row;
mod statement;
mod table;

use crate::cursor::Cursor;
use crate::meta_command::MetaCommand;
use crate::pager::Pager;
use crate::statement::Statement;
use crate::table::Table;
use std::io;

pub fn start() -> io::Result<()> {
    executor::block_on(async {
        let mut threadpool = ThreadPool::new()?;

        let mut listener = TcpListener::bind(&"127.0.0.1:7878".parse().unwrap())?;
        let mut incoming = listener.incoming();

        println!("Listening on 127.0.0.1:7878");

        while let Some(stream) = incoming.next().await {
            let stream = stream?;
            let addr = stream.peer_addr()?;

            threadpool.spawn(async move {
                println!("starting");
                run_statement(stream).await.unwrap();
                println!("Closing stream from: {}", addr);
            }).unwrap();
        }

        Ok(())
    })
}

async fn run_statement(mut stream: TcpStream) -> io::Result<()> {
    println!("asdflkajsdflkj");
    let (mut reader, mut writer) = stream.split();

    let mut pager = Pager::open().unwrap();
    let mut table = Table::new();
    println!("2");

    loop {
        println!("3");
        let mut input_buffer = Vec::new();
        reader.read(&mut input_buffer).await?;
        println!("buffer {:?}", input_buffer);

        let mut cursor = Cursor::table_start(&mut table, &mut pager);

        let raw_input = String::from_utf8(input_buffer);

        let input = match raw_input {
            Ok(content) => content,
            Err(error) => {
                println!("error: {}", error);
                return Ok(());
            }
        };
        println!("input {}", input);

        let mut chars = input.chars().peekable();

        let statement = match chars.peek() {
            Some('.') => {
                let command = MetaCommand::parse(&mut chars);
                match command {
                    MetaCommand::Exit => return Ok(()),
                    MetaCommand::Unrecognized(command) => {
                        println!("Unrecognized meta command {}", command);
                        return Ok(());
                    }
                }
            }
            Some(_) => {
                let statement = Statement::prepare(&mut chars);
                match statement {
                    Statement::Unrecognized(statement) => {
                        println!("Unrecognized statement {}", statement);
                        return Ok(());
                    }
                    statement => statement,
                }
            }
            None => return Ok(()),
        };
        cursor.execute(statement);

        // pager.write()?;

        let msg = format!("{:?}", pager.rows);
        writer.write_all(msg.as_bytes()).await?;
        break;
    }
    Ok(())
}
