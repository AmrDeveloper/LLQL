use inkwell::llvm_sys::core::{LLVMGetInstructionOpcode, LLVMGetOperand};
use inkwell::llvm_sys::prelude::LLVMValueRef;
use inkwell::llvm_sys::LLVMOpcode;

use super::Matcher;

#[derive(PartialEq, Clone)]
enum CastMatcherKind {
    Trunc,
    IntToPtr,
    PtrToInt,
    FPToUI,
    FPToSI,
    FPTrunc,
    FPExt,
    ZExt,
    SExt,
    BitCast,
    AddrSpaceCast,
}

impl CastMatcherKind {
    pub fn llvm_opcode(&self) -> LLVMOpcode {
        match self {
            CastMatcherKind::Trunc => LLVMOpcode::LLVMTrunc,
            CastMatcherKind::FPToUI => LLVMOpcode::LLVMFPToUI,
            CastMatcherKind::FPToSI => LLVMOpcode::LLVMFPToSI,
            CastMatcherKind::FPTrunc => LLVMOpcode::LLVMFPTrunc,
            CastMatcherKind::FPExt => LLVMOpcode::LLVMFPExt,
            CastMatcherKind::IntToPtr => LLVMOpcode::LLVMIntToPtr,
            CastMatcherKind::PtrToInt => LLVMOpcode::LLVMPtrToInt,
            CastMatcherKind::ZExt => LLVMOpcode::LLVMZExt,
            CastMatcherKind::SExt => LLVMOpcode::LLVMSExt,
            CastMatcherKind::BitCast => LLVMOpcode::LLVMBitCast,
            CastMatcherKind::AddrSpaceCast => LLVMOpcode::LLVMAddrSpaceCast,
        }
    }
}

#[derive(Clone)]
pub struct CastInstMatcher {
    value_matcher: Box<dyn Matcher<LLVMValueRef>>,
    kind: CastMatcherKind,
}

impl CastInstMatcher {
    fn new(value_matcher: Box<dyn Matcher<LLVMValueRef>>, kind: CastMatcherKind) -> Self {
        CastInstMatcher {
            value_matcher,
            kind,
        }
    }
}

impl CastInstMatcher {
    pub fn create_trunc(value_matcher: Box<dyn Matcher<LLVMValueRef>>) -> CastInstMatcher {
        CastInstMatcher::new(value_matcher, CastMatcherKind::Trunc)
    }

    pub fn create_fp_to_ui(value_matcher: Box<dyn Matcher<LLVMValueRef>>) -> CastInstMatcher {
        CastInstMatcher::new(value_matcher, CastMatcherKind::FPToUI)
    }

    pub fn create_fp_to_si(value_matcher: Box<dyn Matcher<LLVMValueRef>>) -> CastInstMatcher {
        CastInstMatcher::new(value_matcher, CastMatcherKind::FPToSI)
    }

    pub fn create_fp_trunc(value_matcher: Box<dyn Matcher<LLVMValueRef>>) -> CastInstMatcher {
        CastInstMatcher::new(value_matcher, CastMatcherKind::FPTrunc)
    }

    pub fn create_fpext(value_matcher: Box<dyn Matcher<LLVMValueRef>>) -> CastInstMatcher {
        CastInstMatcher::new(value_matcher, CastMatcherKind::FPExt)
    }

    pub fn create_int_to_ptr(value_matcher: Box<dyn Matcher<LLVMValueRef>>) -> CastInstMatcher {
        CastInstMatcher::new(value_matcher, CastMatcherKind::IntToPtr)
    }

    pub fn create_ptr_to_int(value_matcher: Box<dyn Matcher<LLVMValueRef>>) -> CastInstMatcher {
        CastInstMatcher::new(value_matcher, CastMatcherKind::PtrToInt)
    }

    pub fn create_zext(value_matcher: Box<dyn Matcher<LLVMValueRef>>) -> CastInstMatcher {
        CastInstMatcher::new(value_matcher, CastMatcherKind::ZExt)
    }

    pub fn create_sext(value_matcher: Box<dyn Matcher<LLVMValueRef>>) -> CastInstMatcher {
        CastInstMatcher::new(value_matcher, CastMatcherKind::SExt)
    }

    pub fn create_bit_cast(value_matcher: Box<dyn Matcher<LLVMValueRef>>) -> CastInstMatcher {
        CastInstMatcher::new(value_matcher, CastMatcherKind::BitCast)
    }

    pub fn create_addr_space_cast(
        value_matcher: Box<dyn Matcher<LLVMValueRef>>,
    ) -> CastInstMatcher {
        CastInstMatcher::new(value_matcher, CastMatcherKind::AddrSpaceCast)
    }
}

impl Matcher<LLVMValueRef> for CastInstMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: &LLVMValueRef) -> bool {
        unsafe {
            if self.kind.llvm_opcode() != LLVMGetInstructionOpcode(*instruction) {
                return false;
            }

            let value = LLVMGetOperand(*instruction, 0);
            if !self.value_matcher.is_match(&value) {
                return false;
            }

            true
        }
    }
}
