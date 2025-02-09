#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

pub fn foo() {
    let a: i32 = 10;
    let b: i32 = 20;
    let _c1: i32 = (a + b);
    let _c2: i32 = (a - b);
    let _c3: i32 = (a * b);
    let _c4: f64 = (a / b) as f64;
    let _c5: i32 = -(a);
    let d: f64 = 2.0;
    let _e1: f64 = (a as f64 + d);
    let _e2: f64 = (a as f64 - d);
    let _e3: f64 = (a as f64 * d);
    let _e4: f64 = (a as f64 / d);
    let _f: f64 = -3.0;
    let _g: i32 = -(a);
}

pub fn add1(x: i8, y: i8) -> i16 {
    return (x + y) as i16;
}

pub fn add2(x: i16, y: i16) -> i32 {
    return (x + y) as i32;
}

pub fn add3(x: i32, y: i32) -> i64 {
    return (x + y) as i64;
}

pub fn add4(x: i64, y: i64) -> i64 {
    return (x + y);
}

pub fn add5(x: u8, y: u8) -> u16 {
    return (x + y) as u16;
}

pub fn add6(x: u16, y: u16) -> u32 {
    return (x + y) as u32;
}

pub fn add7(x: u32, y: u32) -> u64 {
    return (x + y) as u64;
}

pub fn add8(x: u64, y: u64) -> u64 {
    return (x + y);
}

pub fn add9(x: i8, y: u16) -> u32 {
    return (x as u16 + y) as u32;
}

pub fn sub(x: i8, y: i8) -> i8 {
    return (x - y);
}

pub fn mul(x: i8, y: i8) -> i16 {
    return (x * y) as i16;
}

pub fn fadd1(x: i8, y: f64) -> f64 {
    return (x as f64 + y);
}

pub fn show() {
    let rv: f64 = fadd1(6, 6.0);
    assert!(rv == 12 as f64);
    println!("{}", "OK");
}

pub fn main() {
    foo();
    show();
}
