use std::ffi::CStr;

use inkwell::llvm_sys::core::LLVMDisposeMessage;
use inkwell::llvm_sys::core::LLVMGetInstructionOpcode;
use inkwell::llvm_sys::core::LLVMGetOperand;
use inkwell::llvm_sys::core::LLVMPrintValueToString;
use inkwell::llvm_sys::prelude::LLVMValueRef;
use inkwell::llvm_sys::LLVMOpcode;

use super::InstMatcher;

#[derive(PartialEq, Clone)]
pub enum BinaryOperator {
    And,
    Or,
    OrDisjoint,
    Xor,
}

impl BinaryOperator {
    pub fn llvm_opcode(&self) -> LLVMOpcode {
        match self {
            BinaryOperator::And => LLVMOpcode::LLVMAnd,
            BinaryOperator::Or => LLVMOpcode::LLVMOr,
            BinaryOperator::OrDisjoint => LLVMOpcode::LLVMOr,
            BinaryOperator::Xor => LLVMOpcode::LLVMXor,
        }
    }
}

/// General Binary Instruction Matcher
#[derive(Clone)]
pub struct BinaryInstMatcher {
    pub lhs_matcher: Box<dyn InstMatcher>,
    pub rhs_matcher: Box<dyn InstMatcher>,
    pub operator: BinaryOperator,
    pub commutatively: bool,
}

impl BinaryInstMatcher {
    pub fn create_and(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(BinaryInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            operator: BinaryOperator::And,
            commutatively: false,
        })
    }

    pub fn create_or(lhs: Box<dyn InstMatcher>, rhs: Box<dyn InstMatcher>) -> Box<dyn InstMatcher> {
        Box::new(BinaryInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            operator: BinaryOperator::Or,
            commutatively: false,
        })
    }

    pub fn create_or_disjoint(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(BinaryInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            operator: BinaryOperator::OrDisjoint,
            commutatively: false,
        })
    }

    pub fn create_xor(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(BinaryInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            operator: BinaryOperator::Xor,
            commutatively: false,
        })
    }

    pub fn create_commutatively_and(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(BinaryInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            operator: BinaryOperator::And,
            commutatively: true,
        })
    }

    pub fn create_commutatively_or(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(BinaryInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            operator: BinaryOperator::Or,
            commutatively: true,
        })
    }

    pub fn create_commutatively_or_disjoint(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(BinaryInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            operator: BinaryOperator::OrDisjoint,
            commutatively: true,
        })
    }

    pub fn create_commutatively_xor(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(BinaryInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            operator: BinaryOperator::Xor,
            commutatively: true,
        })
    }
}

impl InstMatcher for BinaryInstMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: LLVMValueRef) -> bool {
        unsafe {
            let opcode = LLVMGetInstructionOpcode(instruction);
            if opcode == self.operator.llvm_opcode() {
                if self.operator == BinaryOperator::OrDisjoint {
                    let instruction_string = LLVMPrintValueToString(instruction);
                    let instruction_cstr = CStr::from_ptr(instruction_string).to_owned();
                    let is_or_disjoint = instruction_cstr
                        .to_str()
                        .unwrap_or("")
                        .contains("or disjoint");
                    LLVMDisposeMessage(instruction_string);
                    if !is_or_disjoint {
                        return false;
                    }
                }

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
