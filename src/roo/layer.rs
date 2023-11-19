
#[derive(Debug)]
pub struct Layer {
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
  pub next_layer_address: u64,
}

impl Layer {
}