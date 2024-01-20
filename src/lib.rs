mod macros;
mod op;
mod register;
mod vm;

mod memory;

pub mod prelude {
    pub use crate::write_memory;

    pub use crate::op::*;
    pub use crate::register::*;
    pub use crate::vm::*;
}

#[cfg(test)]
mod tests {
    use crate::{
        register::Register,
        vm::{Machine, MEMORY_KILO_BYTES},
        write_memory,
    };

    fn sig_halt(vm: &mut Machine) {
        vm.machine_halted = true;
    }

    // Tests for failure (these should fail!)
    #[test]
    fn unknown_instruction() {
        let mut machine = Machine::new();
        machine.memory.write(0, 0xF).unwrap();
        assert!(machine.step().is_err());
    }

    #[test]
    fn out_of_bounds() -> Result<(), Box<dyn std::error::Error>> {
        let mut machine = Machine::new();

        assert!(machine
            .memory
            .write(u16::try_from(MEMORY_KILO_BYTES * 1024 + 1)?, 0xf)
            .is_err());

        Ok(())
    }

    #[test]
    fn addition() -> Result<(), Box<dyn std::error::Error>> {
        let mut machine = Machine::new();
        machine.define_handler(0xf0, sig_halt);

        write_memory!(machine,
         0 => 0x01,
         1 => 0x0a,
         2 => 0x01,
         3 => 0x08,
         4 => 0x10,
         5 => 0x00,
         6 => 0x02,
         7 => 0x00,
         8 => 0x0f,
         9 => 0xf0
        );

        machine.step().unwrap(); // PUSH 10
        machine.step().unwrap(); // PUSH 8
        machine.step().unwrap(); // ADDSTACK
        machine.step().unwrap(); // POPREGISTER A
        machine.step().unwrap(); // SIGNAL 0xF0

        assert_eq!(machine.get_register(Register::A), 18);

        Ok(())
    }

    #[test]
    fn subsequent_addition() -> Result<(), Box<dyn std::error::Error>> {
        let mut machine = Machine::new();
        machine.define_handler(0xf0, sig_halt);

        for _ in 1..=2 {
            write_memory!(machine,
             0 => 0x01,
             1 => 0x0a,
             2 => 0x01,
             3 => 0x08,
             4 => 0x10,
             5 => 0x00,
             6 => 0x02,
             7 => 0x00,
             8 => 0x0f,
             9 => 0xf0
            );

            machine.step().unwrap(); // PUSH 10
            machine.step().unwrap(); // PUSH 8
            machine.step().unwrap(); // ADDSTACK
            machine.step().unwrap(); // POPREGISTER A
            machine.step().unwrap(); // SIGNAL 0xF0

            assert_eq!(machine.get_register(Register::A), 18);
        }

        Ok(())
    }
}
