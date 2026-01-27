use inkwell::llvm_sys::core::LLVMIsAGetElementPtrInst;
use inkwell::llvm_sys::prelude::LLVMValueRef;

use super::Matcher;

/// Return instruction matcher to check if current inst is GetElementPtr
#[derive(Clone)]
pub struct GetElementPtrMatcher;

impl Matcher<LLVMValueRef> for GetElementPtrMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: &LLVMValueRef) -> bool {
        unsafe { !LLVMIsAGetElementPtrInst(*instruction).is_null() }
    }
}
