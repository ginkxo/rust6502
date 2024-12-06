#[cfg(test)]
mod tests {
    use super::*;

    use rust6502::mos;
    use rust6502::mos::Opcodes;
    use std::process;

    #[test]
    fn setup_debug_autotest() {
        
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
        println!("value test (LDA_IMMEDIATE 0xA9): {:#02x}", mos::CPU::LDA_IMMEDIATE);
        println!("program counter: {:#04x}", cpu.pc);
        println!("stack pointer: {:#04x}", cpu.sp);
        println!("reg A: {}", cpu.r_a);
        
        assert_eq!(1, 1);
    }

    #[test]
    fn mem_test_basic() {
        let mut cpu = mos::build_cpu();
        let mut mem = mos::build_memory();
        mem.memory[0x0002] = 0x0F;
        assert_eq!(mem.memory[0x0002], 0x0F);
    }

    #[test]
    fn cpu_init_pc() {
        let mut cpu = mos::build_cpu();
        assert_eq!(cpu.pc, 0xFFFC);
    }

    #[test]
    fn cpu_init_sp() {
        let mut cpu = mos::build_cpu();
        assert_eq!(cpu.sp, 0x00FF);
    }

    #[test]
    fn LDA_immediate() {
        let mut cpu = mos::build_cpu();
        let mut mem = mos::build_memory();
        mem.memory[0xFFFC] = mos::CPU::LDA_IMMEDIATE;
        mem.memory[0xFFFD] = 0x84;
        let cycles_init = 2;
        let cycles = cpu.execute(cycles_init, &mem).unwrap_or_else( |err| {
            println!("invalid memory and instruction situation {} ...", err);
            process::exit(1);
        });
        let good_cycles = cycles_init == cycles;
        let good_result = cpu.r_a == (0x84);
        assert!(good_result && good_cycles);
    }

    #[test]
    fn LDA_zero_page() {
        let mut cpu = mos::build_cpu();
        let mut mem = mos::build_memory();
        mem.memory[0xFFFC] = mos::CPU::LDA_ZERO_PAGE;
        mem.memory[0xFFFD] = 0x42;
        mem.memory[0x0042] = 0x84;
        let cycles_init = 3;
        let cycles = cpu.execute(cycles_init, &mem).unwrap_or_else( |err| {
            println!("invalid memory and instruction situation {} ...", err);
            process::exit(1);
        });
        let good_cycles = cycles_init == cycles;
        let good_result = cpu.r_a == (0x84);
        assert!(good_result && good_cycles);
    }

    #[test]
    fn LDA_zero_page_X() {
        let mut cpu = mos::build_cpu();
        let mut mem = mos::build_memory();
        cpu.r_x = 4;
        mem.memory[0xFFFC] = mos::CPU::LDA_ZERO_PAGE_X;
        mem.memory[0xFFFD] = 0x42; // 0x0042 + 0x0004 = 0x0046
        mem.memory[0x0046] = 0x84;
        let cycles_init = 4;
        let cycles = cpu.execute(cycles_init, &mem).unwrap_or_else( |err| {
            println!("invalid memory and instruction situation {} ...", err);
            process::exit(1);
        });
        let good_cycles = cycles_init == cycles;
        let good_result = cpu.r_a == (0x84);
        assert!(good_result && good_cycles);
    }

    #[test]
    fn LDA_zero_page_X_wraparound() {
        let mut cpu = mos::build_cpu();
        let mut mem = mos::build_memory();
        cpu.r_x = 0x00FF;
        mem.memory[0xFFFC] = mos::CPU::LDA_ZERO_PAGE_X;
        mem.memory[0xFFFD] = 0x0080; // 0x0042 + 0x0004 = 0x0046
        mem.memory[0x007F] = 0x0084;
        let cycles_init = 4;
        let cycles = cpu.execute(cycles_init, &mem).unwrap_or_else( |err| {
            println!("invalid memory and instruction situation {} ...", err);
            process::exit(1);
        });
        let good_cycles = cycles_init == cycles;
        let good_result = cpu.r_a == (0x84);
        assert!(good_result && good_cycles);
    }

