use std::{
    io::{BufReader, Read, Result, Write},
    net::TcpStream,
};

pub mod controller;

const BUFFER_LENGTH: usize = 101;

pub trait SendRecv {
    fn send_recv(mut stream: TcpStream, buffer: &[u8]) -> Result<Vec<u8>> {
        let r_stream = stream.try_clone().unwrap();
        let mut reader = BufReader::new(r_stream);
        let mut response = Vec::with_capacity(BUFFER_LENGTH);
        stream.write_all(buffer)?;
        reader.read_to_end(&mut response)?;
        Ok(response)
    }
}

pub fn num_to_bytes<T: ToString>(number: T) -> Vec<u8> {
    number.to_string().chars().map(|c| c as u8).collect()
}

pub fn int_to_byte(number: u8) -> u8 {
    number + 48
}

pub fn ascii_to_int(bytes: &[u8]) -> isize {
    let sign = if bytes[0] == 45 { -1 } else { 1 };
    let int = bytes
        .iter()
        .filter(|&&x| (48..=57).contains(&x))
        .fold(0, |mut acc, x| {
            let num = x - 48;
            acc *= 10;
            acc += num as isize;
            acc
        });
    int * sign
}

#[test]
fn test_int_to_bytes() {
    let bytes = num_to_bytes(2300);
    assert_eq!(bytes, [50, 51, 48, 48]);
    let bytes = num_to_bytes(-3400);
    assert_eq!(bytes, [45, 51, 52, 48, 48]);
    let bytes = num_to_bytes(2300.0);
    assert_eq!(bytes, [50, 51, 48, 48]);
    let bytes = num_to_bytes(-3400.0);
    assert_eq!(bytes, [45, 51, 52, 48, 48]);
    let bytes = num_to_bytes((-0.5 * 800.0) as isize);
    println!("{:?}", bytes);
}

#[test]
fn test_bytes_to_int() {
    let int = ascii_to_int([45, 51, 52, 48, 48, 13].as_slice());
    assert_eq!(-3400, int);
    let int = ascii_to_int([50, 51, 48, 48].as_slice());
    assert_eq!(2300, int);
}
