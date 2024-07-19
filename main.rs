use std::io::{self, Write};
use std::net::{TcpStream};

fn input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim_end().to_string())
}

fn client() -> io::Result<()> {
    let host = input("Enter host: ").unwrap();
    let port = input("Enter port: ").unwrap();

    let addr = format!("{}:{}", host, port);

    let mut client = TcpStream::connect(addr).expect("Failed connect to: {addr}");

    client.write("Hello, TCP\n".as_bytes()).expect("Failed to send data");

    Ok(())
}

fn main() {
    let option = input("Enter option: ").unwrap();
    match option.as_str() {
        "connect" => {
            client();
        },
        _ => {
            println!("Not valid option");
        }
    }
}
