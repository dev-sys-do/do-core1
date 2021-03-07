extern crate clap;

use clap::{App, Arg};
use do_core::core::Core;
use do_core::Error;

fn main() -> Result<(), Error> {
    let mut cpu = Core::new();
    let arguments = App::new("do-core1")
        .about("do-core1 emulator")
        .arg(
            Arg::with_name("instruction")
                .long("instruction")
                .help("do-core1 instruction to execute")
                .takes_value(true),
        )
        .get_matches();

    let insn_string = arguments
        .value_of("instruction")
        .expect("Missing --instruction argument")
        .trim_start_matches("0x");

    cpu.dump("Initial CPU state");

    let insn = cpu.decode(u16::from_str_radix(insn_string, 16).unwrap())?;
    cpu.execute(insn)?;

    cpu.dump("Final CPU state");

    Ok(())
}
