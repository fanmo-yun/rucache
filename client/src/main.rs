use std::{io::{self, BufRead, BufReader, BufWriter, Write}, net::TcpStream};

fn read_stdin() -> Vec<u8> {
    let mut buf = String::new();
    let _ = io::stdin().read_line(&mut buf).unwrap();
    let buf_trim = buf.trim_end();
    buf_trim.as_bytes().to_vec()
}

fn main() {
    let client = TcpStream::connect("127.0.0.1:8874").unwrap();

    let mut reader = BufReader::new(&client);
    let mut writer = BufWriter::new(&client);

    let mut read_buf: Vec<u8> = Vec::new();

    loop {
        let will_write_message = read_stdin();

        writer.write_all(&will_write_message).unwrap();

        let size = reader.read_until(b'\n', &mut read_buf).unwrap();
    }
}
