fn add(op0: u16, op1: u16) -> u16 {
    op0 + op1
}

fn main() {
    let mut r0: u16 = 10;
    let r1: u16 = 20;

    // ADD r0, r1
    r0 = add(r0, r1);

    println!("do-core-1: {}", r0);
}
