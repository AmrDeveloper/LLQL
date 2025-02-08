use inkwell::llvm_sys::core::LLVMGetICmpPredicate;
use inkwell::llvm_sys::core::LLVMGetInstructionOpcode;
use inkwell::llvm_sys::core::LLVMGetOperand;
use inkwell::llvm_sys::prelude::LLVMValueRef;
use inkwell::llvm_sys::LLVMIntPredicate;
use inkwell::llvm_sys::LLVMOpcode;

use super::Matcher;

/// Int Comparison Inst Matcher to check if instruction is ICMP and match predicate, LHS, RHS and commutatively
#[derive(Clone)]
pub struct IntComparisonInstMatcher {
    pub lhs_matcher: Box<dyn Matcher<LLVMValueRef>>,
    pub rhs_matcher: Box<dyn Matcher<LLVMValueRef>>,
    pub predicate: LLVMIntPredicate,
    pub commutatively: bool,
}

impl IntComparisonInstMatcher {
    pub fn create_icmp_eq(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(IntComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMIntPredicate::LLVMIntEQ,
            commutatively: false,
        })
    }

    pub fn create_icmp_ne(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(IntComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMIntPredicate::LLVMIntNE,
            commutatively: false,
        })
    }

    pub fn create_icmp_ugt(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(IntComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMIntPredicate::LLVMIntUGT,
            commutatively: false,
        })
    }

    pub fn create_icmp_uge(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(IntComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMIntPredicate::LLVMIntUGE,
            commutatively: false,
        })
    }

    pub fn create_icmp_ult(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(IntComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMIntPredicate::LLVMIntULT,
            commutatively: false,
        })
    }

    pub fn create_icmp_ule(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(IntComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMIntPredicate::LLVMIntULE,
            commutatively: false,
        })
    }

    pub fn create_icmp_sgt(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(IntComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMIntPredicate::LLVMIntSGT,
            commutatively: false,
        })
    }

    pub fn create_icmp_sge(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(IntComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMIntPredicate::LLVMIntSGE,
            commutatively: false,
        })
    }

    pub fn create_icmp_slt(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(IntComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMIntPredicate::LLVMIntSLT,
            commutatively: false,
        })
    }

    pub fn create_icmp_sle(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(IntComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMIntPredicate::LLVMIntSLE,
            commutatively: false,
        })
    }

    pub fn create_commutatively_icmp_eq(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(IntComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMIntPredicate::LLVMIntEQ,
            commutatively: true,
        })
    }

    pub fn create_commutatively_icmp_ne(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(IntComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMIntPredicate::LLVMIntNE,
            commutatively: true,
        })
    }

    pub fn create_commutatively_icmp_ugt(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(IntComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMIntPredicate::LLVMIntUGT,
            commutatively: true,
        })
    }

    pub fn create_commutatively_icmp_uge(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(IntComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMIntPredicate::LLVMIntUGE,
            commutatively: true,
        })
    }

    pub fn create_commutatively_icmp_ult(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(IntComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMIntPredicate::LLVMIntULT,
            commutatively: true,
        })
    }

    pub fn create_commutatively_icmp_ule(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(IntComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMIntPredicate::LLVMIntULE,
            commutatively: true,
        })
    }

    pub fn create_commutatively_icmp_sgt(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(IntComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMIntPredicate::LLVMIntSGT,
            commutatively: true,
        })
    }

    pub fn create_commutatively_icmp_sge(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(IntComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMIntPredicate::LLVMIntSGE,
            commutatively: true,
        })
    }

    pub fn create_commutatively_icmp_slt(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(IntComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMIntPredicate::LLVMIntSLT,
            commutatively: true,
        })
    }

    pub fn create_commutatively_icmp_sle(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(IntComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMIntPredicate::LLVMIntSLE,
            commutatively: true,
        })
    }
}

impl Matcher<LLVMValueRef> for IntComparisonInstMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: &LLVMValueRef) -> bool {
        unsafe {
            let opcode = LLVMGetInstructionOpcode(*instruction);
            if opcode == LLVMOpcode::LLVMICmp
                && self.predicate == LLVMGetICmpPredicate(*instruction)
            {
                let rhs = LLVMGetOperand(*instruction, 1);
                let lhs = LLVMGetOperand(*instruction, 0);

                if self.lhs_matcher.is_match(&lhs) && self.rhs_matcher.is_match(&rhs) {
                    return true;
                }

                if self.commutatively
                    && self.lhs_matcher.is_match(&rhs)
                    && self.rhs_matcher.is_match(&lhs)
                {
                    return true;
                }
            }
            false
        }
    }
}
