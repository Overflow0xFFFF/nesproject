/**
 * Library for emulating a 6502 CPU.
 */

#[cfg(test)]
#[path = "cpu_test.rs"]
mod cpu_test;

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub status: u8,
    pub program_counter: u16,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            status: 0,
            program_counter: 0,
        }
    }

    /**
     * Interpret the program instructions.
     */
    pub fn interpret(&mut self, program: Vec<u8>) {
        self.program_counter = 0;

        loop {
            let opcode = program[self.program_counter as usize];
            self.program_counter += 1;

            match opcode {
                // LDA <param>
                0xA9 => {
                    let param = program[self.program_counter as usize];
                    self.program_counter += 1;
                    self.lda(param);
                }

                // TAX
                0xAA => {
                    self.tax();
                }

                // BRK
                0x00 => {
                    return;
                }
                _ => todo!(),
            }
        }
    }

    /**
     * 6502 Load Accumulator
     *
     * Load a byte of memory into the accumulator setting the zero and
     * negative flags as appropriate.
     */
    fn lda(&mut self, value: u8) {
        self.register_a = value;
        self.set_cpu_status_flags(self.register_a);
    }

    /**
     * 6502 Transfer Accumulator to X
     *
     * Copies the current contents of the accumulator into the X register and
     * sets the zero and negative flags as appropriate.
     */
    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.set_cpu_status_flags(self.register_x);
    }

    /**
     * Set the CPU status flags based on the value of the register passed.
     */
    fn set_cpu_status_flags(&mut self, result: u8) {
        if result == 0 {
            self.status = self.status | 0b0000_0010;
        } else {
            self.status = self.status & 0b1111_1101;
        }

        if result & 0b1000_0000 != 0 {
            self.status = self.status | 0b1000_0000;
        } else {
            self.status = self.status & 0b0111_1111;
        }
    }
}
