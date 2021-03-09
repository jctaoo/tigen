mod code_gen;
mod input;

use crate::code_gen::BTest;

extern crate clap;
use clap::{Arg, App, Subcommand};

fn main() {
    let matches = App::new("tigen")
        .version("0.1.0")
        .author("jctaoo <jctaoo@outlook.com>")
        .about("Generate code to typescript runtime type check.")
        .arg(
            Arg::new("file")
                .about("Sets the input typescript file to use")
                .required(true)
                .index(1)
        )
        .arg(
            Arg::new("interface")
                .about("Sets the interface name in <file> to generate runtime type check code")
                .required(true)
                .index(2)
        )
        .get_matches();
}
