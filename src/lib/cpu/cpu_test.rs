/**
 * Unit tests for the CPU implementation.
 */
use super::*;

#[test]
fn test_0xa9_lda_immediate_load_data() {
    let mut cpu = CPU::new();
    cpu.interpret(vec![0xA9, 0x05, 0x00]);
    assert_eq!(cpu.register_a, 0x05);
    assert!(cpu.status & 0b0000_0010 == 0b00);
    assert!(cpu.status & 0b1000_0000 == 0);
}

#[test]
fn test_0xa9_lda_zero_flag() {
    let mut cpu = CPU::new();
    cpu.interpret(vec![0xA9, 0x00, 0x00]);
    assert!(cpu.status & 0b0000_0010 == 0b10);
}

#[test]
fn test_0xaa_tax_move_a_to_x() {
    let mut cpu = CPU::new();
    cpu.register_a = 10;
    cpu.interpret(vec![0xAA, 0x00]);
    assert_eq!(cpu.register_x, 10);
}

#[test]
fn test_0xe8_inx_increments_x() {
    let mut cpu = CPU::new();
    cpu.register_x = 10;
    cpu.interpret(vec![0xE8, 0x00]);
    assert_eq!(cpu.register_x, 11);
}

#[test]
fn test_inx_overflow() {
    let mut cpu = CPU::new();
    cpu.register_x = 0xff;
    cpu.interpret(vec![0xe8, 0xe8, 0x00]);
    assert_eq!(cpu.register_x, 1)
}

#[test]
fn test_5_ops_working_together() {
    let mut cpu = CPU::new();
    cpu.interpret(vec![0xA9, 0xC0, 0xAA, 0xE8, 0x00]);
    assert_eq!(cpu.register_x, 0xC1);
}
