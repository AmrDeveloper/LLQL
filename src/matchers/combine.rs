use inkwell::llvm_sys::prelude::LLVMValueRef;

use super::Matcher;

#[allow(clippy::enum_variant_names)]
#[derive(PartialEq, Clone)]
enum CombineMatcherKind {
    OneOf,
    AllOf,
    NoneOf,
}

#[derive(Clone)]
pub struct CombineInstMatcher {
    matchers: Vec<Box<dyn Matcher<LLVMValueRef>>>,
    matcher_kind: CombineMatcherKind,
}

impl CombineInstMatcher {
    pub fn create_one_of(matchers: Vec<Box<dyn Matcher<LLVMValueRef>>>) -> Self {
        CombineInstMatcher {
            matchers,
            matcher_kind: CombineMatcherKind::OneOf,
        }
    }

    pub fn create_all_of(matchers: Vec<Box<dyn Matcher<LLVMValueRef>>>) -> Self {
        CombineInstMatcher {
            matchers,
            matcher_kind: CombineMatcherKind::AllOf,
        }
    }

    pub fn create_none_of(matchers: Vec<Box<dyn Matcher<LLVMValueRef>>>) -> Self {
        CombineInstMatcher {
            matchers,
            matcher_kind: CombineMatcherKind::AllOf,
        }
    }
}

impl Matcher<LLVMValueRef> for CombineInstMatcher {
    fn is_match(&self, instruction: &LLVMValueRef) -> bool {
        let mut matches_count = 0;
        let matcher_kind = &self.matcher_kind;
        for matcher in self.matchers.iter() {
            let is_matches = matcher.is_match(instruction);

            // If kind is `oneOf` and one if matches, return true
            if is_matches && CombineMatcherKind::OneOf.eq(matcher_kind) {
                return true;
            }

            // If kind is `allOf` and one is not matches, return false
            if !is_matches && CombineMatcherKind::AllOf.eq(matcher_kind) {
                return false;
            }

            // If kind is `noneOf` and one is matches, return false
            if is_matches && CombineMatcherKind::NoneOf.eq(matcher_kind) {
                return false;
            }

            if is_matches {
                matches_count += 1;
            }
        }

        match self.matcher_kind {
            CombineMatcherKind::OneOf => matches_count > 1,
            CombineMatcherKind::AllOf => matches_count == self.matchers.len(),
            CombineMatcherKind::NoneOf => matches_count == 0,
        }
    }
}

#[derive(PartialEq, Clone)]
enum CombineBinaryMatcherKind {
    And,
    Or,
    Xor,
}

#[derive(Clone)]
pub struct CombineBinaryInstMatcher {
    lhs: Box<dyn Matcher<LLVMValueRef>>,
    rhs: Box<dyn Matcher<LLVMValueRef>>,
    kind: CombineBinaryMatcherKind,
}

impl CombineBinaryInstMatcher {
    pub fn create_and(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Self {
        CombineBinaryInstMatcher {
            lhs,
            rhs,
            kind: CombineBinaryMatcherKind::And,
        }
    }

    pub fn create_or(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Self {
        CombineBinaryInstMatcher {
            lhs,
            rhs,
            kind: CombineBinaryMatcherKind::Or,
        }
    }

    pub fn create_xor(
        lhs: Box<dyn Matcher<LLVMValueRef>>,
        rhs: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Self {
        CombineBinaryInstMatcher {
            lhs,
            rhs,
            kind: CombineBinaryMatcherKind::Xor,
        }
    }
}

impl Matcher<LLVMValueRef> for CombineBinaryInstMatcher {
    fn is_match(&self, instruction: &LLVMValueRef) -> bool {
        match self.kind {
            CombineBinaryMatcherKind::And => {
                self.lhs.is_match(instruction) && self.rhs.is_match(instruction)
            }
            CombineBinaryMatcherKind::Or => {
                self.lhs.is_match(instruction) || self.rhs.is_match(instruction)
            }
            CombineBinaryMatcherKind::Xor => {
                self.lhs.is_match(instruction) ^ self.rhs.is_match(instruction)
            }
        }
    }
}

#[derive(Clone)]
pub struct CombineUnaryInstMatcher {
    rhs: Box<dyn Matcher<LLVMValueRef>>,
}

impl CombineUnaryInstMatcher {
    pub fn create_not(rhs: Box<dyn Matcher<LLVMValueRef>>) -> Self {
        CombineUnaryInstMatcher { rhs }
    }
}

impl Matcher<LLVMValueRef> for CombineUnaryInstMatcher {
    fn is_match(&self, instruction: &LLVMValueRef) -> bool {
        !(self.rhs.is_match(instruction))
    }
}
