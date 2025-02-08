use std::ffi::CStr;

use inkwell::llvm_sys::core::LLVMDisposeMessage;
use inkwell::llvm_sys::core::LLVMGetInstructionOpcode;
use inkwell::llvm_sys::core::LLVMGetOperand;
use inkwell::llvm_sys::core::LLVMPrintValueToString;
use inkwell::llvm_sys::prelude::LLVMValueRef;
use inkwell::llvm_sys::LLVMOpcode;

use super::Matcher;

#[derive(PartialEq, Clone)]
pub enum BinaryOperator {
    Any,

    Add,
    Sub,
    Mul,
    SDiv,
    SRem,
    FAdd,
    FSub,
    FMul,
    FDiv,
    FRem,

    And,
    Or,
    OrDisjoint,
    Xor,

    LogicalShiftLeft,
    LogicalShiftRight,
    ArithmeticShiftRight,
}

impl BinaryOperator {
    pub fn match_llvm_opcode(&self, llvm_op: LLVMOpcode) -> bool {
        match llvm_op {
            LLVMOpcode::LLVMAdd => matches!(self, BinaryOperator::Any | BinaryOperator::Add),

            LLVMOpcode::LLVMSub => matches!(self, BinaryOperator::Any | BinaryOperator::Sub),

            LLVMOpcode::LLVMMul => matches!(self, BinaryOperator::Any | BinaryOperator::Mul),

            LLVMOpcode::LLVMSDiv => matches!(self, BinaryOperator::Any | BinaryOperator::SDiv),

            LLVMOpcode::LLVMSRem => matches!(self, BinaryOperator::Any | BinaryOperator::SRem),

            LLVMOpcode::LLVMFAdd => matches!(self, BinaryOperator::Any | BinaryOperator::FAdd),

            LLVMOpcode::LLVMFSub => matches!(self, BinaryOperator::Any | BinaryOperator::FSub),

            LLVMOpcode::LLVMFMul => matches!(self, BinaryOperator::Any | BinaryOperator::FMul),

            LLVMOpcode::LLVMFDiv => matches!(self, BinaryOperator::Any | BinaryOperator::FDiv),

            LLVMOpcode::LLVMFRem => matches!(self, BinaryOperator::Any | BinaryOperator::FRem),

            LLVMOpcode::LLVMAnd => matches!(self, BinaryOperator::Any | BinaryOperator::And),

            LLVMOpcode::LLVMOr => matches!(
                self,
                BinaryOperator::Any | BinaryOperator::Or | BinaryOperator::OrDisjoint
            ),

            LLVMOpcode::LLVMXor => matches!(self, BinaryOperator::Any | BinaryOperator::Xor),

            LLVMOpcode::LLVMShl => {
                matches!(self, BinaryOperator::Any | BinaryOperator::LogicalShiftLeft)
            }

            LLVMOpcode::LLVMLShr => matches!(
                self,
                BinaryOperator::Any | BinaryOperator::LogicalShiftRight
            ),

            LLVMOpcode::LLVMAShr => matches!(
                self,
                BinaryOperator::Any | BinaryOperator::ArithmeticShiftRight
            ),

            _ => false,
        }
    }
}

/// General Binary Instruction Matcher
#[derive(Clone)]
pub struct BinaryInstMatcher {
    pub lhs_matcher: Box<dyn Matcher<LLVMValueRef>>,
    pub rhs_matcher: Box<dyn Matcher<LLVMValueRef>>,
    pub operator: BinaryOperator,
    pub commutatively: bool,
}

