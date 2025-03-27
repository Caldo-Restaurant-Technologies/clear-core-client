use super::clear_core::{Controller, Error, check_reply};
use crate::{ascii_to_int, int_to_byte, num_to_bytes};
const STX: u8 = 2;
const CR: u8 = 13;
const MOTOR_CHAR: u8 = b'M';

pub enum Status {
    Disabled,
    Enabling,
    Faulted,
    Ready,
    Moving,
}

pub struct Motor {
    id: u8,
    scale: usize,
}

impl Motor {
    pub fn new(id: u8, scale: usize) -> Self {
        Self { id, scale }
    }

    pub fn enable(&self, controller: &mut Controller) -> Result<(), Error> {
        let cmd = [STX, MOTOR_CHAR, int_to_byte(self.id), b'E', b'N', CR];
        let resp = controller.send_recv(cmd.as_slice())?;
        check_reply(resp.as_slice())
    }

    pub fn disable(&self, controller: &mut Controller) -> Result<(), Error> {
        let cmd = [STX, MOTOR_CHAR, int_to_byte(self.id), b'D', b'E', CR];
        let resp = controller.send_recv(cmd.as_slice())?;
        check_reply(resp.as_slice())
    }

    pub fn absolute_move(&self, controller: &mut Controller, position: f64) -> Result<(), Error> {
        let position = num_to_bytes((position * (self.scale as f64)).trunc() as isize);
        let mut msg: Vec<u8> = Vec::with_capacity(100);
        msg.push(STX);
        msg.push(MOTOR_CHAR);
        msg.push(int_to_byte(self.id));
        msg.extend_from_slice(b"AM");
        msg.extend_from_slice(position.as_slice());
        msg.push(CR);
        let resp = controller.send_recv(msg.as_slice())?;
        check_reply(resp.as_slice())
    }

    pub fn relative_move(
        &self,
        controller: &mut Controller,
        displacement: f64,
    ) -> Result<(), Error> {
        let position = num_to_bytes((displacement * (self.scale as f64)).trunc() as isize);
        let mut msg: Vec<u8> = Vec::with_capacity(100);
        msg.push(STX);
        msg.push(MOTOR_CHAR);
        msg.push(int_to_byte(self.id));
        msg.extend_from_slice(b"RM");
        msg.extend_from_slice(position.as_slice());
        msg.push(CR);
        let resp = controller.send_recv(msg.as_slice())?;
        check_reply(resp.as_slice())
    }

    pub fn abrupt_stop(&self, controller: &mut Controller) -> Result<(), Error> {
        let stop_cmd = [STX, b'M', int_to_byte(self.id), b'A', b'S', CR];
        let resp = controller.send_recv(stop_cmd.as_slice())?;
        check_reply(resp.as_slice())
    }

    pub fn stop(&self, controller: &mut Controller) -> Result<(), Error> {
        let stop_cmd = [STX, b'M', int_to_byte(self.id), b'S', b'T', CR];
        let resp = controller.send_recv(stop_cmd.as_slice())?;
        check_reply(resp.as_slice())
    }

    pub fn set_position(&self, controller: &mut Controller, position: f64) -> Result<(), Error> {
        let position = num_to_bytes((position * (self.scale as f64)).trunc() as isize);
        let mut msg: Vec<u8> = Vec::with_capacity(100);
        msg.push(STX);
        msg.push(MOTOR_CHAR);
        msg.push(int_to_byte(self.id));
        msg.extend_from_slice(b"SP");
        msg.extend_from_slice(position.as_slice());
        msg.push(CR);
        let resp = controller.send_recv(msg.as_slice())?;
        check_reply(resp.as_slice())
    }

    pub fn set_velocity(&self, controller: &mut Controller, velocity: f64) -> Result<(), Error> {
        let velocity = if velocity < 0. { 0.0 } else { velocity };
        let velocity = num_to_bytes((velocity * (self.scale as f64)).trunc() as isize);
        let mut msg: Vec<u8> = Vec::with_capacity(100);
        msg.push(STX);
        msg.push(MOTOR_CHAR);
        msg.push(int_to_byte(self.id));
        msg.extend_from_slice(b"SV");
        msg.extend_from_slice(velocity.as_slice());
        msg.push(CR);
        let resp = controller.send_recv(msg.as_slice())?;
        check_reply(resp.as_slice())
    }

    pub fn set_acceleration(
        &self,
        controller: &mut Controller,
        acceleration: f64,
    ) -> Result<(), Error> {
        let acceleration = if acceleration < 0. { 0.0 } else { acceleration };
        let acceleration = num_to_bytes((acceleration * (self.scale as f64)).trunc() as isize);
        let mut msg: Vec<u8> = Vec::with_capacity(100);
        msg.push(STX);
        msg.push(MOTOR_CHAR);
        msg.push(int_to_byte(self.id));
        msg.extend_from_slice(b"SA");
        msg.extend_from_slice(acceleration.as_slice());
        msg.push(CR);
        let resp = controller.send_recv(msg.as_slice())?;
        check_reply(resp.as_slice())
    }

    pub fn set_deceleration(
        &self,
        controller: &mut Controller,
        deceleration: f64,
    ) -> Result<(), Error> {
        let deceleration = if deceleration < 0. { 0.0 } else { deceleration };
        let deceleration = num_to_bytes((deceleration * (self.scale as f64)).trunc() as isize);
        let mut msg: Vec<u8> = Vec::with_capacity(100);
        msg.push(STX);
        msg.push(MOTOR_CHAR);
        msg.push(int_to_byte(self.id));
        msg.extend_from_slice(b"SD");
        msg.extend_from_slice(deceleration.as_slice());
        msg.push(CR);
        let resp = controller.send_recv(msg.as_slice())?;
        check_reply(resp.as_slice())
    }

    pub async fn get_status(&self, controller: &mut Controller) -> Result<Status, Error> {
        let status_cmd = [STX, b'M', int_to_byte(self.id), b'G', b'S', CR];
        let resp = controller.send_recv(status_cmd.as_slice())?;
        check_reply(resp.as_slice())?;
        match resp[3] {
            48 => Ok(Status::Disabled),
            49 => Ok(Status::Enabling),
            50 => Ok(Status::Faulted),
            51 => Ok(Status::Ready),
            52 => Ok(Status::Moving),
            _ => Err(Error {
                message: "unknown status".to_string(),
            }),
        }
    }

    pub fn get_position(&self, controller: &mut Controller) -> Result<f64, Error> {
        let get_pos_cmd = [STX, b'M', int_to_byte(self.id), b'G', b'P', CR];
        let resp = controller.send_recv(get_pos_cmd.as_slice())?;
        check_reply(&resp)?;
        Ok((ascii_to_int(resp.as_slice()) as f64) / (self.scale as f64))
    }

    pub fn clear_alerts(&self, controller: &mut Controller) -> Result<(), Error> {
        let clear_cmd = [STX, b'M', int_to_byte(self.id), b'C', b'A', CR];
        let resp = controller.send_recv(clear_cmd.as_slice())?;
        check_reply(&resp)
    }
}
