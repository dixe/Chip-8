

//struct Registers =



enum Instruction
{
    Clear,
    Ret,
    Jump {address: u16},
    Call {address: u16},
    SkipEq {val: u8, vx: u8},
    SkipNeq {val: u8},
    SkipEqReg {val: u8, vy: u8, vx: u8},
    SetC {val: u8, vx: u8}, // VX = val
    AddC {val: u8, vx: u8},
    SetR {vx: u8, vy: u8}, // Vx = reg[vy]
    OrR {vx: u8, vy: u8},
    AndR {vx: u8, vy: u8},
    XorR {vx: u8, vy: u8},
    AddR {vx: u8, vy: u8}, // Remeber to set carry VF
    SubR {vx: u8, vy: u8}, // Remeber to set carry VF
    ShiftR{vx: u8}, // VF is set to least sig bit before shift
    SubSwitch{ vx: u8, vy: u8}, // VF 0 when borrow and 1 if not
    ShiftL{ vx: u8}, // vf set to most sig bit before shift
    NOP
}

struct Chip8 {
    registers: [u8; 16],
    ireg: u16,
    memory: [u8; 4096],
    pc: u16,

}




fn main() {

    //load instructions,


    let mut instructions = &[Instruction::AddC {val: 10, vx:0},
                             Instruction::SkipEq {val: 150, vx:0},
                             Instruction::Jump {address: 0}];

    let mut prog  = Chip8 { pc: 0,
                            ireg: 0,
                            memory: [0; 4096],
                            registers: [0; 16]
                           };
    run(&mut prog, instructions)
}



fn run(chip: &mut Chip8, instructions: &[Instruction]){

    let inst = &instructions[chip.pc as usize];

    let progLen  = instructions.len();

    while (chip.pc as usize) < progLen {
        let inst = &instructions[chip.pc as usize];
        execute(chip,inst);
        println!("reg0 = {0}", chip.registers[0]);

    }

    println!("Program finished exiting");
}


fn execute(chip: &mut Chip8, inst: &Instruction){

    let mut inc_pc = true;
    match inst {
        &Instruction::AddC {val,  vx} =>{
                chip.registers[vx as usize] += val;
                //TODO check for overflow
            },
        &Instruction::Jump {address} =>{
                chip.pc = address;
                inc_pc = false;
            },
        &Instruction::SkipEq {val, vx} =>{
            if grv(chip, vx) == val{
                chip.pc +=2;
                inc_pc = false;
            }
        }


        _ => println!("Not implemented operation")

    }


    if inc_pc{
        chip.pc +=1;
    }
}



fn grv(chip: &mut Chip8, reg: u8) -> u8{
    return chip.registers[reg as usize];
}
