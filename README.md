# do-core1

The `do-core1` is a simple processor architecture, mostly for educational purposes.

It aims at being a support for system programming and computer architecture fundamentals courses.

## Instruction Set Architecture

The do-core1 [Instruction Set Architecture (ISA)](https://en.wikipedia.org/wiki/Instruction_set) is a simple [Reduced Instruction Set Computer (RISC)](https://en.wikipedia.org/wiki/Reduced_instruction_set_computer)
processor architecture, with a very limited memory model and instruction and register set.

### Registers

`do-core1` exposes **8 general purpose registers**: `R0`, `R1`, `R2`, `R3`, `R4`, `R5`, `R6`, `R7`.

It also uses one Instruction Pointer (`RIP`) register and a operation flags (`RFLAGS`) register.

All `do-core1` registers are **16 bits wide**.

### Memory Model

`do-core1` can address up to **4KiB (4096 bytes) of physical memory**.

### Instruction Set

`do-core1` is a [RISC](https://en.wikipedia.org/wiki/Reduced_instruction_set_computer) architecture and executes fixed-length
instructions of 16 bits.

The `do-core1` is a 2-operand architecture, i.e. its instruction takes at most 2 operands.
`do-core1` operands are register indexes.

A `do-core1` instruction can be split into an operation code (opcode), the first operand (op0)
and the second operand (op1). The opcode is 8 bits long, and both operands are 4 bits long:

```
do-core instruction (16 bits)

Bits  |15                   8|7                4|3               0|
      -------------------------------------------------------------
      |  Opcode (bits 15-8)  |  op0 (bits 7-4)  | op1 (bits 3-0)  |
      -------------------------------------------------------------
```

The `do-core1` is a [load-store](https://en.wikipedia.org/wiki/Load%E2%80%93store_architecture)
architecture and supports the following instructions:


| Opcode | Instruction  | Description                                                          |
|--------|--------------|----------------------------------------------------------------------|
| `0x00` | `LD Rn, Rm`  | Load the value at the memory address contained in `Rm` into `Rn`     |
| `0x01` | `ST Rn, Rm`  | Store the value from `Rn` into the memory address contained in `Rm`  |
| `0x02` | `ADD Rn, Rm` | Add the value contained in `Rm` into `Rn` (`Rn = Rn + Rm`)           |
| `0x03` | `XOR Rn, Rm` | Perform a bitwise exclusive OR between `Rn` and `Rm`(`Rn = Rn ^ Rm`) |
