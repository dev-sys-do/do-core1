use clap::Parser;
use do_core::core::Core;
use do_core::Error;

#[derive(Parser)]
#[clap(version, author)]
struct DoCoreOpts {
    /// DO Core instruction
    #[clap(short, long)]
    insn: String,
}

fn main() -> Result<(), Error> {
    let mut cpu = Core::new();
    let opts: DoCoreOpts = DoCoreOpts::parse();

    cpu.dump("Initial CPU state");

    let insn = cpu.decode(u32::from_str_radix(opts.insn.trim_start_matches("0x"), 16).unwrap())?;
    cpu.execute(insn)?;

    cpu.dump("Final CPU state");

    Ok(())
}
