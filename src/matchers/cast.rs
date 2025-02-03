use inkwell::llvm_sys::core::LLVMGetInstructionOpcode;
use inkwell::llvm_sys::prelude::LLVMValueRef;
use inkwell::llvm_sys::LLVMOpcode;

use super::InstMatcher;

#[derive(PartialEq, Clone)]
enum CastMatcherKind {
    IntToPtr,
    PtrToInt,
}

impl CastMatcherKind {
    pub fn llvm_opcode(&self) -> LLVMOpcode {
        match self {
            CastMatcherKind::IntToPtr => LLVMOpcode::LLVMIntToPtr,
            CastMatcherKind::PtrToInt => LLVMOpcode::LLVMPtrToInt,
        }
    }
}

#[derive(Clone)]
pub struct CastInstMatcher {
    matcher_kind: CastMatcherKind,
}

impl CastInstMatcher {
    pub fn create_int_to_ptr() -> CastInstMatcher {
        CastInstMatcher {
            matcher_kind: CastMatcherKind::IntToPtr,
        }
    }

    pub fn create_ptr_to_int() -> CastInstMatcher {
        CastInstMatcher {
            matcher_kind: CastMatcherKind::PtrToInt,
        }
    }
}

impl InstMatcher for CastInstMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: LLVMValueRef) -> bool {
        unsafe { self.matcher_kind.llvm_opcode() == LLVMGetInstructionOpcode(instruction) }
    }
}
