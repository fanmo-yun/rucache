use std::{io::{self, BufRead, BufReader, BufWriter, Write}, net::{TcpListener, TcpStream}, sync::{Arc, Mutex}, thread};

use database::{RuCache, Value};

fn parse_value(input: &str) -> Value {
    if let Ok(int_val) = input.parse::<i64>() {
        return Value::Int(int_val);
    }

    if let Ok(double_val) = input.parse::<f64>() {
        return Value::Double(double_val);
    }

    return Value::Str(input.to_string());
}

fn handle_client(stream: TcpStream, nosql: Arc<Mutex<RuCache>>) {
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
            "put" => {
                if parts.len() >= 3 {
                    match nosql.lock().unwrap().put(parts[1].to_string(), parse_value(parts[2])) {
                        Ok(true) => writer.write_all(b"update data successful\n").unwrap(),
                        Ok(false) => writer.write_all(b"put data successful\n").unwrap(),
                        Err(_) => unreachable!(),
                    }
                    writer.flush().unwrap();
                }
            },
            "delete" => {
                if parts.len() >= 2 {
                    match nosql.lock().unwrap().delete(parts[1]) {
                        Ok(_) => writer.write_all(b"delete successful\n").unwrap(),
                        Err(_) => writer.write_all(b"delete unsuccessful\n").unwrap(),
                    };
                }
                writer.flush().unwrap();
            },
            "get" => {
                if parts.len() >= 2 {
                    match nosql.lock().unwrap().get(parts[1]) {
                        Some(v) => writer.write_all(format!("\"{}\"\n", v).as_bytes()).unwrap(),
                        None => writer.write_all(format!("cannot get {}\n", parts[1]).as_bytes()).unwrap(),
                    };
                }
                writer.flush().unwrap();
            },
            _ => writer.write_all("Unknown command\n".as_bytes()).unwrap(),
        }
        writer.flush().unwrap();
        read_buffer.clear();
    }
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8874")?;

    let db = Arc::new(Mutex::new(RuCache::new()));

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        
        let nosql = db.clone();
        thread::spawn(|| handle_client(stream, nosql));
    }

    return Ok(());
}
