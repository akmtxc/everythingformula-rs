extern crate image; // это для оперирования с картинками
use std::num::Wrapping;
use num::{BigUint};

fn main() {
  let lumized = image::open("../images/smallspruce.png").unwrap().into_luma8(); // перевожу в чёрно-белую систему цветов
  lumized.save("../images/lumasmallspruce.png").unwrap(); // сохраняю
  let twodecstr = image::open("../images/lumasmallspruce.png").unwrap().into_bytes(); // перевожу в байты

  let mut rightbyte: Vec<u8> = Vec::new(); 
  for (i, &item) in twodecstr.iter().enumerate() {
    if item < 127 {
      rightbyte.push(1);
    } else if item > 127 {
      rightbyte.push(0);
    }
  };
  
  let mut arrstr: &[u8] = &rightbyte;
  println!("{:?}", arrstr);
  let mut decstr = BigUint::from_bytes_le(arrstr);
  decstr = &decstr * 17 as u8;
  println!("{}", decstr);
}
