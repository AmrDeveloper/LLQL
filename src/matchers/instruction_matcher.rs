use std::ffi::CStr;

use dyn_clone::DynClone;
use inkwell::llvm_sys;
use inkwell::llvm_sys::core::LLVMGetFCmpPredicate;
use inkwell::llvm_sys::core::LLVMGetICmpPredicate;
use inkwell::llvm_sys::core::LLVMGetValueKind;
use inkwell::llvm_sys::LLVMIntPredicate;
use inkwell::llvm_sys::LLVMRealPredicate;
use inkwell::llvm_sys::LLVMValueKind;
use llvm_sys::core::LLVMGetInstructionOpcode;
use llvm_sys::core::LLVMGetOperand;
use llvm_sys::prelude::LLVMValueRef;
use llvm_sys::LLVMOpcode;

dyn_clone::clone_trait_object!(InstMatcher);

/// Instruction matcher used to create matcher that check if rules match the instruction or not
pub trait InstMatcher: DynClone {
    fn is_match(&self, instruction: LLVMValueRef) -> bool;
}

/// Instruction Matcher to match to any instruction, used mostly as default matcher
#[derive(Clone)]
pub struct AnyInstMatcher;

impl InstMatcher for AnyInstMatcher {
    fn is_match(&self, _instruction: LLVMValueRef) -> bool {
        true
    }
}

/// Arithmetic instruction matcher to check if instruction is arithmetic and match opcode, LHS, RHS and commutatively
#[derive(Clone)]
pub struct ArithmeticInstMatcher {
    pub lhs_matcher: Box<dyn InstMatcher>,
    pub rhs_matcher: Box<dyn InstMatcher>,
    pub opcode: LLVMOpcode,
    pub commutatively: bool,
}

impl ArithmeticInstMatcher {
    pub fn create_add(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(ArithmeticInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            opcode: LLVMOpcode::LLVMAdd,
            commutatively: false,
        })
    }

    pub fn create_sub(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(ArithmeticInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            opcode: LLVMOpcode::LLVMSub,
            commutatively: false,
        })
    }

    pub fn create_mul(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(ArithmeticInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            opcode: LLVMOpcode::LLVMMul,
            commutatively: false,
        })
    }

    pub fn create_div(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(ArithmeticInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            opcode: LLVMOpcode::LLVMSDiv,
            commutatively: false,
        })
    }

    pub fn create_rem(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(ArithmeticInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            opcode: LLVMOpcode::LLVMSRem,
            commutatively: false,
        })
    }

    pub fn create_commutatively_add(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(ArithmeticInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            opcode: LLVMOpcode::LLVMAdd,
            commutatively: true,
        })
    }

    pub fn create_commutatively_sub(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(ArithmeticInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            opcode: LLVMOpcode::LLVMSub,
            commutatively: true,
        })
    }

    pub fn create_commutatively_mul(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(ArithmeticInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            opcode: LLVMOpcode::LLVMMul,
            commutatively: true,
        })
    }

    pub fn create_commutatively_div(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(ArithmeticInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            opcode: LLVMOpcode::LLVMSDiv,
            commutatively: true,
        })
    }

    pub fn create_commutatively_rem(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(ArithmeticInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            opcode: LLVMOpcode::LLVMSRem,
            commutatively: true,
        })
    }

    pub fn create_float_add(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(ArithmeticInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            opcode: LLVMOpcode::LLVMFAdd,
            commutatively: false,
        })
    }

    pub fn create_float_sub(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(ArithmeticInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            opcode: LLVMOpcode::LLVMFSub,
            commutatively: false,
        })
    }

    pub fn create_float_mul(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(ArithmeticInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            opcode: LLVMOpcode::LLVMFMul,
            commutatively: false,
        })
    }

    pub fn create_float_div(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(ArithmeticInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            opcode: LLVMOpcode::LLVMFDiv,
            commutatively: false,
        })
    }

    pub fn create_float_rem(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(ArithmeticInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            opcode: LLVMOpcode::LLVMFRem,
            commutatively: false,
        })
    }

    pub fn create_commutatively_float_add(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(ArithmeticInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            opcode: LLVMOpcode::LLVMFAdd,
            commutatively: true,
        })
    }

    pub fn create_commutatively_float_sub(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(ArithmeticInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            opcode: LLVMOpcode::LLVMFSub,
            commutatively: true,
        })
    }

    pub fn create_commutatively_float_mul(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(ArithmeticInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            opcode: LLVMOpcode::LLVMFMul,
            commutatively: true,
        })
    }

    pub fn create_commutatively_float_div(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(ArithmeticInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            opcode: LLVMOpcode::LLVMFDiv,
            commutatively: true,
        })
    }

    pub fn create_commutatively_float_rem(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(ArithmeticInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            opcode: LLVMOpcode::LLVMFRem,
            commutatively: true,
        })
    }
}

