use inkwell::llvm_sys::core::{LLVMGetInstructionOpcode, LLVMGetOperand};
use inkwell::llvm_sys::prelude::LLVMValueRef;
use inkwell::llvm_sys::LLVMOpcode;

use super::InstMatcher;

/// Arithmetic instruction matcher to check if instruction is arithmetic and match opcode, LHS, RHS and commutatively
#[derive(Clone)]
pub struct ArithmeticInstMatcher {
    pub lhs_matcher: Box<dyn InstMatcher>,
    pub rhs_matcher: Box<dyn InstMatcher>,
    pub opcode: LLVMOpcode,
    pub commutatively: bool,
}

impl ArithmeticInstMatcher {
    pub fn create_add(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(ArithmeticInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            opcode: LLVMOpcode::LLVMAdd,
            commutatively: false,
        })
    }

    pub fn create_sub(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(ArithmeticInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            opcode: LLVMOpcode::LLVMSub,
            commutatively: false,
        })
    }

    pub fn create_mul(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(ArithmeticInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            opcode: LLVMOpcode::LLVMMul,
            commutatively: false,
        })
    }

    pub fn create_div(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(ArithmeticInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            opcode: LLVMOpcode::LLVMSDiv,
            commutatively: false,
        })
    }

    pub fn create_rem(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(ArithmeticInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            opcode: LLVMOpcode::LLVMSRem,
            commutatively: false,
        })
    }

    pub fn create_commutatively_add(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(ArithmeticInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            opcode: LLVMOpcode::LLVMAdd,
            commutatively: true,
        })
    }

    pub fn create_commutatively_sub(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(ArithmeticInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            opcode: LLVMOpcode::LLVMSub,
            commutatively: true,
        })
    }

    pub fn create_commutatively_mul(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(ArithmeticInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            opcode: LLVMOpcode::LLVMMul,
            commutatively: true,
        })
    }

    pub fn create_commutatively_div(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(ArithmeticInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            opcode: LLVMOpcode::LLVMSDiv,
            commutatively: true,
        })
    }

    pub fn create_commutatively_rem(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(ArithmeticInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            opcode: LLVMOpcode::LLVMSRem,
            commutatively: true,
        })
    }

    pub fn create_float_add(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(ArithmeticInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            opcode: LLVMOpcode::LLVMFAdd,
            commutatively: false,
        })
    }

    pub fn create_float_sub(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(ArithmeticInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            opcode: LLVMOpcode::LLVMFSub,
            commutatively: false,
        })
    }

    pub fn create_float_mul(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(ArithmeticInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            opcode: LLVMOpcode::LLVMFMul,
            commutatively: false,
        })
    }

    pub fn create_float_div(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(ArithmeticInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            opcode: LLVMOpcode::LLVMFDiv,
            commutatively: false,
        })
    }

    pub fn create_float_rem(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(ArithmeticInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            opcode: LLVMOpcode::LLVMFRem,
            commutatively: false,
        })
    }

    pub fn create_commutatively_float_add(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(ArithmeticInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            opcode: LLVMOpcode::LLVMFAdd,
            commutatively: true,
        })
    }

    pub fn create_commutatively_float_sub(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(ArithmeticInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            opcode: LLVMOpcode::LLVMFSub,
            commutatively: true,
        })
    }

    pub fn create_commutatively_float_mul(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(ArithmeticInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            opcode: LLVMOpcode::LLVMFMul,
            commutatively: true,
        })
    }

    pub fn create_commutatively_float_div(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(ArithmeticInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            opcode: LLVMOpcode::LLVMFDiv,
            commutatively: true,
        })
    }

    pub fn create_commutatively_float_rem(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(ArithmeticInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            opcode: LLVMOpcode::LLVMFRem,
            commutatively: true,
        })
    }
}

impl InstMatcher for ArithmeticInstMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: LLVMValueRef) -> bool {
        unsafe {
            let opcode = LLVMGetInstructionOpcode(instruction);
            if opcode == self.opcode {
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
