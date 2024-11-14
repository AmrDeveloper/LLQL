use inkwell::llvm_sys::core::LLVMGetFCmpPredicate;
use inkwell::llvm_sys::core::LLVMGetInstructionOpcode;
use inkwell::llvm_sys::core::LLVMGetOperand;
use inkwell::llvm_sys::prelude::LLVMValueRef;
use inkwell::llvm_sys::LLVMOpcode;
use inkwell::llvm_sys::LLVMRealPredicate;

use super::InstMatcher;

/// Float Comparison Inst Matcher to check if instruction is FCMP and match predicate, LHS, RHS and commutatively
#[derive(Clone)]
pub struct FloatComparisonInstMatcher {
    pub lhs_matcher: Box<dyn InstMatcher>,
    pub rhs_matcher: Box<dyn InstMatcher>,
    pub predicate: LLVMRealPredicate,
    pub commutatively: bool,
}

impl FloatComparisonInstMatcher {
    pub fn create_fcmp_eq(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(FloatComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMRealPredicate::LLVMRealOEQ,
            commutatively: false,
        })
    }

    pub fn create_fcmp_ne(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(FloatComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMRealPredicate::LLVMRealONE,
            commutatively: false,
        })
    }

    pub fn create_fcmp_gt(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(FloatComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMRealPredicate::LLVMRealOGT,
            commutatively: false,
        })
    }

    pub fn create_fcmp_ge(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(FloatComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMRealPredicate::LLVMRealOGE,
            commutatively: false,
        })
    }

    pub fn create_fcmp_lt(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(FloatComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMRealPredicate::LLVMRealOLT,
            commutatively: false,
        })
    }

    pub fn create_fcmp_le(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(FloatComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMRealPredicate::LLVMRealOLE,
            commutatively: false,
        })
    }

    pub fn create_fcmp_ord(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(FloatComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMRealPredicate::LLVMRealORD,
            commutatively: false,
        })
    }

    pub fn create_fcmp_uno(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(FloatComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMRealPredicate::LLVMRealUNO,
            commutatively: false,
        })
    }

    pub fn create_fcmp_ueq(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(FloatComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMRealPredicate::LLVMRealUEQ,
            commutatively: false,
        })
    }

    pub fn create_fcmp_une(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(FloatComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMRealPredicate::LLVMRealUNE,
            commutatively: false,
        })
    }

    pub fn create_fcmp_ugt(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(FloatComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMRealPredicate::LLVMRealUGT,
            commutatively: false,
        })
    }

    pub fn create_fcmp_uge(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(FloatComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMRealPredicate::LLVMRealUGE,
            commutatively: false,
        })
    }

    pub fn create_fcmp_ult(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(FloatComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMRealPredicate::LLVMRealULT,
            commutatively: false,
        })
    }

    pub fn create_fcmp_ule(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(FloatComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMRealPredicate::LLVMRealULE,
            commutatively: false,
        })
    }

    pub fn create_commutatively_fcmp_eq(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(FloatComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMRealPredicate::LLVMRealOEQ,
            commutatively: true,
        })
    }

    pub fn create_commutatively_fcmp_ne(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(FloatComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMRealPredicate::LLVMRealONE,
            commutatively: true,
        })
    }

    pub fn create_commutatively_fcmp_gt(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(FloatComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMRealPredicate::LLVMRealOGT,
            commutatively: true,
        })
    }

    pub fn create_commutatively_fcmp_ge(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(FloatComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMRealPredicate::LLVMRealOGE,
            commutatively: true,
        })
    }

    pub fn create_commutatively_fcmp_lt(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(FloatComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMRealPredicate::LLVMRealOLT,
            commutatively: true,
        })
    }

    pub fn create_commutatively_fcmp_le(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(FloatComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMRealPredicate::LLVMRealOLE,
            commutatively: true,
        })
    }

    pub fn create_commutatively_fcmp_ord(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(FloatComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMRealPredicate::LLVMRealORD,
            commutatively: true,
        })
    }

    pub fn create_commutatively_fcmp_uno(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(FloatComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMRealPredicate::LLVMRealUNO,
            commutatively: true,
        })
    }

    pub fn create_commutatively_fcmp_ueq(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(FloatComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMRealPredicate::LLVMRealUEQ,
            commutatively: true,
        })
    }

    pub fn create_commutatively_fcmp_une(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(FloatComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMRealPredicate::LLVMRealUNE,
            commutatively: true,
        })
    }

    pub fn create_commutatively_fcmp_ugt(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(FloatComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMRealPredicate::LLVMRealUGT,
            commutatively: true,
        })
    }

    pub fn create_commutatively_fcmp_uge(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(FloatComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMRealPredicate::LLVMRealUGE,
            commutatively: true,
        })
    }

    pub fn create_commutatively_fcmp_ult(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(FloatComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMRealPredicate::LLVMRealULT,
            commutatively: true,
        })
    }

    pub fn create_commutatively_fcmp_ule(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(FloatComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMRealPredicate::LLVMRealULE,
            commutatively: true,
        })
    }
}

impl InstMatcher for FloatComparisonInstMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: LLVMValueRef) -> bool {
        unsafe {
            let opcode = LLVMGetInstructionOpcode(instruction);
            if opcode == LLVMOpcode::LLVMFCmp && self.predicate == LLVMGetFCmpPredicate(instruction)
            {
                let rhs = LLVMGetOperand(instruction, 1);
                let lhs = LLVMGetOperand(instruction, 0);

                if self.lhs_matcher.is_match(lhs) && self.rhs_matcher.is_match(rhs) {
                    return true;
                }

                if self.commutatively
                    && self.lhs_matcher.is_match(rhs)
                    && self.rhs_matcher.is_match(lhs)
                {
                    return true;
                }
            }
            false
        }
    }
}
