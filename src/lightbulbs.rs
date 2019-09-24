use std::io::prelude::*;
use std::net::TcpStream;

#[derive(Debug)]
pub enum LightbulbError {
	IO(std::io::Error),
	FromUtf8(std::string::FromUtf8Error)
}

impl From<std::io::Error> for LightbulbError {
	fn from(e: std::io::Error) -> LightbulbError { LightbulbError::IO(e) }
}
impl From<std::string::FromUtf8Error> for LightbulbError {
	fn from(e: std::string::FromUtf8Error) -> LightbulbError { LightbulbError::FromUtf8(e) }
}

pub type Result<T> = std::result::Result<T, LightbulbError>;

fn tplink_encrypt(data: &mut [u8]) {
    let mut key = 0xABu8;

    for elem in data.iter_mut() {
        *elem ^= key;
        key = *elem;
    }
}

fn tplink_decrypt(data: &mut [u8]) {
    let mut key = 0xABu8;

    for elem in data.iter_mut() {
        let val = *elem;
        *elem ^= key;
        key = val;
    }
}

fn send_packet(stream: &mut TcpStream, packet: &[u8]) -> Result<()> {
	let mut enc_packet = packet.to_vec();
	tplink_encrypt(&mut enc_packet);

    stream.write_all(&(packet.len() as u32).to_be_bytes())?;
    stream.write_all(&enc_packet)?;
    stream.flush()?;

	Ok(())
}

fn read_packet(stream: &mut TcpStream) -> Result<Vec<u8>> {
    let mut length_buffer = [0u8; 4];
    stream.read_exact(&mut length_buffer)?;

    let output_length = u32::from_be_bytes(length_buffer);

    let mut output = vec![0u8; output_length as usize];
    stream.read_exact(&mut output)?;
    tplink_decrypt(&mut output);

	return Ok(output);
}

pub fn send_json(stream: &mut TcpStream, json: &str) -> Result<String> {
	send_packet(stream, json.as_bytes())?;
	Ok(String::from_utf8(read_packet(stream)?)?)
}

pub fn get_status(stream: &mut TcpStream) -> Result<String> {
    let json = "{\"system\":{\"get_sysinfo\":\"\"}}";
	return send_json(stream, json);
}

pub fn set_on_off(stream: &mut TcpStream, state: bool, transition_period: i32) -> Result<()> {
	let json = format!(
		"{{\"smartlife.iot.smartbulb.lightingservice\":{{\"transition_light_state\":{{\"ignore_default\":1,\"transition_period\":{},\"on_off\":{}}}}}}}",
		transition_period,
		if state { 1 } else { 0 }
	);

	send_json(stream, &json)?;
	Ok(())
}