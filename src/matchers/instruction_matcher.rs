use dyn_clone::DynClone;
use inkwell::llvm_sys;
use llvm_sys::core::LLVMGetInstructionOpcode;
use llvm_sys::core::LLVMGetOperand;
use llvm_sys::core::LLVMTypeOf;
use llvm_sys::prelude::LLVMValueRef;
use llvm_sys::LLVMOpcode;

use super::type_matcher::TypeMatcher;

dyn_clone::clone_trait_object!(InstMatcher);

/// Instruction matcher used to create matcher that check if rules match the instruction or not
pub trait InstMatcher: DynClone {
    fn is_match(&self, instruction: LLVMValueRef) -> bool;
}

/// Instruction Matcher to match to any instruction, used mostly as default matcher
#[derive(Clone)]
pub struct AnyInstMatcher;

impl InstMatcher for AnyInstMatcher {
    fn is_match(&self, _instruction: LLVMValueRef) -> bool {
        true
    }
}

/// Binary instruction matcher to check to match instruction opcode, right and left hand sides
#[derive(Clone)]
pub struct BinaryInstMatcher {
    pub lhs_matcher: Box<dyn InstMatcher>,
    pub rhs_matcher: Box<dyn InstMatcher>,
    pub opcode: LLVMOpcode,
    pub commutatively: bool,
}

impl BinaryInstMatcher {
    pub fn create_add(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(BinaryInstMatcher {
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
        Box::new(BinaryInstMatcher {
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
        Box::new(BinaryInstMatcher {
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
        Box::new(BinaryInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            opcode: LLVMOpcode::LLVMSDiv,
            commutatively: false,
        })
    }

    pub fn create_commutatively_add(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(BinaryInstMatcher {
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
        Box::new(BinaryInstMatcher {
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
        Box::new(BinaryInstMatcher {
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
        Box::new(BinaryInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            opcode: LLVMOpcode::LLVMSDiv,
            commutatively: true,
        })
    }
}

impl InstMatcher for BinaryInstMatcher {
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

// Return instruction matcher to check if current instruction is return instruction with specific type or not
#[derive(Clone)]
pub struct ReturnInstMatcher {
    pub type_matcher: Box<dyn TypeMatcher>,
}

impl InstMatcher for ReturnInstMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: LLVMValueRef) -> bool {
        unsafe {
            let opcode = LLVMGetInstructionOpcode(instruction);
            if opcode == LLVMOpcode::LLVMRet {
                let return_value = LLVMGetOperand(instruction, 0);
                let return_type = LLVMTypeOf(return_value);
                return self.type_matcher.is_match(return_type);
            }
            false
        }
    }
}

// Unreachable instruction matcher to check if current instruction is Unreachable instruction
#[derive(Clone)]
pub struct UnreachableInstMatcher;

impl InstMatcher for UnreachableInstMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: LLVMValueRef) -> bool {
        unsafe { LLVMGetInstructionOpcode(instruction) == LLVMOpcode::LLVMUnreachable }
    }
}
