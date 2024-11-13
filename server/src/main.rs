use std::{io::{self, BufRead, BufReader, BufWriter, Write}, net::{TcpListener, TcpStream}, thread};

fn handle_client(stream: TcpStream) {
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);

    let mut read_buffer: Vec<u8> = Vec::new();
    loop {
        let size = match reader.read_until(b'\n', &mut read_buffer) {
            Ok(0) => {
                break;
            }
            Ok(n) => n,
            Err(_) => {
                eprintln!("Error reading from stream");
                break;
            }
        };

        let read = String::from_utf8_lossy(&read_buffer[..size]);
        let parts: Vec<&str> = read.split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "exit" => break,
            _ => writer.write_all("Unknown command\n".as_bytes()).unwrap(),
        }
        writer.flush().unwrap();
        read_buffer.clear();
    }
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8874")?;

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| handle_client(stream));
    }

    return Ok(());
}
