use std::error::Error;

// core types
type Byte = u8;                 // 1 byte: 0x00
type Opcode = Byte;             // 1 byte: 0x00
type Word = u16;                // 2 bytes: 0x0000

// u32 is unsigned int
// i32 is signed int

static MAX_MEM: u32 = 1024 * 64;

/* MEMORY */

pub struct MEMORY {
    pub memory: Vec<Byte>,
}

impl MEMORY {

    pub fn write_word(&mut self, value: Word, address: Word, mut cycles: i32) -> Result<i32, Box<dyn Error>> {
        let least_significant: Byte = (value & 0xFF) as Byte;
        let most_significant: Byte = (value >> 8) as Byte;
        self.memory[address as usize] = least_significant;
        self.memory[(address + 1) as usize] = most_significant;
        cycles -= 2; // write twice : 2 cycles
        Ok(cycles)
    }
}

pub fn build_memory() -> MEMORY {
    MEMORY {
        memory: vec![0; MAX_MEM as usize],
    }
}

/* CPU */

// as associated constant
pub trait Opcodes {

    // LDA
    const LDA_IMMEDIATE: Opcode;
    const LDA_ZERO_PAGE: Opcode;
    const LDA_ZERO_PAGE_X: Opcode;
    const LDA_ABSOLUTE: Opcode;
    const LDA_ABSOLUTE_X: Opcode;
    const LDA_ABSOLUTE_Y: Opcode;
    const LDA_INDIRECT_X: Opcode;
    const LDA_INDIRECT_Y: Opcode;
    // LDX
    const LDX_IMMEDIATE: Opcode;
    const LDX_ZERO_PAGE: Opcode;
    const LDX_ZERO_PAGE_Y: Opcode;
    const LDX_ABSOLUTE: Opcode;
    const LDX_ABSOLUTE_Y: Opcode;
    // LDY
    const LDY_IMMEDIATE: Opcode;
    const LDY_ZERO_PAGE: Opcode;
    const LDY_ZERO_PAGE_X: Opcode;
    const LDY_ABSOLUTE: Opcode;
    const LDY_ABSOLUTE_X: Opcode;
    // JSR
    const JSR_ABSOLUTE: Opcode;

}

pub struct CPU {

    // counter & pointer
    pub pc: Word,                   // program counter
    pub sp: Word,                   // stack pointer

    // registers
    pub r_a: Byte,                  // A register
    pub r_x: Byte,                  // X register
    pub r_y: Byte,                  // Y register

    // processor status bit fields
    pub ps_carry: Byte,             // carry bit
    pub ps_zero: Byte,              // zero bit
    pub ps_interrupt: Byte,         // interrupt bit
    pub ps_decimal: Byte,           // decimal mode bit
    pub ps_break: Byte,             // break command bit
    pub ps_overflow: Byte,          // overflow bit
    pub ps_negative: Byte,          // negative value bit

}

impl Opcodes for CPU {
    // LDA
    const LDA_IMMEDIATE: Opcode = 0xA9;
    const LDA_ZERO_PAGE: Opcode = 0xA5;
    const LDA_ZERO_PAGE_X: Opcode = 0xB5;
    const LDA_ABSOLUTE: Opcode = 0xAD;
    const LDA_ABSOLUTE_X: Opcode = 0xBD;
    const LDA_ABSOLUTE_Y: Opcode = 0xB9;
    const LDA_INDIRECT_X: Opcode = 0xA1;
    const LDA_INDIRECT_Y: Opcode = 0xB1;
    // LDX
    const LDX_IMMEDIATE: Opcode = 0xA2;
    const LDX_ZERO_PAGE: Opcode = 0xA6;
    const LDX_ZERO_PAGE_Y: Opcode = 0xB6;
    const LDX_ABSOLUTE: Opcode = 0xAE;
    const LDX_ABSOLUTE_Y: Opcode = 0xBE;
    // LDY
    const LDY_IMMEDIATE: Opcode = 0xA0;
    const LDY_ZERO_PAGE: Opcode = 0xA4;
    const LDY_ZERO_PAGE_X: Opcode = 0xB4;
    const LDY_ABSOLUTE: Opcode = 0xAC;
    const LDY_ABSOLUTE_X: Opcode = 0xBC;
    // JSR
    const JSR_ABSOLUTE: Opcode = 0x20;
}

