use inkwell::llvm_sys::core::LLVMGetInstructionOpcode;
use inkwell::llvm_sys::prelude::LLVMValueRef;
use inkwell::llvm_sys::LLVMOpcode;

use super::InstMatcher;

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
    kind: CastMatcherKind,
}

impl CastInstMatcher {
    fn new(kind: CastMatcherKind) -> Self {
        CastInstMatcher { kind }
    }
}

impl CastInstMatcher {
    pub fn create_trunc() -> CastInstMatcher {
        CastInstMatcher::new(CastMatcherKind::Trunc)
    }

    pub fn create_fp_to_ui() -> CastInstMatcher {
        CastInstMatcher::new(CastMatcherKind::FPToUI)
    }

    pub fn create_fp_to_si() -> CastInstMatcher {
        CastInstMatcher::new(CastMatcherKind::FPToSI)
    }

    pub fn create_fp_trunc() -> CastInstMatcher {
        CastInstMatcher::new(CastMatcherKind::FPTrunc)
    }

    pub fn create_fpext() -> CastInstMatcher {
        CastInstMatcher::new(CastMatcherKind::FPExt)
    }

    pub fn create_int_to_ptr() -> CastInstMatcher {
        CastInstMatcher::new(CastMatcherKind::IntToPtr)
    }

    pub fn create_ptr_to_int() -> CastInstMatcher {
        CastInstMatcher::new(CastMatcherKind::PtrToInt)
    }

    pub fn create_zext() -> CastInstMatcher {
        CastInstMatcher::new(CastMatcherKind::ZExt)
    }

    pub fn create_sext() -> CastInstMatcher {
        CastInstMatcher::new(CastMatcherKind::SExt)
    }

    pub fn create_bit_cast() -> CastInstMatcher {
        CastInstMatcher::new(CastMatcherKind::BitCast)
    }

    pub fn create_addr_space_cast() -> CastInstMatcher {
        CastInstMatcher::new(CastMatcherKind::AddrSpaceCast)
    }
}

impl InstMatcher for CastInstMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: LLVMValueRef) -> bool {
        unsafe { self.kind.llvm_opcode() == LLVMGetInstructionOpcode(instruction) }
    }
}
