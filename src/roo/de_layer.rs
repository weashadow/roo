use std::{cell::RefCell, fs::File, io::{BufReader, Seek}, rc::Rc, ops::Add};

use super::{layer::Layer, utils::{read_u16, read_float, read_u32}};

impl Layer {
    pub fn deserializer(reader: Rc<RefCell<BufReader<File>>>) -> Self {
        let reader = &mut reader.borrow_mut();
        let pause = read_u16(reader);
        let pause_position_z = read_float(reader);
        let position_z = read_float(reader);
        let exposure_time = read_float(reader);
        let light_off_delay = read_float(reader);
        let wait_time_after_cure = read_float(reader);
        let wait_time_after_lift = read_float(reader);
        let wait_time_before_cure = read_float(reader);
        let lift_height = read_float(reader);
        let lift_speed = read_float(reader);
        let lift_height_2 = read_float(reader);
        let lift_speed_2 = read_float(reader);
        let retract_height = read_float(reader);
        let retract_speed = read_float(reader);
        let retract_height_2 = read_float(reader);
        let retract_speed_2 = read_float(reader);
        let light_pwm = read_u16(reader);
        // skip 2 bytes for delimiter
        reader.seek(std::io::SeekFrom::Current(2)).unwrap();
        let data_length = read_u32(reader);

        reader.seek(std::io::SeekFrom::Current(data_length.into())).unwrap();
        // get reader pointer
        reader.seek(std::io::SeekFrom::Current(2)).unwrap();
        let pos = reader.stream_position().unwrap();
        let next_layer_address = pos;
        
        return Self {
          pause,
          pause_position_z,
          position_z,
          exposure_time,
          light_off_delay,
          wait_time_after_cure,
          wait_time_after_lift,
          wait_time_before_cure,
          lift_height,
          lift_speed,
          lift_height_2,
          lift_speed_2,
          retract_height,
          retract_speed,
          retract_height_2,
          retract_speed_2,
          light_pwm,
          data_length,
          next_layer_address,
        };
    }
}
