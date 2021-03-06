use std::io::{Write,BufReader,BufWriter,BufRead};
use std::net::{TcpStream,ToSocketAddrs};

const LOCAL_HOST: &str = "127.0.0.1";
const PORT_NUMBE: u32  = 8000;


fn read_something (reader: &mut BufReader<&TcpStream>) {
  let mut msg = String::new();
  reader.read_line(&mut msg).expect("RECEIVE FAILURE!!!");
 
  println!("{}", msg);
}

fn write_something (writer: &mut BufWriter<&TcpStream>, comment: &str) {
  let msg = format!("MESSAGE: {}\n", comment);
 
  writer.write(msg.as_bytes()).expect("SEND FAILURE!!!");
  writer.flush().unwrap();
}

fn main() {

    let host_and_port = format!("{}:{}", LOCAL_HOST, PORT_NUMBE);
    let mut addrs = host_and_port.to_socket_addrs().unwrap();

    if let Some(addr) = addrs.find(|x| (*x).is_ipv4()) {
    match TcpStream::connect(addr) {
        Err(_) => {
        println!("Connection NG.");
        }
        Ok(stream) => {
        println!("Connection Ok.");
            let mut reader = BufReader::new(&stream);
            let mut writer = BufWriter::new(&stream);

            read_something(&mut reader);
            write_something(&mut writer, "hoge");
      }
    }
  } else {
    eprintln!("Invalid Host:Port Number");
  }
}
