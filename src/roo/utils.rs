use std::{io::{BufReader, Read}, fs::File};


static DELIMITER: [u8; 2] = [0x0D, 0x0A];

pub fn read_string(reader: &mut BufReader<File>, len: usize) -> String {
    let mut buf = vec![0; len];
    reader.read_exact(&mut buf).unwrap();
    buf.retain(|&x| x != 0);
    return String::from_utf8_lossy(&buf).to_string();
}

pub fn read_bytes(reader: &mut BufReader<File>, len: usize) -> Vec<u8> {
    let mut buf = vec![0; len];
    reader.read_exact(&mut buf).unwrap();
    return buf;
}

pub fn read_bool(reader: &mut BufReader<File>) -> bool {
    let mut buf = vec![0; 1];
    reader.read_exact(&mut buf).unwrap();
    buf.reverse();
    return buf[0] == 1;
}

pub fn read_u8(reader: &mut BufReader<File>) -> u8 {
    let mut buf = vec![0; 1];
    reader.read_exact(&mut buf).unwrap();
    buf.reverse();
    return u8::from_le_bytes(buf.try_into().unwrap());
}

pub fn read_u16(reader: &mut BufReader<File>) -> u16 {
    let mut buf = vec![0; 2];
    reader.read_exact(&mut buf).unwrap();
    buf.reverse();
    return u16::from_le_bytes(buf.try_into().unwrap());
}

pub fn read_u32(reader: &mut BufReader<File>) -> u32 {
    let mut buf = vec![0; 4];
    reader.read_exact(&mut buf).unwrap();
    buf.reverse();
    return u32::from_le_bytes(buf.try_into().unwrap());
}

pub fn read_float(reader: &mut BufReader<File>) -> f32 {
    let mut buf = vec![0; 4];
    reader.read_exact(&mut buf).unwrap();
    buf.reverse();
    return f32::from_le_bytes(buf.try_into().unwrap());
}