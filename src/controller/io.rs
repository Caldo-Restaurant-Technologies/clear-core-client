use std::{net::TcpStream, usize};

pub struct DigitalInput {
    stream: TcpStream,
    id: usize,
}

impl DigitalInput {
    pub fn new(stream: TcpStream, id: usize) -> Self {
        Self { stream, id }
    }
}

pub struct AnalogInput {
    stream: TcpStream,
    id: usize,
}

impl AnalogInput {
    pub fn new(stream: TcpStream, id: usize) -> Self {
        Self { stream, id }
    }
}

pub struct DigitalOutput {
    stream: TcpStream,
    id: usize,
}

impl DigitalOutput {
    pub fn new(stream: TcpStream, id: usize) -> Self {
        Self { stream, id }
    }
}

pub struct AnalogOutput {
    stream: TcpStream,
    id: usize,
}

impl AnalogOutput {
    pub fn new(stream: TcpStream, id: usize) -> Self {
        Self { stream, id }
    }
}

pub struct HBridge {
    stream: TcpStream,
    id: usize,
}

impl HBridge {
    pub fn new(stream: TcpStream, id: usize) -> Self {
        Self { stream, id }
    }
}

pub enum Io {
    DigitalInput(DigitalInput),
    DigitalOutput(DigitalOutput),
    AnalogOutput(AnalogOutput),
    HBridge(HBridge),
}
