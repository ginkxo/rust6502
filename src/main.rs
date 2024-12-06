pub mod mos;
use crate::mos::CPU;
use crate::mos::Opcodes;
use std::process;

fn main() {
    
    let mut cpu = mos::build_cpu();
    let mut mem = mos::build_memory();

    println!("before memory set ...");
    println!("[0x0001] : {:#04x}", mem.memory[0x0001]);
    println!("[0x0002] : {:#04x}", mem.memory[0x0002]);
    println!("[0x0003] : {:#04x}", mem.memory[0x0003]);

    mem.memory[0x0002] = 0x0F;

    println!("after memory set ...");
    println!("[0x0001] : {:#04x}", mem.memory[0x0001]);
    println!("[0x0002] : {:#04x}", mem.memory[0x0002]);
    println!("[0x0003] : {:#04x}", mem.memory[0x0003]);


    // basic cpu tests

    println!("CPU DEBUGS");

    println!("value test (LDA_IMMEDIATE 0xA9): {:#02x}", CPU::LDA_IMMEDIATE);

    println!("program counter: {:#04x}", cpu.pc);
    println!("stack pointer: {:#04x}", cpu.sp);
    println!("reg A: {}", cpu.r_a);
    println!("------ executing ...");

    // load reg immediate

    mem.memory[0xFFFC] = CPU::LDA_IMMEDIATE;
    mem.memory[0xFFFD] = 0x84;
    let cycles_init = 2;
    let cycles = cpu.execute(cycles_init, &mem).unwrap_or_else( |err| {
        println!("invalid memory and instruction situation {} ...", err);
        println!{"init cycles: {}", cycles_init};
        println!("program counter: {:#04x}", cpu.pc);
        println!("stack pointer: {:#04x}", cpu.sp);
        println!("reg A: {:#04x}", cpu.r_a);
        println!("memory stack: ");
        println!("[0xFFFB] : {:#04x}", mem.memory[0xFFFB]);
        println!("[0xFFFC] : {:#04x}", mem.memory[0xFFFC]);
        println!("[0xFFFD] : {:#04x}", mem.memory[0xFFFD]);
        println!("[0xFFFE] : {:#04x}", mem.memory[0xFFFE]);
        println!("[0xFFFF] : {:#04x}", mem.memory[0xFFFF]);
        process::exit(1);
    });

    println!("init cycles: {}", cycles_init);
    println!("cycles: {}", cycles);
    println!("program counter: {:#04x}", cpu.pc);
    println!("stack pointer: {:#04x}", cpu.sp);
    println!("reg A: {:#04x}", cpu.r_a);
    
}
