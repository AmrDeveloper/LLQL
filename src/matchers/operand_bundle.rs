use std::ffi::CStr;
use std::slice::from_raw_parts;

use inkwell::llvm_sys::core::LLVMGetNumOperandBundles;
use inkwell::llvm_sys::core::LLVMGetOperandBundleAtIndex;
use inkwell::llvm_sys::core::LLVMGetOperandBundleTag;
use inkwell::llvm_sys::prelude::LLVMValueRef;

use super::Matcher;

#[derive(Clone)]
pub struct OperandBundleNameMatcher {
    pub name: String,
}

impl OperandBundleNameMatcher {
    pub fn create(name: String) -> Self {
        OperandBundleNameMatcher { name }
    }
}

impl Matcher<LLVMValueRef> for OperandBundleNameMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: &LLVMValueRef) -> bool {
        unsafe {
            let operand_bundle_len = LLVMGetNumOperandBundles(*instruction);
            if operand_bundle_len == 0 {
                return false;
            }

            for i in 0..operand_bundle_len {
                let operand_bundle_ref = LLVMGetOperandBundleAtIndex(*instruction, i);
                let mut len = 0;
                let tag_ptr = LLVMGetOperandBundleTag(operand_bundle_ref, &mut len);
                if tag_ptr.is_null() {
                    continue;
                }

                if let Ok(tag_literal) =
                    CStr::from_bytes_until_nul(from_raw_parts(tag_ptr as *const u8, len + 1))
                {
                    if let Ok(tag_literal_string) = tag_literal.to_str() {
                        return self.name == tag_literal_string;
                    }
                }
            }

            false
        }
    }
}
