use std::ffi::CStr;

use inkwell::llvm_sys::core::LLVMGetInstructionOpcode;
use inkwell::llvm_sys::core::LLVMGetOperand;
use inkwell::llvm_sys::core::LLVMGetValueKind;
use inkwell::llvm_sys::core::LLVMGetValueName2;
use inkwell::llvm_sys::core::LLVMTypeOf;
use inkwell::llvm_sys::prelude::LLVMValueRef;
use inkwell::llvm_sys::LLVMOpcode;
use inkwell::llvm_sys::LLVMValueKind;

use super::InstMatcher;
use super::TypeMatcher;

#[derive(Clone)]
pub struct InstTypeMatcher {
    pub matcher: Box<dyn TypeMatcher>,
}

impl InstMatcher for InstTypeMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: LLVMValueRef) -> bool {
        unsafe {
            let value_type = LLVMTypeOf(instruction);
            self.matcher.is_match(value_type)
        }
    }
}

#[derive(Clone)]
pub struct PoisonValueMatcher;

impl InstMatcher for PoisonValueMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: LLVMValueRef) -> bool {
        unsafe {
            let value_kind = LLVMGetValueKind(instruction);
            value_kind == LLVMValueKind::LLVMPoisonValueKind
        }
    }
}

/// Return instruction matcher to check if current value is function argument with optional name
#[derive(Clone)]
pub struct ArgumentMatcher {
    pub name: Option<String>,
    pub type_matcher: Option<Box<dyn TypeMatcher>>,
}

impl InstMatcher for ArgumentMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: LLVMValueRef) -> bool {
        unsafe {
            let value_kind = LLVMGetValueKind(instruction);
            if value_kind == LLVMValueKind::LLVMArgumentValueKind {
                if let Some(name) = &self.name {
                    let mut name_len: usize = 0;
                    let label_value_name =
                        LLVMGetValueName2(instruction, &mut name_len as *mut usize);
                    let name_str = CStr::from_ptr(label_value_name).to_str().unwrap();
                    let is_name_matches = name.eq(name_str);
                    if is_name_matches {
                        if let Some(type_matcher) = &self.type_matcher {
                            let value_type = LLVMTypeOf(instruction);
                            return type_matcher.is_match(value_type);
                        }
                        return true;
                    }
                }
                return true;
            }
            false
        }
    }
}

/// Return instruction matcher to check if current instruction is return instruction with optional specific name or not
#[derive(Clone)]
pub struct LabelInstMatcher {
    pub name: Option<String>,
}

impl InstMatcher for LabelInstMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: LLVMValueRef) -> bool {
        unsafe {
            let value_kind = LLVMGetValueKind(instruction);
            if value_kind == LLVMValueKind::LLVMBasicBlockValueKind {
                if let Some(name) = &self.name {
                    let mut name_len: usize = 0;
                    let label_value_name =
                        LLVMGetValueName2(instruction, &mut name_len as *mut usize);
                    let name_str = CStr::from_ptr(label_value_name).to_str().unwrap();
                    return name.eq(name_str);
                }
                return true;
            }
            false
        }
    }
}

/// Return instruction matcher to check if current instruction is return instruction with specific type or not
#[derive(Clone)]
pub struct ReturnInstMatcher {
    pub matcher: Box<dyn InstMatcher>,
}

impl InstMatcher for ReturnInstMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: LLVMValueRef) -> bool {
        unsafe {
            let opcode = LLVMGetInstructionOpcode(instruction);
            if opcode == LLVMOpcode::LLVMRet {
                let return_value = LLVMGetOperand(instruction, 0);
                return self.matcher.is_match(return_value);
            }
            false
        }
    }
}

/// Unreachable instruction matcher to check if current instruction is Unreachable instruction
#[derive(Clone)]
pub struct UnreachableInstMatcher;

impl InstMatcher for UnreachableInstMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: LLVMValueRef) -> bool {
        unsafe { LLVMGetInstructionOpcode(instruction) == LLVMOpcode::LLVMUnreachable }
    }
}
