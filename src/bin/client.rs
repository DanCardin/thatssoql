#![feature(async_await)]

use std::io;

use futures::executor;
use futures::io::{AsyncReadExt, AllowStdIo};

use romio::TcpStream;
use futures::prelude::*;

fn main() -> io::Result<()> {
    executor::block_on(async {
        let mut stream = TcpStream::connect(&"127.0.0.1:7878".parse().unwrap()).await?;
        let mut stdout = AllowStdIo::new(io::stdout());

        let (mut reader, mut writer) = stream.split();

        // loop {
            print!("> ");
            stdout.flush().await?;

            let mut input_buffer = String::new();
            io::stdin().read_line(&mut input_buffer)?;
            writer.write_all(input_buffer.as_bytes());
            println!("Done");
            // stdout.flush().await?;
            //
            // reader.copy_into(&mut stdout).await?;
            // stdout.flush().await?;
        // }

        Ok(())
    })
}


