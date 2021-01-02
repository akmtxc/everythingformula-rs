use image::{*};
use std::{thread, time};
use uint::construct_uint;

construct_uint! {
  struct U8192(128);
}

fn main() {
  let lumized = image::open("images/newyearspruce.png").unwrap().into_luma8();
  lumized.save("images/lumanewyearspruce.png").unwrap();
  let twodecstr = image::open("images/lumanewyearspruce.png").unwrap().into_bytes(); // ок 

  let mut rightbyte: Vec<u8> = Vec::new();
  for (i, &item) in twodecstr.iter().enumerate() {
    if item < 127 || item == 255 {
      rightbyte.push(0);
    } else if item > 127 && item != 255 {
      rightbyte.push(1);
    }
  };

  let mut decstr = to_u8192(rightbyte);
  decstr = &decstr * 17 as u8;
  println!("{:?}", decstr);


  let secs = time::Duration::from_secs(2);

  thread::sleep(secs);
}

fn to_u8192(slice: Vec<u8>) -> U8192 {
  slice.iter().rev().fold(U8192::from(0), |acc, &b| U8192::from((acc << 1) | U8192::from(b)) + U8192::from(b))
}