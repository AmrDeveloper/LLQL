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
///
pub trait InstMatcher: DynClone {
    fn is_match(&self, instruction: LLVMValueRef) -> bool;
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
