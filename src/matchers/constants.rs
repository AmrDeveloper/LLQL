use inkwell::llvm_sys::core::LLVMGetValueKind;
use inkwell::llvm_sys::prelude::LLVMValueRef;
use inkwell::llvm_sys::LLVMValueKind;

use super::InstMatcher;

/// Return instruction matcher to check if current value is a constants integer
#[derive(Clone)]
pub struct ConstIntMatcher;

impl InstMatcher for ConstIntMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: LLVMValueRef) -> bool {
        unsafe {
            let value_kind = LLVMGetValueKind(instruction);
            value_kind == LLVMValueKind::LLVMConstantIntValueKind
        }
    }
}

/// Return instruction matcher to check if current value is a constants floating point
#[derive(Clone)]
pub struct ConstFloatMatcher;

impl InstMatcher for ConstFloatMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: LLVMValueRef) -> bool {
        unsafe {
            let value_kind = LLVMGetValueKind(instruction);
            value_kind == LLVMValueKind::LLVMConstantFPValueKind
        }
    }
}

/// Return instruction matcher to check if current value is a constants pointer null
#[derive(Clone)]
pub struct ConstPointerNullMatcher;

impl InstMatcher for ConstPointerNullMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: LLVMValueRef) -> bool {
        unsafe {
            let value_kind = LLVMGetValueKind(instruction);
            value_kind == LLVMValueKind::LLVMConstantPointerNullValueKind
        }
    }
}
