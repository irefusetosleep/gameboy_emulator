use std::{fs, mem, os::raw::c_uchar};

mod cartridge;

fn main() {
    let liscense_name = cartridge::get_lic_code(0x01);

    println!("Liscense name: {liscense_name}");
}