impl BinaryInstMatcher {
    pub fn create_any(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(BinaryInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            operator: BinaryOperator::Any,
            commutatively: false,
        })
    }

    pub fn create_commutatively_any(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(BinaryInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            operator: BinaryOperator::Any,
            commutatively: true,
        })
    }

    pub fn create_add(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(BinaryInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            operator: BinaryOperator::Add,
            commutatively: false,
        })
    }

    pub fn create_sub(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(BinaryInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            operator: BinaryOperator::Sub,
            commutatively: false,
        })
    }

    pub fn create_mul(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(BinaryInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            operator: BinaryOperator::Mul,
            commutatively: false,
        })
    }

    pub fn create_div(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(BinaryInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            operator: BinaryOperator::SDiv,
            commutatively: false,
        })
    }

    pub fn create_rem(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(BinaryInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            operator: BinaryOperator::SRem,
            commutatively: false,
        })
    }

    pub fn create_commutatively_add(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(BinaryInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            operator: BinaryOperator::Add,
            commutatively: true,
        })
    }

    pub fn create_commutatively_sub(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(BinaryInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            operator: BinaryOperator::Sub,
            commutatively: true,
        })
    }

    pub fn create_commutatively_mul(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(BinaryInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            operator: BinaryOperator::Mul,
            commutatively: true,
        })
    }

    pub fn create_commutatively_div(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(BinaryInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            operator: BinaryOperator::SDiv,
            commutatively: true,
        })
    }

    pub fn create_commutatively_rem(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(BinaryInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            operator: BinaryOperator::SRem,
            commutatively: true,
        })
    }

    pub fn create_float_add(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(BinaryInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            operator: BinaryOperator::FAdd,
            commutatively: false,
        })
    }

    pub fn create_float_sub(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(BinaryInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            operator: BinaryOperator::FSub,
            commutatively: false,
        })
    }

    pub fn create_float_mul(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(BinaryInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            operator: BinaryOperator::FMul,
            commutatively: false,
        })
    }

    pub fn create_float_div(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(BinaryInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            operator: BinaryOperator::FDiv,
            commutatively: false,
        })
    }

    pub fn create_float_rem(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(BinaryInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            operator: BinaryOperator::FRem,
            commutatively: false,
        })
    }

    pub fn create_commutatively_float_add(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(BinaryInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            operator: BinaryOperator::FAdd,
            commutatively: true,
        })
    }

    pub fn create_commutatively_float_sub(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(BinaryInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            operator: BinaryOperator::FSub,
            commutatively: true,
        })
    }

    pub fn create_commutatively_float_mul(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(BinaryInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            operator: BinaryOperator::FMul,
            commutatively: true,
        })
    }

    pub fn create_commutatively_float_div(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(BinaryInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            operator: BinaryOperator::FDiv,
            commutatively: true,
        })
    }

    pub fn create_commutatively_float_rem(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(BinaryInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            operator: BinaryOperator::FRem,
            commutatively: true,
        })
    }

    pub fn create_logical_shl(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(BinaryInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            operator: BinaryOperator::LogicalShiftLeft,
            commutatively: false,
        })
    }

    pub fn create_logical_shr(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(BinaryInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            operator: BinaryOperator::LogicalShiftRight,
            commutatively: false,
        })
    }

    pub fn create_arithmetic_shr(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(BinaryInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            operator: BinaryOperator::ArithmeticShiftRight,
            commutatively: false,
        })
    }

    pub fn create_commutatively_logical_shl(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(BinaryInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            operator: BinaryOperator::LogicalShiftLeft,
            commutatively: true,
        })
    }

    pub fn create_commutatively_logical_shr(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(BinaryInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            operator: BinaryOperator::LogicalShiftRight,
            commutatively: true,
        })
    }

    pub fn create_commutatively_arithmetic_shr(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(BinaryInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            operator: BinaryOperator::ArithmeticShiftRight,
            commutatively: true,
        })
    }

    pub fn create_and(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(BinaryInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            operator: BinaryOperator::And,
            commutatively: false,
        })
    }

    pub fn create_or(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(BinaryInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            operator: BinaryOperator::Or,
            commutatively: false,
        })
    }

    pub fn create_or_disjoint(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(BinaryInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            operator: BinaryOperator::OrDisjoint,
            commutatively: false,
        })
    }

    pub fn create_xor(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(BinaryInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            operator: BinaryOperator::Xor,
            commutatively: false,
        })
    }

    pub fn create_commutatively_and(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(BinaryInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            operator: BinaryOperator::And,
            commutatively: true,
        })
    }

    pub fn create_commutatively_or(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(BinaryInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            operator: BinaryOperator::Or,
            commutatively: true,
        })
    }

    pub fn create_commutatively_or_disjoint(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(BinaryInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            operator: BinaryOperator::OrDisjoint,
            commutatively: true,
        })
    }

    pub fn create_commutatively_xor(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(BinaryInstMatcher {
            lhs_matcher: lhs,
            rhs_matcher: rhs,
            operator: BinaryOperator::Xor,
            commutatively: true,
        })
    }
}

impl Matcher<LLVMValueRef> for BinaryInstMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: &LLVMValueRef) -> bool {
        unsafe {
            let operator = LLVMGetInstructionOpcode(*instruction);
            if self.operator.match_llvm_opcode(operator) {
                if self.operator == BinaryOperator::OrDisjoint {
                    let instruction_string = LLVMPrintValueToString(*instruction);
                    let instruction_cstr = CStr::from_ptr(instruction_string).to_owned();
                    let is_or_disjoint = instruction_cstr
                        .to_str()
                        .unwrap_or("")
                        .contains("or disjoint");
                    LLVMDisposeMessage(instruction_string);
                    if !is_or_disjoint {
                        return false;
                    }
                }

                let lhs = LLVMGetOperand(*instruction, 0);
                let rhs = LLVMGetOperand(*instruction, 1);

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
