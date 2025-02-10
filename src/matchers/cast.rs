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
    Ext,
    FPExt,
    ZExt,
    SExt,
    BitCast,
    AddrSpaceCast,
}

impl CastMatcherKind {
    pub fn match_llvm_opcode(&self, llvm_op: LLVMOpcode) -> bool {
        match llvm_op {
            LLVMOpcode::LLVMTrunc => matches!(self, CastMatcherKind::Trunc),
            LLVMOpcode::LLVMFPToUI => matches!(self, CastMatcherKind::FPToUI),
            LLVMOpcode::LLVMFPToSI => matches!(self, CastMatcherKind::FPToSI),
            LLVMOpcode::LLVMFPTrunc => matches!(self, CastMatcherKind::FPTrunc),

            LLVMOpcode::LLVMFPExt => matches!(self, CastMatcherKind::Ext | CastMatcherKind::FPExt),
            LLVMOpcode::LLVMZExt => matches!(self, CastMatcherKind::Ext | CastMatcherKind::ZExt),
            LLVMOpcode::LLVMSExt => matches!(self, CastMatcherKind::Ext | CastMatcherKind::SExt),

            LLVMOpcode::LLVMIntToPtr => matches!(self, CastMatcherKind::IntToPtr),
            LLVMOpcode::LLVMPtrToInt => matches!(self, CastMatcherKind::PtrToInt),
            LLVMOpcode::LLVMBitCast => matches!(self, CastMatcherKind::BitCast),
            LLVMOpcode::LLVMAddrSpaceCast => matches!(self, CastMatcherKind::AddrSpaceCast),
            _ => false,
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

    pub fn create_ext(value_matcher: Box<dyn Matcher<LLVMValueRef>>) -> CastInstMatcher {
        CastInstMatcher::new(value_matcher, CastMatcherKind::Ext)
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
            let opcode = LLVMGetInstructionOpcode(*instruction);
            if !self.kind.match_llvm_opcode(opcode) {
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