impl CPU {

    fn reset_cpu(&mut self) {
        self.pc = 0xFFFC;
        self.sp = 0x00FF;
        self.r_a = 0;
        self.r_x = 0;
        self.r_y = 0;
        self.ps_carry = 0;
        self.ps_zero = 0;
        self.ps_interrupt = 0;
        self.ps_decimal = 0;
        self.ps_break = 0;
        self.ps_overflow = 0;
        self.ps_negative = 0;
    }

    fn fetch_byte(&mut self, mem: &MEMORY, mut cycles: i32) -> (Byte, i32) {
        let instruction: Byte = mem.memory[self.pc as usize];
        self.pc += 1;
        cycles -= 1;
        return (instruction, cycles)
    }

    fn read_byte(address: Word, mem: &MEMORY, mut cycles: i32) -> (Byte, i32){
        let byte: Byte = mem.memory[address as usize];
        cycles -= 1;
        return (byte, cycles)
    }

    fn read_byte_zero_page(address: Byte, mem: &MEMORY, mut cycles: i32) -> (Byte, i32){
        let byte: Byte = mem.memory[address as usize];
        cycles -= 1;
        return (byte, cycles)
    }

    fn read_word(address: Word, mem: &MEMORY, cycles: i32) -> (Word, i32) {
        let (lo_byte, cycles_min_one): (Byte, i32) = CPU::read_byte(address, mem, cycles);
        let (hi_byte, cycles_min_two): (Byte, i32) = CPU::read_byte(address + 1, mem, cycles_min_one);
        let lo_byte_word: Word = lo_byte.into();
        let hi_byte_word: Word = <u8 as Into<Word>>::into(hi_byte) << 8;
        let full_word: Word = lo_byte_word | hi_byte_word;
        return (full_word, cycles_min_two)
    }

    fn fetch_word(&mut self, mem: &MEMORY, mut cycles: i32) -> (Word, i32) {
        let lo_byte: Word = mem.memory[self.pc as usize].into();
        self.pc += 1; // goto next memory addr for hi byte
        let hi_byte: Word = <u8 as Into<Word>>::into(mem.memory[self.pc as usize]) << 8;
        self.pc += 1;
        cycles -= 2; // fetch twice: 2 cycles
        let full_word: Word = lo_byte | hi_byte;
        return (full_word, cycles)
    }

    pub fn execute(&mut self, mut cycles: i32, mem: &MEMORY) -> Result<i32, &'static str>{

        let requested_cycles: i32 = cycles;


