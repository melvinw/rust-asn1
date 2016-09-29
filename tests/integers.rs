extern crate asn1;
use asn1::aper::{self, Constraint, Constraints};
use std::i32;

#[test]
fn unconstrained_negative() {
    let data = b"\x04\xff\xff\xff\xd5";
    let mut d = aper::Decoder::new(data);
    assert_eq!(-43, d.decode_int(None, None).unwrap());
}

#[test]
fn unconstrained_positive() {
    let data = b"\x02\x10\x00";
    let mut d = aper::Decoder::new(data);
    assert_eq!(4096, d.decode_int(None, None).unwrap());
}

#[test]
fn constrained_bounds() {
    let data = b"\x00";
    let mut d = aper::Decoder::new(data);
    assert_eq!(4000, d.decode_int(Some(4000), Some(4255)).unwrap());
}

#[test]
fn semiconstrainted_bounds() {
    let data = b"\x02\x10\x01";
    let mut d = aper::Decoder::new(data);
    assert_eq!(4096, d.decode_int(Some(-1), None).unwrap());
}

#[test]
fn std_i8() {
    let data_min = b"\x00"; // i8::MIN
    let data_med = b"\xab"; // 43
    let data_max = b"\xff"; // i8::MAX
    let mut d = aper::Decoder::new(data_min);
    assert_eq!(std::i8::MIN, d.decode::<i8>(Constraints{value: None, size: None}).unwrap());
    d = aper::Decoder::new(data_med);
    assert_eq!(43 as i8, d.decode::<i8>(Constraints{value: None, size: None}).unwrap());
    d = aper::Decoder::new(data_max);
    assert_eq!(std::i8::MAX, d.decode::<i8>(Constraints{value: None, size: None}).unwrap());
}

#[test]
fn std_i16() {
    let data_min = b"\x00\x00"; // i16::MIN
    let data_med = b"\x80\x2b"; // 43
    let data_max = b"\xff\xff"; // i16::MAX
    let mut d = aper::Decoder::new(data_min);
    assert_eq!(std::i16::MIN, d.decode::<i16>(Constraints{value: None, size: None}).unwrap());
    d = aper::Decoder::new(data_med);
    assert_eq!(43 as i16, d.decode::<i16>(Constraints{value: None, size: None}).unwrap());
    d = aper::Decoder::new(data_max);
    assert_eq!(std::i16::MAX, d.decode::<i16>(Constraints{value: None, size: None}).unwrap());
}

#[test]
fn std_i32() {
    let data_min = b"\x04\x00\x00\x00\x00"; // i32::MIN
    let data_med = b"\x04\x80\x00\x00\x2b"; // 43
    let data_max = b"\x04\xff\xff\xff\xff"; // i32::MAX
    let mut d = aper::Decoder::new(data_min);
    assert_eq!(std::i32::MIN, d.decode::<i32>(Constraints{value: None, size: None}).unwrap());
    d = aper::Decoder::new(data_med);
    assert_eq!(43 as i32, d.decode::<i32>(Constraints{value: None, size: None}).unwrap());
    d = aper::Decoder::new(data_max);
    assert_eq!(std::i32::MAX, d.decode::<i32>(Constraints{value: None, size: None}).unwrap());
}

#[test]
fn std_u8() {
    let data_min = b"\x00"; // u8::MIN
    let data_med = b"\x2b"; // 43
    let data_max = b"\xff"; // u8::MAX
    let mut d = aper::Decoder::new(data_min);
    assert_eq!(std::u8::MIN, d.decode::<u8>(Constraints{value: None, size: None}).unwrap());
    d = aper::Decoder::new(data_med);
    assert_eq!(43 as u8, d.decode::<u8>(Constraints{value: None, size: None}).unwrap());
    d = aper::Decoder::new(data_max);
    assert_eq!(std::u8::MAX, d.decode::<u8>(Constraints{value: None, size: None}).unwrap());
}

#[test]
fn std_u16() {
    let data_min = b"\x00\x00"; // u16::MIN
    let data_med = b"\x00\x2b"; // 43
    let data_max = b"\xff\xff"; // u16::MAX
    let mut d = aper::Decoder::new(data_min);
    assert_eq!(std::u16::MIN, d.decode::<u16>(Constraints{value: None, size: None}).unwrap());
    d = aper::Decoder::new(data_med);
    assert_eq!(43 as u16, d.decode::<u16>(Constraints{value: None, size: None}).unwrap());
    d = aper::Decoder::new(data_max);
    assert_eq!(std::u16::MAX, d.decode::<u16>(Constraints{value: None, size: None}).unwrap());
}

#[test]
fn std_u32() {
    let data_min = b"\x04\x00\x00\x00\x00"; // u32::MIN
    let data_med = b"\x04\x00\x00\x00\x2b"; // 43
    let data_max = b"\x04\xff\xff\xff\xff"; // u32::MAX
    let mut d = aper::Decoder::new(data_min);
    assert_eq!(std::u32::MIN, d.decode::<u32>(Constraints{value: None, size: None}).unwrap());
    d = aper::Decoder::new(data_med);
    assert_eq!(43 as u32, d.decode::<u32>(Constraints{value: None, size: None}).unwrap());
    d = aper::Decoder::new(data_max);
    assert_eq!(std::u32::MAX, d.decode::<u32>(Constraints{value: None, size: None}).unwrap());
}
