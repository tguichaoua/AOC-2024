use core::fmt::Write;
use std::ops::ControlFlow;

use itertools::Itertools;

advent_of_code::solution!(*, 1);

struct Input {
    registers: [u64; 3],
    instructions: Vec<u8>,
}

fn parse(input: &str) -> Input {
    debug_assert!(input.is_ascii());
    let (a, b, c, _, instructions) = input.lines().collect_tuple().unwrap();

    const REGISTER_PREFIX_LEN: usize = "Register A: ".len();
    const INSTRUCTION_PREFIX: &str = "Program: ";

    let a = a[REGISTER_PREFIX_LEN..].parse().unwrap();
    let b = b[REGISTER_PREFIX_LEN..].parse().unwrap();
    let c = c[REGISTER_PREFIX_LEN..].parse().unwrap();

    let instructions = instructions.strip_prefix(INSTRUCTION_PREFIX).unwrap();

    let instructions = (0..instructions.len())
        .step_by(2)
        .map(|i| {
            let c = instructions.as_bytes()[i];
            debug_assert!(c.is_ascii_digit());
            c - b'0'
        })
        .collect();

    Input {
        registers: [a, b, c],
        instructions,
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum RunExit {
    /// The program stop itself because it reach the end.
    Finished,
    /// The program was stop.
    Stopped,
}

fn run(
    register_a: u64,
    instructions: &[u8],
    mut output: impl FnMut(u64) -> ControlFlow<()>,
) -> RunExit {
    let mut vm = VM {
        registers: Registers {
            a: register_a,
            b: 0,
            c: 0,
        },
        instructions,
    };
    let mut pc = 0;

    loop {
        let Some(&operand) = vm.instructions.get(pc + 1) else {
            break;
        };
        let Some(&opcode) = vm.instructions.get(pc) else {
            break;
        };

        match opcode {
            0 => {
                let num = vm.registers.a;
                let denom_pow = vm.combo_operand(operand);
                let result = num >> denom_pow;
                vm.registers.a = result;
            }
            1 => {
                let result = vm.registers.b ^ (u64::from(operand));
                vm.registers.b = result;
            }
            2 => {
                let value = vm.combo_operand(operand);
                let result = value % 8;
                vm.registers.b = result;
            }
            3 => {
                if vm.registers.a != 0 {
                    pc = usize::from(operand);
                    continue; // skip `pc` increment
                }
            }
            4 => {
                let result = vm.registers.b ^ vm.registers.c;
                vm.registers.b = result;
            }
            5 => {
                let result = vm.combo_operand(operand) % 8;

                match output(result) {
                    ControlFlow::Continue(_) => {}
                    ControlFlow::Break(_) => return RunExit::Stopped,
                };
            }
            6 => {
                let num = vm.registers.a;
                let denom_pow = vm.combo_operand(operand);
                let result = num >> denom_pow;
                vm.registers.b = result;
            }
            7 => {
                let num = vm.registers.a;
                let denom_pow = vm.combo_operand(operand);
                let result = num >> denom_pow;
                vm.registers.c = result;
            }
            _ => unreachable!(),
        }

        pc += 2;
    }

    return RunExit::Finished;

    struct Registers {
        a: u64,
        b: u64,
        c: u64,
    }

    struct VM<'a> {
        registers: Registers,
        instructions: &'a [u8],
    }

    impl VM<'_> {
        fn combo_operand(&self, operand: u8) -> u64 {
            match operand {
                0..=3 => u64::from(operand),
                4 => self.registers.a,
                5 => self.registers.b,
                6 => self.registers.c,
                _ => unreachable!(),
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let Input {
        registers: [a, b, c],
        instructions,
    } = parse(input);

    assert!(b == 0);
    assert!(c == 0);

    let mut output = String::new();

    run(a, &instructions, |value| {
        if output.is_empty() {
            write!(output, "{value}").unwrap();
        } else {
            write!(output, ",{value}").unwrap();
        }

        ControlFlow::Continue(())
    });

    Some(output)
}

/* -------------------------------------------------------------------------- */

/*

pub fn part_two(input: &str) -> Option<u64> {
    let Input {
        registers: [_, b, c],
        instructions,
    } = parse(input);

    assert!(b == 0);
    assert!(c == 0);

    let mut halted = Vec::new();
    let mut alternatives = Vec::new();
    alternatives.push(VM {
        registers: Registers {
            a: Value::MysInt(MysInt::new()),
            b: Value::Int(0),
            c: Value::Int(0),
        },
        instructions: &instructions,
        pc: 0,
        output: Vec::new(),
        solved: SolvedMystery::new(),
    });

    while let Some(mut vm) = alternatives.pop() {
        loop {
            let Some(&operand) = vm.instructions.get(vm.pc + 1) else {
                break;
            };
            let Some(&opcode) = vm.instructions.get(vm.pc) else {
                break;
            };

            match opcode {
                0 => {
                    let num = vm.registers.a;
                    let denom_pow = vm.combo_operand(operand);
                    let result = Value::rshift(num, denom_pow);
                    vm.registers.a = result;
                }
                1 => {
                    let b = vm.registers.b;
                    let rhs = Value::Int(u64::from(operand));
                    let result = Value::xor(b, rhs);
                    vm.registers.b = result;
                }
                2 => {
                    let value = vm.combo_operand(operand);
                    let result = Value::mod_8(value);
                    vm.registers.b = result;
                }
                3 => {
                    {
                        // we crate a new alternative in which register A is 0
                        // and the jump didn't occurs.
                        let mut alternative = vm.clone();
                        alternative.registers.a = Value::Int(0);
                        alternative.pc += 2;
                        alternatives.push(alternative);
                    }

                    // In this reality register A is not 0
                    vm.pc = usize::from(operand);
                    continue; // skip `pc` increment
                }
                4 => {
                    let b = vm.registers.b;
                    let c = vm.registers.c;
                    let result = Value::xor(b, c);
                    vm.registers.b = result;
                }
                5 => {
                    let result = vm.combo_operand(operand).mod_8();
                    vm.output.push(result);

                    // We have already too much output
                    if vm.output.len() > instructions.len() {
                        break;
                    }
                }
                6 => {
                    let num = vm.registers.a;
                    let denom_pow = vm.combo_operand(operand);
                    let result = Value::rshift(num, denom_pow);
                    vm.registers.b = result;
                }
                7 => {
                    let num = vm.registers.a;
                    let denom_pow = vm.combo_operand(operand);
                    let result = Value::rshift(num, denom_pow);
                    vm.registers.c = result;
                }
                _ => unreachable!(),
            }

            vm.pc += 2;
        }

        // We keep the vm only if it's output may be the program.
        if vm.output.len() == instructions.len() {
            halted.push(vm);
        }
    }

    assert!(halted.len() == 1);

    let [vm] = halted.try_into().ok().unwrap();

    return None;

    #[derive(Clone, Copy)]
    enum Bit {
        Zero,
        One,
    }

    #[derive(Clone, Copy)]
    enum MysBit {
        Zero,
        One,
        Mystery { offset: u8, switched: bool },
    }

    #[derive(Clone, Copy)]
    struct MysInt {
        bits: [MysBit; 64],
    }

    #[derive(Clone, Copy)]
    struct SolvedMystery {
        bits: [Option<Bit>; 64],
    }

    #[derive(Clone, Copy)]
    enum Value {
        Int(u64),
        MysInt(MysInt),
    }

    #[derive(Clone)]
    struct Registers {
        a: Value,
        b: Value,
        c: Value,
    }

    #[derive(Clone)]
    struct VM<'a> {
        registers: Registers,
        instructions: &'a [u8],
        pc: usize,
        output: Vec<Value>,
        solved: SolvedMystery,
    }

    impl SolvedMystery {
        fn new() -> Self {
            Self {
                bits: [const { None }; 64],
            }
        }
    }

    impl Bit {
        fn opposite(self) -> Self {
            match self {
                Bit::Zero => Bit::One,
                Bit::One => Bit::Zero,
            }
        }
    }

    impl SolvedMystery {
        fn with(self, offset: u8, bit: Bit) -> Self {
            let mut bits = self.bits;
            bits[offset as usize] = Some(bit);
            Self { bits }
        }

        fn solve_with(
            self,
            offset: u8,
            switched: bool,
        ) -> ((Bit, SolvedMystery), Option<(Bit, SolvedMystery)>) {
            match self.bits[offset as usize] {
                Some(bit) if !switched => ((bit, self), None),
                Some(bit) => ((bit.opposite(), self), None),
                None => {
                    let zero = self.with(offset, Bit::Zero);
                    let one = self.with(offset, Bit::One);
                    if switched {
                        ((Bit::One, zero), Some((Bit::Zero, one)))
                    } else {
                        ((Bit::Zero, zero), Some((Bit::One, one)))
                    }
                }
            }
        }
    }

    impl MysInt {
        fn new() -> Self {
            let bits = core::array::from_fn(|offset| MysBit::Mystery {
                offset: offset.try_into().unwrap(),
                switched: false,
            });
            Self { bits }
        }

        fn rshift(self, amount: usize) -> Self {
            let mut bits = [const { MysBit::Zero }; 64];
            let len = bits.len();
            bits[0..len - amount].copy_from_slice(&self.bits[amount..]);
            Self { bits }
        }

        fn xor(self, mask: u64) -> Self {
            let mut bits = self.bits;

            let mask = (0..u64::BITS).map(|i| ((mask >> i) & 1) == 1);

            bits.iter_mut().zip(mask).for_each(|(bit, mask)| {
                *bit = match (*bit, mask) {
                    (MysBit::Zero, true) => MysBit::One,
                    (MysBit::One, true) => MysBit::Zero,
                    (MysBit::Mystery { offset, switched }, mask) => MysBit::Mystery {
                        offset,
                        switched: switched ^ mask,
                    },
                    (bit, false) => bit,
                };
            });

            Self { bits }
        }

        fn mod_8(self) -> Self {
            let mut bits = self.bits;
            bits[3..].fill(MysBit::Zero);
            Self { bits }
        }

        fn solve(self, solved: SolvedMystery) -> impl Iterator<Item = (u64, SolvedMystery)> {
            todo!();
            core::iter::empty()
        }
    }

    impl Value {
        fn rshift(self, rhs: Value, solved: SolvedMystery) -> Self {
            match (self, rhs) {
                (Value::Int(lhs), Value::Int(rhs)) => Value::Int(lhs >> rhs),
                (Value::MysInt(lhs), Value::Int(rhs)) => {
                    Value::MysInt(lhs.rshift(rhs.try_into().unwrap()))
                }

                (Value::Int(_), Value::MysInt(_)) => {
                    todo!();
                }

                (Value::MysInt(_), Value::MysInt(_)) => todo!(),
            }
        }

        fn xor(self, rhs: Value) -> Self {
            match (self, rhs) {
                (Value::Int(lhs), Value::Int(rhs)) => Value::Int(lhs ^ rhs),
                (Value::MysInt(lhs), Value::Int(rhs)) => {
                    Value::MysInt(lhs.rshift(rhs.try_into().unwrap()))
                }

                (Value::Int(_), Value::MysInt(_)) => todo!(),
                (Value::MysInt(_), Value::MysInt(_)) => todo!(),
            }
        }

        fn mod_8(self) -> Self {
            match self {
                Value::Int(value) => Value::Int(value % 8),
                Value::MysInt(value) => Value::MysInt(value.mod_8()),
            }
        }
    }

    impl VM<'_> {
        fn combo_operand(&self, operand: u8) -> Value {
            match operand {
                0..=3 => Value::Int(u64::from(operand)),
                4 => self.registers.a,
                5 => self.registers.b,
                6 => self.registers.c,
                _ => unreachable!(),
            }
        }
    }
}

*/

/* -------------------------------------------------------------------------- */

// pub fn part_two(input: &str) -> Option<u64> {
//     let Input {
//         registers: [_, b, c],
//         instructions,
//     } = parse(input);

//     assert!(b == 0);
//     assert!(c == 0);

//     for n in 0.. {
//         let min = n * u64::from(u32::MAX);
//         let max = (n + 1) * u64::from(u32::MAX);

//         println!("{n} [{min}, {max})");

//         let result = (min..max)
//             .into_par_iter()
//             .map_init(
//                 || Vec::with_capacity(instructions.len()),
//                 |outputs, a| {
//                     outputs.clear();

//                     let exit = run(a, &instructions, |value| {
//                         let value: u8 = value.try_into().unwrap();

//                         if outputs.len() == instructions.len() {
//                             // The output is already full
//                             return ControlFlow::Break(());
//                         }

//                         let expected = instructions[outputs.len()];
//                         if value != expected {
//                             return ControlFlow::Break(());
//                         }

//                         outputs.push(value);

//                         ControlFlow::Continue(())
//                     });

//                     if (exit == RunExit::Finished) && outputs == &instructions {
//                         Some(a)
//                     } else {
//                         None
//                     }
//                 },
//             )
//             .flatten()
//             .min();

//         if let Some(result) = result {
//             return Some(result);
//         }
//     }

//     unreachable!()
// }

/* -------------------------------------------------------------------------- */

// pub fn part_two(input: &str) -> Option<u64> {
//     let Input {
//         registers: [_, b, c],
//         instructions,
//     } = parse(input);

//     assert!(b == 0);
//     assert!(c == 0);

//     let mut halted = Vec::new();
//     let mut alternatives = Vec::new();
//     alternatives.push(VM {
//         registers: Registers {
//             a: Value::MysteryGuy(MysteryGuy::new()),
//             b: Value::Int(0),
//             c: Value::Int(0),
//         },
//         instructions: &instructions,
//         pc: 0,
//         output: Vec::new(),
//         are_not_zero: Vec::new(),
//     });

//     while let Some(mut vm) = alternatives.pop() {
//         loop {
//             let Some(&operand) = vm.instructions.get(vm.pc + 1) else {
//                 break;
//             };
//             let Some(&opcode) = vm.instructions.get(vm.pc) else {
//                 break;
//             };

//             match opcode {
//                 0 => {
//                     let num = vm.registers.a.clone();
//                     let denom_pow = vm.combo_operand(operand);
//                     let result = Value::right_shift(num, denom_pow);
//                     vm.registers.a = result;
//                 }
//                 1 => {
//                     let b = vm.registers.b.clone();
//                     let rhs = Value::Int(u32::from(operand));
//                     let result = Value::xor(b, rhs);
//                     vm.registers.b = result;
//                 }
//                 2 => {
//                     let value = vm.combo_operand(operand);
//                     let result = Value::mod_8(value);
//                     vm.registers.b = result;
//                 }
//                 3 => {
//                     {
//                         // we crate a new alternative in which register A is 0
//                         // and the jump didn't occurs.
//                         let mut alternative = vm.clone();
//                         alternative.registers.a = Value::Int(0);
//                         alternative.pc += 2;
//                         alternatives.push(alternative);
//                     }

//                     // In this reality register A is not 0
//                     vm.are_not_zero.push(vm.registers.a.clone());
//                     vm.pc = usize::from(operand);
//                     continue; // skip `pc` increment
//                 }
//                 4 => {
//                     let b = vm.registers.b.clone();
//                     let c = vm.registers.c.clone();
//                     let result = Value::xor(b, c);
//                     vm.registers.b = result;
//                 }
//                 5 => {
//                     let result = vm.combo_operand(operand).mod_8();
//                     vm.output.push(result);

//                     // We have already too much output
//                     if vm.output.len() > instructions.len() {
//                         break;
//                     }
//                 }
//                 6 => {
//                     let num = vm.registers.a.clone();
//                     let denom_pow = vm.combo_operand(operand);
//                     let result = Value::right_shift(num, denom_pow);
//                     vm.registers.b = result;
//                 }
//                 7 => {
//                     let num = vm.registers.a.clone();
//                     let denom_pow = vm.combo_operand(operand);
//                     let result = Value::right_shift(num, denom_pow);
//                     vm.registers.c = result;
//                 }
//                 _ => unreachable!(),
//             }

//             vm.pc += 2;
//         }

//         // We keep the vm only if it's output may be the program.
//         if vm.output.len() == instructions.len() {
//             halted.push(vm);
//         }
//     }

//     assert!(halted.len() == 1);

//     let [vm] = halted.try_into().ok().unwrap();

//     return None;

//     #[derive(Copy, Clone)]
//     struct MysteryGuy {
//         // bits that have been switched
//         xor_mask: u32,
//         // bits that are not set to 0
//         mask: u32,
//         // the number of bit this value has been right shifted
//         right_shifted: u32,
//     }

//     #[derive(Copy, Clone)]
//     enum Value {
//         Int(u32),
//         MysteryGuy(MysteryGuy),
//     }

//     #[derive(Clone)]
//     struct Registers {
//         a: Value,
//         b: Value,
//         c: Value,
//     }

//     #[derive(Clone)]
//     struct VM<'a> {
//         registers: Registers,
//         instructions: &'a [u8],
//         pc: usize,
//         output: Vec<Value>,
//         // I don't know if it's useful
//         are_not_zero: Vec<Value>,
//     }

//     impl VM<'_> {
//         fn combo_operand(&self, operand: u8) -> Value {
//             match operand {
//                 0..=3 => Value::Int(u32::from(operand)),
//                 4 => self.registers.a,
//                 5 => self.registers.b,
//                 6 => self.registers.c,
//                 _ => unreachable!(),
//             }
//         }
//     }

//     impl MysteryGuy {
//         fn new() -> Self {
//             Self {
//                 xor_mask: 0,
//                 mask: u32::MAX,
//                 right_shifted: 0,
//             }
//         }

//         fn with_xor_mask(self, mask: u32) -> Self {
//             Self {
//                 xor_mask: self.xor_mask ^ (mask << self.right_shifted),
//                 ..self
//             }
//         }

//         fn mod_8(self) -> Self {
//             Self {
//                 mask: self.mask & (0b111 << self.right_shifted),
//                 ..self
//             }
//         }

//         fn right_shift(self, amount: u32) -> Self {
//             Self {
//                 right_shifted: self.right_shifted + amount,
//                 ..self
//             }
//         }
//     }

//     impl Value {
//         fn xor(self, rhs: Value) -> Value {
//             match (self, rhs) {
//                 (Value::Int(lhs), Value::Int(rhs)) => Value::Int(lhs ^ rhs),
//                 (Value::Int(mask), Value::MysteryGuy(mystery_guy))
//                 | (Value::MysteryGuy(mystery_guy), Value::Int(mask)) => {
//                     Value::MysteryGuy(mystery_guy.with_xor_mask(mask))
//                 }
//                 (Value::MysteryGuy(_), Value::MysteryGuy(_)) => todo!("mg xor mg"),
//             }
//         }

//         fn mod_8(self) -> Value {
//             match self {
//                 Value::Int(int) => Value::Int(int % 8),
//                 Value::MysteryGuy(mystery_guy) => Value::MysteryGuy(mystery_guy.mod_8()),
//             }
//         }

//         fn right_shift(self, rhs: Value) -> Value {
//             match (self, rhs) {
//                 (Value::Int(lhs), Value::Int(rhs)) => Value::Int(lhs >> rhs),

//                 (Value::MysteryGuy(mystery_guy), Value::Int(amount)) => {
//                     Value::MysteryGuy(mystery_guy.right_shift(amount))
//                 }
//                 (Value::Int(_), Value::MysteryGuy(_)) => {
//                     todo!("int >> mg");
//                 }
//                 (Value::MysteryGuy(_), Value::MysteryGuy(_)) => todo!("mg >> mg"),
//             }
//         }
//     }
// }

// pub fn part_two(input: &str) -> Option<u64> {
//     let Input {
//         registers: [_, b, c],
//         instructions,
//     } = parse(input);

//     let mut halted = Vec::new();
//     let mut alternatives = Vec::new();
//     alternatives.push(VM {
//         registers: Registers {
//             a: Value::A,
//             b: Value::Known(b),
//             c: Value::Known(c),
//         },
//         instructions: &instructions,
//         pc: 0,
//         output: Vec::new(),
//         are_not_zero: Vec::new(),
//     });

//     while let Some(mut vm) = alternatives.pop() {
//         loop {
//             let Some(&operand) = vm.instructions.get(vm.pc + 1) else {
//                 break;
//             };
//             let Some(&opcode) = vm.instructions.get(vm.pc) else {
//                 break;
//             };

//             match opcode {
//                 0 => {
//                     let num = vm.registers.a.clone();
//                     let denom_pow = vm.combo_operand(operand);
//                     let result = Value::right_shift(num, denom_pow);
//                     vm.registers.a = result;
//                 }
//                 1 => {
//                     let b = vm.registers.b.clone();
//                     let rhs = Value::Known(u32::from(operand));
//                     let result = Value::xor(b, rhs);
//                     vm.registers.b = result;
//                 }
//                 2 => {
//                     let value = vm.combo_operand(operand);
//                     let result = Value::mod_8(value);
//                     vm.registers.b = result;
//                 }
//                 3 => {
//                     {
//                         // we crate a new alternative in which register A is 0
//                         // and the jump didn't occurs.
//                         let mut alternative = vm.clone();
//                         alternative.registers.a = Value::Known(0);
//                         alternative.pc += 2;
//                         alternatives.push(alternative);
//                     }

//                     // In this reality register A is not 0
//                     vm.are_not_zero.push(vm.registers.a.clone());
//                     vm.pc = usize::from(operand);
//                     continue; // skip `pc` increment
//                 }
//                 4 => {
//                     let b = vm.registers.b.clone();
//                     let c = vm.registers.c.clone();
//                     let result = Value::xor(b, c);
//                     vm.registers.b = result;
//                 }
//                 5 => {
//                     let result = vm.combo_operand(operand).mod_8();
//                     vm.output.push(result);

//                     // We have already too much output
//                     if vm.output.len() > instructions.len() {
//                         break;
//                     }
//                 }
//                 6 => {
//                     let num = vm.registers.a.clone();
//                     let denom_pow = vm.combo_operand(operand);
//                     let result = Value::right_shift(num, denom_pow);
//                     vm.registers.b = result;
//                 }
//                 7 => {
//                     let num = vm.registers.a.clone();
//                     let denom_pow = vm.combo_operand(operand);
//                     let result = Value::right_shift(num, denom_pow);
//                     vm.registers.c = result;
//                 }
//                 _ => unreachable!(),
//             }

//             vm.pc += 2;
//         }

//         // We keep the vm only if it's output may be the program.
//         if vm.output.len() == instructions.len() {
//             halted.push(vm);
//         }
//     }

//     assert!(halted.len() == 1);

//     let [vm] = halted.try_into().ok().unwrap();

//     let to_resolve = instructions
//         .iter()
//         .copied()
//         .map(Word::new)
//         .map(Option::unwrap)
//         .zip(&vm.output);

//     for (Word(expected), value) in to_resolve.clone() {
//         println!("{expected} = {value}\n");
//     }

//     let result = resolve(to_resolve);

//     return Some(result);

//     #[derive(Debug, Clone)]
//     enum Value {
//         /// The value of the register A at the start of the program.
//         /// This is the value we're look for.
//         A,
//         Known(u32),
//         Expr(Rc<Expr>),
//     }

//     #[derive(Debug, Clone)]
//     enum Expr {
//         RightShift { lhs: Value, rhs: Value },
//         Xor { lhs: Value, rhs: Value },
//         Mod8 { value: Value },
//     }

//     #[derive(Clone)]
//     struct Registers {
//         a: Value,
//         b: Value,
//         c: Value,
//     }

//     #[derive(Clone)]
//     struct VM<'a> {
//         registers: Registers,
//         instructions: &'a [u8],
//         pc: usize,
//         output: Vec<Value>,
//         // I don't know if it's useful
//         are_not_zero: Vec<Value>,
//     }

//     impl VM<'_> {
//         fn combo_operand(&self, operand: u8) -> Value {
//             match operand {
//                 0..=3 => Value::Known(u32::from(operand)),
//                 4 => self.registers.a.clone(),
//                 5 => self.registers.b.clone(),
//                 6 => self.registers.c.clone(),
//                 _ => unreachable!(),
//             }
//         }
//     }

//     impl Expr {
//         fn is_rshift_with_known_rhs(&self) -> bool {
//             match self {
//                 Expr::RightShift { rhs, .. } => matches!(rhs, Value::Known(_)),
//                 Expr::Xor { .. } | Expr::Mod8 { .. } => false,
//             }
//         }
//     }

//     impl Value {
//         fn right_shift(self, rhs: Value) -> Value {
//             match (self, rhs) {
//                 (Value::Known(lhs), Value::Known(rhs)) => Value::Known(lhs >> rhs),
//                 // (x >> a >> b) = x >> (a + b)
//                 (Value::Expr(lhs), Value::Known(outer_rhs)) if lhs.is_rshift_with_known_rhs() => {
//                     match &*lhs {
//                         Expr::RightShift {
//                             lhs,
//                             rhs: Value::Known(inner_rhs),
//                         } => Value::Expr(Rc::new(Expr::RightShift {
//                             lhs: lhs.clone(),
//                             rhs: Value::Known(inner_rhs + outer_rhs),
//                         })),
//                         _ => unreachable!(),
//                     }
//                 }
//                 (lhs, rhs) => Value::Expr(Rc::new(Expr::RightShift { lhs, rhs })),
//             }
//         }

//         fn xor(self, rhs: Value) -> Value {
//             match (self, rhs) {
//                 (Value::Known(lhs), Value::Known(rhs)) => Value::Known(lhs ^ rhs),
//                 // (a ^ b) ^ c = a ^ (b ^ c)
//                 (Value::Known(known), Value::Expr(the_xor))
//                 | (Value::Expr(the_xor), Value::Known(known))
//                     if matches!(&*the_xor, Expr::Xor { .. }) =>
//                 {
//                     match &*the_xor {
//                         Expr::Xor {
//                             lhs: Value::Known(inner),
//                             rhs: value,
//                         }
//                         | Expr::Xor {
//                             lhs: value,
//                             rhs: Value::Known(inner),
//                         } => Value::Expr(Rc::new(Expr::Xor {
//                             lhs: value.clone(),
//                             rhs: Value::Known(known ^ inner),
//                         })),

//                         Expr::Xor { .. } => Value::Expr(Rc::new(Expr::Xor {
//                             lhs: Value::Expr(the_xor),
//                             rhs: Value::Known(known),
//                         })),

//                         _ => unreachable!(),
//                     }
//                 }

//                 (lhs, rhs) => Value::Expr(Rc::new(Expr::Xor { lhs, rhs })),
//             }
//         }

//         fn mod_8(self) -> Value {
//             match self {
//                 Value::Known(this) => Value::Known(this % 8),
//                 // ((X % 8) % 8) = (X % 8)
//                 Value::Expr(this) if matches!(&*this, Expr::Mod8 { .. }) => match &*this {
//                     Expr::Mod8 { .. } => Value::Expr(this),
//                     _ => unreachable!(),
//                 },
//                 value => Value::Expr(Rc::new(Expr::Mod8 { value })),
//             }
//         }
//     }

//     impl fmt::Display for Value {
//         fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//             match self {
//                 Value::A => f.write_str("A"),
//                 Value::Known(value) => fmt::Display::fmt(value, f),
//                 Value::Expr(expr) => fmt::Display::fmt(expr, f),
//             }
//         }
//     }

//     impl fmt::Display for Expr {
//         fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//             match self {
//                 Expr::RightShift { lhs, rhs } => write!(f, "({lhs} >> {rhs})"),
//                 Expr::Xor { lhs, rhs } => write!(f, "({lhs} ^ {rhs})"),
//                 Expr::Mod8 { value } => write!(f, "({value} % 8)"),
//             }
//         }
//     }

//     #[derive(Clone, Copy)]
//     struct Word(u8);

//     impl Word {
//         fn new(x: u8) -> Option<Self> {
//             (x < 8).then_some(Word(x))
//         }

//         fn into_bits(self) -> [Bit; 3] {
//             let a = (self.0 >> 2 & 1) == 1;
//             let b = (self.0 >> 1 & 1) == 1;
//             let c = (self.0 & 1) == 1;

//             [a, b, c]
//         }
//     }

//     type Bit = bool;

//     fn resolve<'a>(to_resolve: impl IntoIterator<Item = (Word, &'a Value)>) -> u64 {
//         let mut to_resolve = Vec::from_iter(
//             to_resolve
//                 .into_iter()
//                 .map(|(word, value)| (word.into_bits().map(Some).to_vec(), value)),
//         );
//         let mut resolved = Vec::new();

//         while let Some((bits, value)) = to_resolve.pop() {
//             match value {
//                 Value::A => {
//                     resolved.push(bits);
//                 }
//                 Value::Known(_) => {
//                     unreachable!("shouldn't be reached, but if it does we have to check the values are coherent")
//                 }
//                 Value::Expr(expr) => match &**expr {
//                     Expr::RightShift { lhs, rhs } => match (lhs, rhs) {
//                         (lhs, Value::Known(rhs)) => {
//                             let rhs = *rhs;
//                             let mut bits = bits;
//                             bits.extend(iter::repeat_n(None, rhs.try_into().unwrap()));

//                             to_resolve.push((bits, lhs));
//                         }

//                         (Value::A, Value::A) => todo!("A >> A"),
//                         (Value::A, Value::Expr(_)) => todo!("A >> expr"),
//                         (Value::Known(_), Value::A) => todo!("n >> A"),
//                         (Value::Known(_), Value::Expr(_)) => todo!("n >> expr"),
//                         (Value::Expr(_), Value::A) => todo!("expr >> A"),
//                         (Value::Expr(_), Value::Expr(_)) => todo!("expr >> expr"),
//                     },
//                     Expr::Xor { lhs, rhs } => match (lhs, rhs) {
//                         (Value::Known(mask), value) | (value, Value::Known(mask)) => {
//                             let mask = *mask;
//                             let mut bits = bits;

//                             assert!(bits.len() < 32);

//                             (0..u32::BITS)
//                                 .map(|i| ((mask >> i) & 1) == 1)
//                                 .zip(bits.iter_mut().rev())
//                                 .for_each(|(mask, x)| {
//                                     if let Some(x) = x {
//                                         *x ^= mask;
//                                     }
//                                 });

//                             to_resolve.push((bits, value));
//                         }

//                         (Value::A, Value::A) => todo!("A xor A"),

//                         (Value::A, Value::Expr(_)) => todo!("A xor expr"),

//                         (Value::Expr(_), Value::A) => todo!("expr xor A"),

//                         (Value::Expr(_), Value::Expr(_)) => todo!("expr xor expr"),
//                     },
//                     Expr::Mod8 { value } => {
//                         to_resolve.push((bits[bits.len() - 3..].to_vec(), value));
//                     }
//                 },
//             }
//         }

//         for resolved in &resolved {
//             println!("{resolved:?}\n");
//         }

//         let resolved = resolved.into_iter().fold(Vec::new(), |acc, cur| {
//             acc.into_iter()
//                 .zip_longest(cur.into_iter().rev())
//                 .map(|x| match x {
//                     itertools::EitherOrBoth::Both(a, b) => match (a, b) {
//                         (Some(a), Some(b)) if a == b => Some(a),
//                         (Some(a), None) => Some(a),
//                         (None, Some(b)) => Some(b),
//                         (None, None) => None,
//                         (Some(_), Some(_)) => panic!(),
//                     },
//                     itertools::EitherOrBoth::Left(x) | itertools::EitherOrBoth::Right(x) => x,
//                 })
//                 .collect_vec()
//         });

//         println!("{resolved:?}\n");

//         let resolved = resolved
//             .iter()
//             .copied()
//             .map(|b| b.unwrap_or(false))
//             .enumerate()
//             .fold(0, |acc, (i, cur)| if cur { acc | (1 << i) } else { acc });

//         resolved
//     }
// }

/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result.as_deref(), Some("4,6,3,5,6,3,5,2,1,0"));
    }

    #[test]
    fn test_part_two_check_output() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 21,
        ));
        assert_eq!(result.as_deref(), Some("0,3,5,4,3,0"));
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file_part(
    //         "examples", DAY, 2,
    //     ));

    //     assert_eq!(result, Some(117440));
    // }
}
