use anyhow::{anyhow, Error};
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    ADV,
    BXL,
    BST,
    JNZ,
    BXC,
    OUT,
    BDV,
    CDV,
}

impl TryFrom<i32> for Instruction {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Instruction::ADV),
            1 => Ok(Instruction::BXL),
            2 => Ok(Instruction::BST),
            3 => Ok(Instruction::JNZ),
            4 => Ok(Instruction::BXC),
            5 => Ok(Instruction::OUT),
            6 => Ok(Instruction::BDV),
            7 => Ok(Instruction::CDV),
            _ => Err("Invalid instruction value"),
        }
    }
}

#[derive(Debug)]
struct Registers {
    a: i32,
    b: i32,
    c: i32,
}

#[derive(Debug)]
struct Device {
    registers: Registers,
    stack: Vec<i32>,
    esp: usize,
}

impl Device {
    fn _operand_combo(self: &Self, operand: i32) -> i32 {
        if operand <= 3 {
            return operand;
        }
        match operand {
            4 => return self.registers.a,
            5 => return self.registers.b,
            6 => return self.registers.c,
            _ => panic!("invalid operand combo: {}", operand),
        }
    }

    fn run(self: &mut Self) -> Result<Vec<u8>, Error> {
        let mut results: Vec<u8> = vec![];
        loop {
            if self.esp >= self.stack.len() {
                break;
            }
            let instruction = Instruction::try_from(self.stack[self.esp as usize])
                .map_err(|e| anyhow!("failed to parse instruction: {}", e))?;

            let operand = self.stack[self.esp + 1 as usize];

            let mut esp_inc = true;
            match instruction {
                Instruction::ADV => {
                    // performs division. The numerator is the value in the A registers
                    // the denominator is found by raising 2 to the power of the instruction's combo operand
                    let numerator = self.registers.a;
                    let denominator = (2 as i32).pow(self._operand_combo(operand) as u32);
                    self.registers.a = numerator / denominator;
                }
                Instruction::BXL => {
                    // calculates the bitwise XOR of register B and the instruction's literal operand, then stores the result in register B
                    self.registers.b = self.registers.b ^ operand;
                }
                Instruction::BST => {
                    // instruction (opcode 2) calculates the value of its combo operand modulo 8 (thereby keeping only its lowest 3 bits),
                    // then writes that value to the B register
                    self.registers.b = self._operand_combo(operand) % 8;
                }
                Instruction::JNZ => {
                    // instruction (opcode 3) does nothing if the A register is 0. However, if the A register is not zero,
                    // it jumps by setting the instruction pointer to the value of its literal operand;
                    // if this instruction jumps, the instruction pointer is not increased by 2 after this instruction
                    if self.registers.a != 0 {
                        esp_inc = false;
                        self.esp = operand as usize;
                    }
                }
                Instruction::BXC => {
                    // instruction (opcode 4) calculates the bitwise XOR of register B and register C,
                    // then stores the result in register B
                    self.registers.b = self.registers.b ^ self.registers.c;
                }
                Instruction::OUT => {
                    // instruction (opcode 5) calculates the value of its combo operand modulo 8,
                    // then outputs that value
                    let result = self._operand_combo(operand) % 8;
                    results.push(result as u8);
                }
                Instruction::BDV => {
                    // instruction (opcode 6) works exactly like the adv instruction except that the result is stored in the B register
                    let numerator = self.registers.a;
                    let denominator = (2 as i32).pow(self._operand_combo(operand) as u32);
                    self.registers.b = numerator / denominator;
                }
                Instruction::CDV => {
                    // instruction (opcode 7) works exactly like the adv instruction except that the result is stored in the C register.
                    let numerator = self.registers.a;
                    let denominator = (2 as i32).pow(self._operand_combo(operand) as u32);
                    self.registers.c = numerator / denominator;
                }
            }

            if esp_inc {
                self.esp = self.esp + 2;
            }
        }

        return Ok(results);
    }
}

fn parse_device_definition(file_path: &str) -> Result<Device, anyhow::Error> {
    let input = File::open(file_path)?;
    let reader = io::BufReader::new(input);

    let mut register_a = 0;
    let mut register_b = 0;
    let mut register_c = 0;
    let mut stack: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.starts_with("Register A:") {
            register_a = line.split_whitespace().nth(2).unwrap().parse().unwrap();
        } else if line.starts_with("Register B:") {
            register_b = line.split_whitespace().nth(2).unwrap().parse().unwrap();
        } else if line.starts_with("Register C:") {
            register_c = line.split_whitespace().nth(2).unwrap().parse().unwrap();
        } else if line.starts_with("Program:") {
            let stack_str = line.split(':').nth(1).unwrap().trim();
            stack = stack_str
                .split(',')
                .filter_map(|s| s.trim().parse::<i32>().ok()) // Convert to u8
                .collect();
        }
    }

    Ok(Device {
        registers: Registers {
            a: register_a,
            b: register_b,
            c: register_c,
        },
        stack,
        esp: 0,
    })
}

fn main() -> Result<(), Error> {
    let mut device = parse_device_definition("input.prod")?;
    let results = device.run()?;

    let result = results
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",");

    println!("{}", result);

    Ok(())
}
