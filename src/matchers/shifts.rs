use inkwell::llvm_sys::core::{LLVMGetInstructionOpcode, LLVMGetOperand};
use inkwell::llvm_sys::prelude::LLVMValueRef;
use inkwell::llvm_sys::LLVMOpcode;

use super::InstMatcher;

#[derive(Clone)]
pub enum ShiftKind {
    LogicalShiftLeft,
    LogicalShiftRight,
    ArithmeticShiftRight,
}

impl ShiftKind {
    pub fn llvm_opcode(&self) -> LLVMOpcode {
        match self {
            ShiftKind::LogicalShiftLeft => LLVMOpcode::LLVMShl,
            ShiftKind::LogicalShiftRight => LLVMOpcode::LLVMLShr,
            ShiftKind::ArithmeticShiftRight => LLVMOpcode::LLVMAShr,
        }
    }
}

/// Int Shift Inst Matcher to check if instruction is Logical and Arithmetic shifts and match LHS, RHS and commutatively
#[derive(Clone)]
pub struct ShiftInstMatcher {
    pub lhs_matcher: Box<dyn InstMatcher>,
    pub rhs_matcher: Box<dyn InstMatcher>,
    pub shift_kind: ShiftKind,
    pub commutatively: bool,
}

impl ShiftInstMatcher {
    pub fn create_logical_shl(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(ShiftInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            shift_kind: ShiftKind::LogicalShiftLeft,
            commutatively: false,
        })
    }

    pub fn create_logical_shr(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(ShiftInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            shift_kind: ShiftKind::LogicalShiftRight,
            commutatively: false,
        })
    }

    pub fn create_arithmetic_shr(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(ShiftInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            shift_kind: ShiftKind::ArithmeticShiftRight,
            commutatively: false,
        })
    }

    pub fn create_commutatively_logical_shl(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(ShiftInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            shift_kind: ShiftKind::LogicalShiftLeft,
            commutatively: true,
        })
    }

    pub fn create_commutatively_logical_shr(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(ShiftInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            shift_kind: ShiftKind::LogicalShiftRight,
            commutatively: true,
        })
    }

    pub fn create_commutatively_arithmetic_shr(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(ShiftInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            shift_kind: ShiftKind::ArithmeticShiftRight,
            commutatively: true,
        })
    }
}

impl InstMatcher for ShiftInstMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: LLVMValueRef) -> bool {
        unsafe {
            let opcode = LLVMGetInstructionOpcode(instruction);
            if opcode == self.shift_kind.llvm_opcode() {
                let rhs = LLVMGetOperand(instruction, 1);
                let lhs = LLVMGetOperand(instruction, 0);

                if self.lhs_matcher.is_match(lhs) && self.rhs_matcher.is_match(rhs) {
                    return true;
                }

                if self.commutatively
                    && self.lhs_matcher.is_match(rhs)
                    && self.rhs_matcher.is_match(lhs)
                {
                    return true;
                }
            }
            false
        }
    }
}
