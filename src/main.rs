fn add(op0: u32, op1: u32) -> u32 {
    op0 + op1
}

fn main() {
    let mut r0: u32 = 10;
    let r1: u32 = 20;

    // ADD r0, r1
    r0 = add(r0, r1);

    println!("do-core-1: {}", r0);
}
