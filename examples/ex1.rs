// (c) 2015 Joost Yervante Damad <joost@damad.be>

extern crate rustysexp;

fn main() {
    let s = rustysexp::parser::parse_file("examples/SILABS_EFM32_QFN24.kicad_mod").unwrap();
    println!("{}", s);
}
