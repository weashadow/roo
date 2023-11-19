use std::cell::RefCell;
use std::io::Seek;
use std::rc::Rc;
use std::{fs::File, io::BufReader};

use super::utils::{read_string, read_bytes, read_u16, read_bool, read_float, read_u32, read_u8};

use super::Roo;

static FILE_VERSION: &str = "V3.0";

static FILE_MAGIC: [u8; 8] = [0x07, 0x00, 0x00, 0x00, 0x44, 0x4C, 0x50, 0x00];


impl Roo {
    pub fn deserializer(r:Rc<RefCell<BufReader<File>>>) -> Self {
        let reader = &mut r.borrow_mut();
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
        let display_width = read_float(reader);
        let display_height = read_float(reader);

        let machine_z = read_float(reader);
        let layer_height = read_float(reader);
        let exposure_time = read_float(reader);

        let delay_mode = match read_u8(reader) {
            0 => crate::roo::DelayModes::LightOff,
            1 => crate::roo::DelayModes::WaitTime,
            _ => panic!("Delay mode is not supported"),
        };

        let light_off_delay = read_float(reader);
        let bottom_wait_time_after_cure = read_float(reader);
        let bottom_wait_time_after_lift = read_float(reader);
        let bottom_wait_time_before_cure = read_float(reader);
        let wait_time_after_cure = read_float(reader);
        let wait_time_after_lift = read_float(reader);
        let wait_time_before_cure = read_float(reader);
        let bottom_exposure_time = read_float(reader);
        let bottom_layer_count = read_u32(reader);
        let bottom_lift_height = read_float(reader);
        let bottom_lift_speed = read_float(reader);
        let lift_height = read_float(reader);
        let lift_speed = read_float(reader);
        let bottom_retract_height = read_float(reader);
        let bottom_retract_speed = read_float(reader);
        let retract_height = read_float(reader);
        let retract_speed = read_float(reader);
        let bottom_lift_height_2 = read_float(reader);
        let bottom_lift_speed_2 = read_float(reader);
        let lift_height_2 = read_float(reader);
        let lift_speed_2 = read_float(reader);
        let bottom_retract_height_2 = read_float(reader);
        let bottom_retract_speed_2 = read_float(reader);
        let retract_height_2 = read_float(reader);
        let retract_speed_2 = read_float(reader);
        let bottom_light_pwm = read_u16(reader);
        let light_pwm = read_u16(reader);
        let pre_layer_settings = read_bool(reader);
        let print_time = read_u32(reader);
        let volume = read_float(reader);
        let material_grams = read_float(reader);
        let material_cost = read_float(reader);
        let price_current_symbol = read_string(reader, 8);

        // skip layer def address for 8 bytes
        // reader.seek(std::io::SeekFrom::Current(8)).unwrap();
        let layer_def_address = read_u32(reader);

        let gray_scale_levels = read_u8(reader);
        let transition_layer_count = read_u16(reader);

        return Self {
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
            display_width,
            display_height,
            machine_z,
            layer_height,
            exposure_time,
            delay_mode,
            light_off_delay,
            bottom_wait_time_after_cure,
            bottom_wait_time_after_lift,
            bottom_wait_time_before_cure,
            wait_time_after_cure,
            wait_time_after_lift,
            wait_time_before_cure,
            bottom_exposure_time,
            bottom_layer_count,
            bottom_lift_height,
            bottom_lift_speed,
            lift_height,
            lift_speed,
            bottom_retract_height,
            bottom_retract_speed,
            retract_height,
            retract_speed,
            bottom_lift_height_2,
            bottom_lift_speed_2,
            lift_height_2,
            lift_speed_2,
            bottom_retract_height_2,
            bottom_retract_speed_2,
            retract_height_2,
            retract_speed_2,
            bottom_light_pwm,
            light_pwm,
            pre_layer_settings,
            print_time,
            volume,
            material_grams,
            material_cost,
            price_current_symbol,
            gray_scale_levels,
            transition_layer_count,

            current_layer: 0,
            next_layer_address: layer_def_address as u64,

            buf_reader: r.clone(),
        };
    }
}
