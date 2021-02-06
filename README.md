# do-core1

The `do-core1` is a simple 32-bit processor architecture, mostly for educational purposes.

It aims at being a support for system programming and computer architecture fundamentals courses.

## Instruction Set Architecture

The do-core1 [Instruction Set Architecture (ISA)](https://en.wikipedia.org/wiki/Instruction_set) is a simple [Reduced Instruction Set Computer (RISC)](https://en.wikipedia.org/wiki/Reduced_instruction_set_computer)
processor architecture, with a very limited memory model and instruction and register set.

### Registers

`do-core1` exposes **32 registers**, from `R0` to`R31`.

It also uses one Program Counter (`PC`) register and a operation flags (`RFLAGS`) register.

All `do-core1` registers are **32 bits wide**.

### Memory Model

`do-core1` can address up to **4GiB (4 Giga Bytes) of physical memory**.

### Instruction Set

`do-core1` is a [RISC](https://en.wikipedia.org/wiki/Reduced_instruction_set_computer) architecture and executes fixed-length
instructions of 32 bits.

The `do-core1` is a 2-operand architecture, i.e. its instruction takes at most 2 operands.
The result of the instruction is always stored in the first operand.

`do-core1` operands are register indexes i.e. the instruction operates on `R[op]`.
For example, an operand set to 14 is addressing R14.

When using 2 operands, a `do-core1` instruction can be split into an operation code (opcode),
the first operand (op0) and the second operand (op1).
The opcode is 6 bits long, and operands are 5 bits long:

```
do-core instruction with 2 operands:

Bits  |15              11|10                6|5               0|
      ----------------------------------------------------------
      | op1 (bits 11-15) | op0 (bits 6-10) | Opcode (bits 0-5) |
      ----------------------------------------------------------
```

The `do-core1` is a [load-store](https://en.wikipedia.org/wiki/Load%E2%80%93store_architecture)
architecture and supports the following instructions:


| Opcode | Instruction  | Description                                                                                    |
|--------|--------------|------------------------------------------------------------------------------------------------|
| `0x00` | `LDW Rn, Rm` | **L**oa**D** **W**ord: Load the 32-bit value at the memory address contained in `Rm` into `Rn` |
| `0x01` | `STW Rn, Rm` | **ST**ore **W**ord: Store the 32-bit value from `Rn` into the memory address contained in `Rm` |
| `0x02` | `ADD Rn, Rm` | **ADD**: Add the value contained in `Rm` into `Rn` (`Rn = Rn + Rm`)                            |
| `0x03` | `XOR Rn, Rm` | e**X**clusive **OR**: Perform a bitwise exclusive OR between `Rn` and `Rm` (`Rn = Rn ^ Rm`)    |