impl InstMatcher for ArithmeticInstMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: LLVMValueRef) -> bool {
        unsafe {
            let opcode = LLVMGetInstructionOpcode(instruction);
            if opcode == self.opcode {
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

/// Int Comparison Inst Matcher to check if instruction is ICMP and match predicate, LHS, RHS and commutatively
#[derive(Clone)]
pub struct IntComparisonInstMatcher {
    pub lhs_matcher: Box<dyn InstMatcher>,
    pub rhs_matcher: Box<dyn InstMatcher>,
    pub predicate: LLVMIntPredicate,
    pub commutatively: bool,
}

impl IntComparisonInstMatcher {
    pub fn create_icmp_eq(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(IntComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMIntPredicate::LLVMIntEQ,
            commutatively: false,
        })
    }

    pub fn create_icmp_ne(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(IntComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMIntPredicate::LLVMIntNE,
            commutatively: false,
        })
    }

    pub fn create_icmp_ugt(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(IntComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMIntPredicate::LLVMIntUGT,
            commutatively: false,
        })
    }

    pub fn create_icmp_uge(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(IntComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMIntPredicate::LLVMIntUGE,
            commutatively: false,
        })
    }

    pub fn create_icmp_ult(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(IntComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMIntPredicate::LLVMIntULT,
            commutatively: false,
        })
    }

    pub fn create_icmp_ule(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(IntComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMIntPredicate::LLVMIntULE,
            commutatively: false,
        })
    }

    pub fn create_icmp_sgt(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(IntComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMIntPredicate::LLVMIntSGT,
            commutatively: false,
        })
    }

    pub fn create_icmp_sge(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(IntComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMIntPredicate::LLVMIntSGE,
            commutatively: false,
        })
    }

    pub fn create_icmp_slt(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(IntComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMIntPredicate::LLVMIntSLT,
            commutatively: false,
        })
    }

    pub fn create_icmp_sle(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(IntComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMIntPredicate::LLVMIntSLE,
            commutatively: false,
        })
    }

    pub fn create_commutatively_icmp_eq(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(IntComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMIntPredicate::LLVMIntEQ,
            commutatively: true,
        })
    }

    pub fn create_commutatively_icmp_ne(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(IntComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMIntPredicate::LLVMIntNE,
            commutatively: true,
        })
    }

    pub fn create_commutatively_icmp_ugt(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(IntComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMIntPredicate::LLVMIntUGT,
            commutatively: true,
        })
    }

    pub fn create_commutatively_icmp_uge(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(IntComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMIntPredicate::LLVMIntUGE,
            commutatively: true,
        })
    }

    pub fn create_commutatively_icmp_ult(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(IntComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMIntPredicate::LLVMIntULT,
            commutatively: true,
        })
    }

    pub fn create_commutatively_icmp_ule(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(IntComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMIntPredicate::LLVMIntULE,
            commutatively: true,
        })
    }

    pub fn create_commutatively_icmp_sgt(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(IntComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMIntPredicate::LLVMIntSGT,
            commutatively: true,
        })
    }

    pub fn create_commutatively_icmp_sge(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(IntComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMIntPredicate::LLVMIntSGE,
            commutatively: true,
        })
    }

    pub fn create_commutatively_icmp_slt(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(IntComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMIntPredicate::LLVMIntSLT,
            commutatively: true,
        })
    }

    pub fn create_commutatively_icmp_sle(
        lhs: Box<dyn InstMatcher>,
        rhs: Box<dyn InstMatcher>,
    ) -> Box<dyn InstMatcher> {
        Box::new(IntComparisonInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            predicate: LLVMIntPredicate::LLVMIntSLE,
            commutatively: true,
        })
    }
}