        while cycles > 0 {

            let (instruction, mut cycles): (Opcode, i32) = CPU::fetch_byte(self, mem, cycles);
            
            match instruction {

                CPU::LDA_IMMEDIATE => {
                    let (value, mut cycles): (Byte, i32) = CPU::fetch_byte(self, mem, cycles);
                    self.r_a = value;
                    return Ok(requested_cycles - cycles);
                }
                CPU::LDA_ZERO_PAGE => {
                    let (zero_page_address, mut cycles): (Byte, i32) = CPU::fetch_byte(self, mem, cycles);
                    let (value, mut cycles): (Byte, i32) = CPU::read_byte_zero_page(zero_page_address, mem, cycles);
                    self.r_a = value;
                    return Ok(requested_cycles - cycles);
                }
                CPU::LDA_ZERO_PAGE_X => {
                    let (zero_page_address, mut cycles): (Byte, i32) = CPU::fetch_byte(self, mem, cycles);
                    let zero_page_addr_x_word: Word = (zero_page_address as Word) + (self.r_x as Word);
                    let zero_page_addr_x: Byte = zero_page_addr_x_word as Byte;
                    cycles -= 1;
                    let (value, mut cycles): (Byte, i32) = CPU::read_byte_zero_page(zero_page_addr_x, mem, cycles);
                    self.r_a = value;
                    return Ok(requested_cycles - cycles);
                }
                CPU::LDA_ABSOLUTE => {
                    let (absolute_address, mut cycles): (Word, i32) = CPU::fetch_word(self, mem, cycles);
                    let (value, mut cycles): (Byte, i32) = CPU::read_byte(absolute_address, mem, cycles);
                    self.r_a = value;
                    return Ok(requested_cycles - cycles);
                }
                CPU::LDA_ABSOLUTE_X => {
                    let (absolute_address, mut cycles): (Word, i32) = CPU::fetch_word(self, mem, cycles);
                    let absolute_addr_x: Word = absolute_address + (self.r_x as Word);
                    let (value, mut cycles): (Byte, i32) = CPU::read_byte(absolute_addr_x, mem, cycles);
                    self.r_a = value;
                    if (absolute_addr_x - absolute_address) >= 0xFF {
                        cycles -= 1;
                    }
                    return Ok(requested_cycles - cycles);
                }
                CPU::LDA_ABSOLUTE_Y => {
                    let (absolute_address, mut cycles): (Word, i32) = CPU::fetch_word(self, mem, cycles);
                    let absolute_addr_y: Word = absolute_address + (self.r_y as Word);
                    let (value, mut cycles): (Byte, i32) = CPU::read_byte(absolute_addr_y, mem, cycles);
                    self.r_a = value;
                    if (absolute_addr_y - absolute_address) >= 0xFF {
                        cycles -= 1;
                    }
                    return Ok(requested_cycles - cycles);
                }
                CPU::LDA_INDIRECT_X => {
                    let (zero_page_address, mut cycles): (Byte, i32) = CPU::fetch_byte(self, mem, cycles);
                    let zero_page_addr_x: Word = (zero_page_address + self.r_x) as Word;
                    cycles -= 1;
                    let (effective_address, mut cycles): (Word, i32) = CPU::read_word(zero_page_addr_x, mem, cycles);
                    let (value, mut cycles): (Byte, i32) = CPU::read_byte(effective_address, mem, cycles);
                    self.r_a = value;
                    return Ok(requested_cycles - cycles);
                }
                CPU::LDA_INDIRECT_Y => {
                    let (zero_page_address, mut cycles): (Byte, i32) = CPU::fetch_byte(self, mem, cycles);
                    let (effective_address, mut cycles): (Word, i32) = CPU::read_word(zero_page_address as Word, mem, cycles);
                    let effective_addr_y = effective_address + (self.r_y as Word);
                    let (value, mut cycles): (Byte, i32) = CPU::read_byte(effective_addr_y, mem, cycles);
                    self.r_a = value;
                    if (effective_addr_y - effective_address) >= 0xFF {
                        cycles -= 1;
                    }
                    return Ok(requested_cycles - cycles);
                }
                CPU::JSR_ABSOLUTE => {
                    let (subroutine_addr, mut cycles): (Word, i32) = CPU::fetch_word(self, mem, cycles);
                    // finish this later
                    return Ok(requested_cycles - cycles);
                }
                CPU::LDX_IMMEDIATE => {
                    let (value, mut cycles): (Byte, i32) = CPU::fetch_byte(self, mem, cycles);
                    self.r_x = value;
                    return Ok(requested_cycles - cycles);
                }
                CPU::LDY_IMMEDIATE => {
                    let (value, mut cycles): (Byte, i32) = CPU::fetch_byte(self, mem, cycles);
                    self.r_y = value;
                    return Ok(requested_cycles - cycles);
                }
                _ => {
                    println!("bad instruction {} ...", instruction);
                    return Err("invalid instruction error");
                }
            }
        }

        return Ok(requested_cycles - cycles);

    }
}

pub fn build_cpu() -> CPU {
    CPU {
        pc: 0xFFFC,
        sp: 0x00FF,
        r_a: 0,
        r_x: 0,
        r_y: 0,
        ps_carry: 0,
        ps_zero: 0,
        ps_interrupt: 0,
        ps_decimal: 0,
        ps_break: 0,
        ps_overflow: 0,
        ps_negative: 0,
    }
}
