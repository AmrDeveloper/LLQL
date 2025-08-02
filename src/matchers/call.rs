use inkwell::llvm_sys::core::LLVMGetCalledValue;
use inkwell::llvm_sys::core::LLVMGetInstructionOpcode;
use inkwell::llvm_sys::core::LLVMGetIntrinsicID;
use inkwell::llvm_sys::core::LLVMGetValueName2;
use inkwell::llvm_sys::core::LLVMIsACallInst;
use inkwell::llvm_sys::core::LLVMIsAFunction;
use inkwell::llvm_sys::prelude::LLVMValueRef;
use inkwell::llvm_sys::LLVMOpcode;

use super::Matcher;

#[derive(Clone)]
pub struct CallInstMatcher;

impl CallInstMatcher {
    pub fn create_call() -> Self {
        CallInstMatcher
    }
}

impl Matcher<LLVMValueRef> for CallInstMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: &LLVMValueRef) -> bool {
        unsafe { LLVMGetInstructionOpcode(*instruction) == LLVMOpcode::LLVMCall }
    }
}

#[derive(Clone)]
pub struct IntrinsicInstMatcher {
    pub name: Option<String>,
}

impl IntrinsicInstMatcher {
    pub fn create_call(name: Option<String>) -> Self {
        IntrinsicInstMatcher { name }
    }
}

impl Matcher<LLVMValueRef> for IntrinsicInstMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: &LLVMValueRef) -> bool {
        unsafe {
            if !LLVMIsACallInst(*instruction).is_null() {
                let called_value = LLVMGetCalledValue(*instruction);
                let intrinsic_id = LLVMGetIntrinsicID(called_value);
                if intrinsic_id == 0 {
                    return false;
                }

                if let Some(intrinsic_name) = &self.name {
                    let function = LLVMIsAFunction(called_value);
                    if function.is_null() {
                        return false;
                    }

                    let mut len: usize = 0;
                    let name_ptr = LLVMGetValueName2(function, &mut len);
                    let name_slice = std::slice::from_raw_parts(name_ptr as *const u8, len);
                    let name = std::str::from_utf8_unchecked(name_slice).to_string();
                    return name.eq(intrinsic_name);
                }

                return true;
            }

            false
        }
    }
}
