use std::io::{Read, Seek};
use std::{fs::File, io::BufReader};

use crate::roo::Roo;

static FILE_VERSION: &str = "V3.0";

static FILE_MAGIC: [u8; 8] = [0x07, 0x00, 0x00, 0x00, 0x44, 0x4C, 0x50, 0x00];

static DELIMITER: [u8; 2] = [0x0D, 0x0A];

fn read_string(reader: &mut BufReader<File>, len: usize) -> String {
    let mut buf = vec![0; len];
    reader.read_exact(&mut buf).unwrap();
    buf.retain(|&x| x != 0);
    return String::from_utf8_lossy(&buf).to_string();
}

fn read_bytes(reader: &mut BufReader<File>, len: usize) -> Vec<u8> {
    let mut buf = vec![0; len];
    reader.read_exact(&mut buf).unwrap();
    return buf;
}

fn read_bool(reader: &mut BufReader<File>) -> bool {
    let mut buf = vec![0; 1];
    reader.read_exact(&mut buf).unwrap();
    buf.reverse();
    return buf[0] == 1;
}

fn read_u16(reader: &mut BufReader<File>) -> u16 {
    let mut buf = vec![0; 2];
    reader.read_exact(&mut buf).unwrap();
    buf.reverse();
    return u16::from_le_bytes(buf.try_into().unwrap());
}

fn read_u32(reader: &mut BufReader<File>) -> u32 {
    let mut buf = vec![0; 4];
    reader.read_exact(&mut buf).unwrap();
    buf.reverse();
    return u32::from_le_bytes(buf.try_into().unwrap());
}


impl Roo {
    pub fn deserializer(reader: &mut BufReader<File>) -> Roo {
        let version = read_string(reader, 4);
        assert!(version == FILE_VERSION, "File version is not supported");
        let magic_num = read_bytes(reader, 8);
        assert!(
            magic_num == FILE_MAGIC,
            "File magic number is not supported"
        );
        let software_name = read_string(reader, 32);
        let software_version = read_string(reader, 24);
        let file_create_time = read_string(reader, 24);
        let machine_name = read_string(reader, 32);
        let machine_type = read_string(reader, 32);
        let profile_name = read_string(reader, 32);
        let anti_aliasing_level = read_u16(reader);
        let grey_levels = read_u16(reader);
        let blur_levels = read_u16(reader);

        // skip small thumbnail
        reader.seek(std::io::SeekFrom::Current(116*116*2)).unwrap();
        // skip delimiter
        reader.seek(std::io::SeekFrom::Current(2)).unwrap();
        // skip big thumbnail
        reader.seek(std::io::SeekFrom::Current(290*290*2)).unwrap();
        // skip delimiter
        reader.seek(std::io::SeekFrom::Current(2)).unwrap();

        let layer_count = read_u32(reader);
        let resolution_x = read_u16(reader);
        let resolution_y = read_u16(reader);
        let mirror_x = read_bool(reader);
        let mirror_y = read_bool(reader);

        return Roo {
            version,
            file_magic: TryInto::try_into(magic_num).unwrap(),
            software_name,
            software_version,
            file_create_time,
            machine_name,
            machine_type,
            profile_name,
            anti_aliasing_level,
            grey_levels,
            blur_levels,
            layer_count,
            resolution_x,
            resolution_y,
            mirror_x,
            mirror_y,
        };
    }
}