    #[test]
    fn LDA_absolute() {
        let mut cpu = mos::build_cpu();
        let mut mem = mos::build_memory();
        mem.memory[0xFFFC] = mos::CPU::LDA_ABSOLUTE;
        mem.memory[0xFFFD] = 0x80; // 0x0042 + 0x0004 = 0x0046
        mem.memory[0xFFFE] = 0x44;
        mem.memory[0x4480] = 0x84;
        let cycles_init = 4;
        let cycles = cpu.execute(cycles_init, &mem).unwrap_or_else( |err| {
            println!("invalid memory and instruction situation {} ...", err);
            process::exit(1);
        });
        let good_cycles = cycles_init == cycles;
        let good_result = cpu.r_a == (0x84);
        assert!(good_result && good_cycles);
    }

    #[test]
    fn LDA_absolute_X() {
        let mut cpu = mos::build_cpu();
        let mut mem = mos::build_memory();
        cpu.r_x = 1;
        mem.memory[0xFFFC] = mos::CPU::LDA_ABSOLUTE_X;
        mem.memory[0xFFFD] = 0x02; // 0x0042 + 0x0004 = 0x0046
        mem.memory[0xFFFE] = 0x44;
        mem.memory[0x4403] = 0x84;
        let cycles_init = 4;
        let cycles = cpu.execute(cycles_init, &mem).unwrap_or_else( |err| {
            println!("invalid memory and instruction situation {} ...", err);
            process::exit(1);
        });
        let good_cycles = cycles_init == cycles;
        let good_result = cpu.r_a == (0x84);
        assert!(good_result && good_cycles);
    }

    #[test]
    fn LDA_absolute_X_cross_page() {
        let mut cpu = mos::build_cpu();
        let mut mem = mos::build_memory();
        cpu.r_x = 0xFF;
        mem.memory[0xFFFC] = mos::CPU::LDA_ABSOLUTE_X;
        mem.memory[0xFFFD] = 0x02; // 0x0042 + 0x0004 = 0x0046
        mem.memory[0xFFFE] = 0x44;
        mem.memory[0x4501] = 0x84;
        let cycles_init = 5;
        let cycles = cpu.execute(cycles_init, &mem).unwrap_or_else( |err| {
            println!("invalid memory and instruction situation {} ...", err);
            process::exit(1);
        });
        let good_cycles = cycles_init == cycles;
        let good_result = cpu.r_a == (0x84);
        assert!(good_result && good_cycles);
    }

    #[test]
    fn LDA_absolute_Y() {
        let mut cpu = mos::build_cpu();
        let mut mem = mos::build_memory();
        cpu.r_y = 1;
        mem.memory[0xFFFC] = mos::CPU::LDA_ABSOLUTE_Y;
        mem.memory[0xFFFD] = 0x02; // 0x0042 + 0x0004 = 0x0046
        mem.memory[0xFFFE] = 0x44;
        mem.memory[0x4403] = 0x84;
        let cycles_init = 4;
        let cycles = cpu.execute(cycles_init, &mem).unwrap_or_else( |err| {
            println!("invalid memory and instruction situation {} ...", err);
            process::exit(1);
        });
        let good_cycles = cycles_init == cycles;
        let good_result = cpu.r_a == (0x84);
        assert!(good_result && good_cycles);
    }

    #[test]
    fn LDA_absolute_Y_cross_page() {
        let mut cpu = mos::build_cpu();
        let mut mem = mos::build_memory();
        cpu.r_y = 0xFF;
        mem.memory[0xFFFC] = mos::CPU::LDA_ABSOLUTE_Y;
        mem.memory[0xFFFD] = 0x02; // 0x0042 + 0x0004 = 0x0046
        mem.memory[0xFFFE] = 0x44;
        mem.memory[0x4501] = 0x84;
        let cycles_init = 5;
        let cycles = cpu.execute(cycles_init, &mem).unwrap_or_else( |err| {
            println!("invalid memory and instruction situation {} ...", err);
            process::exit(1);
        });
        let good_cycles = cycles_init == cycles;
        let good_result = cpu.r_a == (0x84);
        assert!(good_result && good_cycles);
    }

    

}