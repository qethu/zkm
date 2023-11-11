use std::borrow::{Borrow, BorrowMut};
use std::mem::{size_of, transmute};
use std::ops::{Deref, DerefMut};

use crate::util::transmute_no_compile_time_size_checks;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct OpsColumnsView<T: Copy> {
    pub binary_op: T,     // Combines ADD, MUL, SUB, DIV, SLL, ... flags.
    pub binary_imm_op: T, // Combines ADDI, ADDIU
    pub eq_iszero: T,     // Combines EQ and ISZERO flags.
    pub logic_op: T,      // Combines AND, OR and XOR flags.
    pub condmov_op: T,
    pub count_op: T,
    pub not: T,
    pub shift: T, // Combines SHL and SHR flags.
    pub keccak_general: T,
    pub prover_input: T,
    pub jumps: T, // Combines JUMP and JUMPI flags.
    pub branch: T,
    pub pc: T,
    pub get_context: T,
    pub set_context: T,
    pub exit_kernel: T,
    pub m_op_general: T,

    pub syscall: T,
}

// `u8` is guaranteed to have a `size_of` of 1.
pub const NUM_OPS_COLUMNS: usize = size_of::<OpsColumnsView<u8>>();

impl<T: Copy> From<[T; NUM_OPS_COLUMNS]> for OpsColumnsView<T> {
    fn from(value: [T; NUM_OPS_COLUMNS]) -> Self {
        unsafe { transmute_no_compile_time_size_checks(value) }
    }
}

impl<T: Copy> From<OpsColumnsView<T>> for [T; NUM_OPS_COLUMNS] {
    fn from(value: OpsColumnsView<T>) -> Self {
        unsafe { transmute_no_compile_time_size_checks(value) }
    }
}

impl<T: Copy> Borrow<OpsColumnsView<T>> for [T; NUM_OPS_COLUMNS] {
    fn borrow(&self) -> &OpsColumnsView<T> {
        unsafe { transmute(self) }
    }
}

impl<T: Copy> BorrowMut<OpsColumnsView<T>> for [T; NUM_OPS_COLUMNS] {
    fn borrow_mut(&mut self) -> &mut OpsColumnsView<T> {
        unsafe { transmute(self) }
    }
}

impl<T: Copy> Deref for OpsColumnsView<T> {
    type Target = [T; NUM_OPS_COLUMNS];
    fn deref(&self) -> &Self::Target {
        unsafe { transmute(self) }
    }
}

impl<T: Copy> DerefMut for OpsColumnsView<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { transmute(self) }
    }
}