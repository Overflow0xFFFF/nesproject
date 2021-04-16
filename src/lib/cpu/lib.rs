/**
 * Library for emulating a 6502 CPU.
 */

#[cfg(test)]
#[path = "cpu_test.rs"]
mod cpu_test;

const NES_MAX_MEMORY: usize = 0xFFFF; // 64 KiB
const NES_ROM_PROGRAM_START: usize = 0x8000;

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub status: u8,
    pub program_counter: u16,
    memory: [u8; NES_MAX_MEMORY],
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            status: 0,
            program_counter: 0,
            memory: [0; NES_MAX_MEMORY],
        }
    }

    /**
     * Read a byte from memory.
     *
     * @param addr The address of memory from which to read.
     */
    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    /**
     * Read a word from memory.
     *
     * This function reads data from memory packed in little-endian format.
     *
     * @param pos Position in memory from which to read.
     * @return The word at that position.
     */
    fn mem_read_u16(&self, pos: u16) -> u16 {
        let lower = self.mem_read(pos);
        let upper = self.mem_read(pos + 1);
        u16::from_le_bytes([lower, upper])
    }

    /**
     * Write a byte to a location in memory.
     *
     * @param addr The address of memory to which to write.
     * @param data The byte to write to the address.
     */
    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    /**
     * Write a word to a location in memory.
     *
     * This function writes data to memory, packed in little-endian format.
     *
     * @param pos The position in memory to which to write.
     * @param data The word to write to the address.
     */
    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let bytes = data.to_le_bytes();
        let lower = bytes[0];
        let upper = bytes[1];
        self.mem_write(pos, lower);
        self.mem_write(pos + 1, upper);
    }

    /**
     * Run the program on the CPU.
     */
    pub fn run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.execute();
    }

    /**
     * Load program into memory.
     */
    pub fn load(&mut self, program: Vec<u8>) {
        let program_end = NES_ROM_PROGRAM_START + program.len();
        self.memory[NES_ROM_PROGRAM_START..program_end].copy_from_slice(&program[..]);

        self.mem_write_u16(0xFFFC, NES_ROM_PROGRAM_START as u16);
        self.program_counter = NES_ROM_PROGRAM_START as u16;
    }

    /**
     * Reset CPU registers and initialize program counter.
     */
    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.status = 0;
        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    /**
     * Execute the program from system memory.
     *
     * Requires that a program has been `load()`ed and that the CPU has
     * been `reset()` first.
     */
    pub fn execute(&mut self) {
        loop {
            let opcode = self.mem_read(self.program_counter);
            self.program_counter += 1;

            match opcode {
                // LDA <param>
                0xA9 => {
                    let param = self.mem_read(self.program_counter);
                    self.program_counter += 1;
                    self.lda(param);
                }

                // TAX
                0xAA => {
                    self.tax();
                }

                // INX
                0xE8 => {
                    self.inx();
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
     * 6502 Increment X Register
     *
     * Adds one to the X register setting the zero and negative flags as
     * appropriate.
     */
    fn inx(&mut self) {
        // Check for overflow
        if self.register_x == u8::max_value() {
            self.register_x = 0;
        } else {
            self.register_x += 1;
        }
        self.set_cpu_status_flags(self.register_x);
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
