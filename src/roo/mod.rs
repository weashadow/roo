use std::{
    cell::RefCell,
    fs::File,
    io::{BufReader, Seek, SeekFrom},
    rc::Rc,
};
mod de_layer;
mod de_roo;
mod layer;
mod utils;

#[derive(Debug)]
pub enum DelayModes {
    LightOff = 0,
    WaitTime = 1,
}

#[derive(Debug)]
pub struct Roo {
    pub version: String,
    pub file_magic: [u8; 8],
    pub software_name: String,
    pub software_version: String,
    pub file_create_time: String,
    pub machine_name: String,
    pub machine_type: String,
    pub profile_name: String,
    pub anti_aliasing_level: u16,
    pub grey_levels: u16,
    pub blur_levels: u16,
    pub layer_count: u32,
    pub resolution_x: u16,
    pub resolution_y: u16,
    pub mirror_x: bool,
    pub mirror_y: bool,
    pub display_width: f32,
    pub display_height: f32,
    pub machine_z: f32,
    pub layer_height: f32,
    pub exposure_time: f32,
    pub delay_mode: DelayModes,
    pub light_off_delay: f32,
    pub bottom_wait_time_after_cure: f32,
    pub bottom_wait_time_after_lift: f32,
    pub bottom_wait_time_before_cure: f32,
    pub wait_time_after_cure: f32,
    pub wait_time_after_lift: f32,
    pub wait_time_before_cure: f32,
    pub bottom_exposure_time: f32,
    pub bottom_layer_count: u32,
    pub bottom_lift_height: f32,
    pub bottom_lift_speed: f32,
    pub lift_height: f32,
    pub lift_speed: f32,
    pub bottom_retract_height: f32,
    pub bottom_retract_speed: f32,
    pub retract_height: f32,
    pub retract_speed: f32,
    pub bottom_lift_height_2: f32,
    pub bottom_lift_speed_2: f32,
    pub lift_height_2: f32,
    pub lift_speed_2: f32,
    pub bottom_retract_height_2: f32,
    pub bottom_retract_speed_2: f32,
    pub retract_height_2: f32,
    pub retract_speed_2: f32,
    pub bottom_light_pwm: u16,
    pub light_pwm: u16,
    pub pre_layer_settings: bool,
    pub print_time: u32,
    pub volume: f32,
    pub material_grams: f32,
    pub material_cost: f32,
    pub price_current_symbol: String,
    pub gray_scale_levels: u8,
    pub transition_layer_count: u16,

    pub current_layer: u32,
    pub next_layer_address: u64,

    buf_reader: Rc<RefCell<BufReader<File>>>,
}

impl Roo {
    pub fn from_reader(reader: Rc<RefCell<BufReader<File>>>) -> Self {
        Roo::deserializer(reader)
    }

    pub fn from_path(path: &str) -> Self {
        let file = File::open(path).unwrap();
        Roo::from_file(file)
    }

    pub fn from_file(file: File) -> Self {
        let reader = BufReader::new(file);

        return Roo::from_reader(Rc::new(RefCell::new(reader)));
    }
}

impl Iterator for Roo {
    type Item = layer::Layer;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_layer >= self.layer_count {
            return None;
        }

        let reader = self.buf_reader.clone();
        reader
            .borrow_mut()
            .seek(SeekFrom::Start(self.next_layer_address))
            .unwrap();

        println!("next_layer_address: {:X}", self.next_layer_address);

        let layer = layer::Layer::deserializer(reader);

        self.current_layer += 1;
        self.next_layer_address = layer.next_layer_address;
        return Some(layer);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_layers() {
        let path = "test.goo";
        let file = File::open(path).unwrap();
        let mut roo = Roo::from_file(file);

        let mut count = 0;
        while let Some(layer) = roo.next() {
            count += 1;
        }

        assert_eq!(count, 32);
    }
}
