use std::{fs::File, io::BufReader};
use std::io::Read;



#[derive(Debug, )]
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
}

impl Roo {
  pub fn from_reader(reader: &mut BufReader<File>) -> Self {
    Roo::deserializer(reader)
  }

  pub fn from_path(path: &str) -> Self {
    let file = File::open(path).unwrap();
    Roo::from_file(file)
  }

  pub fn from_file(file: File) -> Self {
    let mut reader = BufReader::new(file);

    return Roo::from_reader(&mut reader);
  }

}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn load_from_path() {
    let path = "test.goo";
    let result = Roo::from_path(path);
    println!("{:?}", result);
    todo!();
  }
}