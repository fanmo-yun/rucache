use std::{io::{self, BufRead, BufReader, BufWriter, Write}, net::TcpStream};

fn read_stdin() -> String {
    let mut buf = String::new();
    let _ = io::stdin().read_line(&mut buf).unwrap();
    let buf_trim = buf.trim_end();
    let final_buf = format!("{}\n", buf_trim);
    final_buf
}

fn main() {
    let client = TcpStream::connect("127.0.0.1:8874").unwrap();

    let mut reader = BufReader::new(&client);
    let mut writer = BufWriter::new(&client);

    let mut read_buf: Vec<u8> = Vec::new();

    loop {
        print!(":P ");
        io::stdout().flush().unwrap();
        let will_write_message = read_stdin();

        if &will_write_message == "exit\n" {
            writer.write_all(&will_write_message.as_bytes()).unwrap();
            writer.flush().unwrap();
            break;
        }

        writer.write_all(&will_write_message.as_bytes()).unwrap();
        writer.flush().unwrap();

        let size = reader.read_until(b'\n', &mut read_buf).unwrap();
        print!("{}", String::from_utf8_lossy(&read_buf[..size]));
        io::stdout().flush().unwrap();
        read_buf.clear();
    }
}
