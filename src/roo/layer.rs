use std::{rc::Rc, cell::RefCell, io::{BufReader, Seek, Read}, fs::File};


#[derive(Debug)]
pub struct Layer {
  pub(crate) buf_reader: Rc<RefCell<BufReader<File>>>,

  pub pause: u16,
  pub pause_position_z: f32,
  pub position_z: f32,
  pub exposure_time: f32,
  pub light_off_delay: f32,
  pub wait_time_after_cure: f32,
  pub wait_time_after_lift: f32,
  pub wait_time_before_cure: f32,
  pub lift_height: f32,
  pub lift_speed: f32,
  pub lift_height_2: f32,
  pub lift_speed_2: f32,
  pub retract_height: f32,
  pub retract_speed: f32,
  pub retract_height_2: f32,
  pub retract_speed_2: f32,
  pub light_pwm: u16,
  pub data_length: u32,
  pub(crate) next_layer_address: u64,

  pub(crate) rle_data_position: u64,
}

impl Layer {
  pub fn get_rle_data(&mut self) -> Vec<u8> {
    let mut reader = self.buf_reader.borrow_mut();
    reader.seek(std::io::SeekFrom::Start(self.rle_data_position)).unwrap();
    let mut data = vec![0; self.data_length as usize];
    reader.read_exact(&mut data).unwrap();
    return data;
  }
}