impl InstMatcher for IntComparisonInstMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: LLVMValueRef) -> bool {
        unsafe {
            let opcode = LLVMGetInstructionOpcode(instruction);
            if opcode == LLVMOpcode::LLVMICmp && self.predicate == LLVMGetICmpPredicate(instruction)
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

// Return instruction matcher to check if current value is a constants integer
#[derive(Clone)]
pub struct ConstIntMatcher;

impl InstMatcher for ConstIntMatcher {
    #[allow(deprecated)]
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: LLVMValueRef) -> bool {
        unsafe {
            let value_kind = LLVMGetValueKind(instruction);
            value_kind == LLVMValueKind::LLVMConstantIntValueKind
        }
    }
}

// Return instruction matcher to check if current value is a constants floating point
#[derive(Clone)]
pub struct ConstFloatMatcher;

impl InstMatcher for ConstFloatMatcher {
    #[allow(deprecated)]
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: LLVMValueRef) -> bool {
        unsafe {
            let value_kind = LLVMGetValueKind(instruction);
            value_kind == LLVMValueKind::LLVMConstantFPValueKind
        }
    }
}

// Return instruction matcher to check if current value is a constants pointer null
#[derive(Clone)]
pub struct ConstPointerNullMatcher;

impl InstMatcher for ConstPointerNullMatcher {
    #[allow(deprecated)]
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: LLVMValueRef) -> bool {
        unsafe {
            let value_kind = LLVMGetValueKind(instruction);
            value_kind == LLVMValueKind::LLVMConstantPointerNullValueKind
        }
    }
}

// Return instruction matcher to check if current value is poison
#[derive(Clone)]
pub struct PoisonInstMatcher;

impl InstMatcher for PoisonInstMatcher {
    #[allow(deprecated)]
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: LLVMValueRef) -> bool {
        unsafe {
            let value_kind = LLVMGetValueKind(instruction);
            value_kind == LLVMValueKind::LLVMPoisonValueKind
        }
    }
}

// Return instruction matcher to check if current instruction is return instruction with optional specific name or not
#[derive(Clone)]
pub struct LabelInstMatcher {
    pub name: Option<String>,
}

impl InstMatcher for LabelInstMatcher {
    #[allow(deprecated)]
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: LLVMValueRef) -> bool {
        unsafe {
            let value_kind = LLVMGetValueKind(instruction);
            if value_kind == LLVMValueKind::LLVMBasicBlockValueKind {
                if let Some(name) = &self.name {
                    let label_value_name = llvm_sys::core::LLVMGetValueName(instruction);
                    let name_str = CStr::from_ptr(label_value_name).to_str().unwrap();
                    return name.eq(name_str);
                }
                return true;
            }
            false
        }
    }
}

// Return instruction matcher to check if current instruction is return instruction with specific type or not
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

// Unreachable instruction matcher to check if current instruction is Unreachable instruction
#[derive(Clone)]
pub struct UnreachableInstMatcher;

impl InstMatcher for UnreachableInstMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: LLVMValueRef) -> bool {
        unsafe { LLVMGetInstructionOpcode(instruction) == LLVMOpcode::LLVMUnreachable }
    }
}
