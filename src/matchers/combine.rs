use inkwell::llvm_sys::prelude::LLVMValueRef;

use super::InstMatcher;

#[derive(PartialEq, Clone)]
enum CombineMatcherKind {
    OneOf,
    AllOf,
}

#[derive(Clone)]
pub struct CombineInstMatcher {
    matchers: Vec<Box<dyn InstMatcher>>,
    matcher_kind: CombineMatcherKind,
}

impl CombineInstMatcher {
    pub fn create_one_of(matchers: Vec<Box<dyn InstMatcher>>) -> Self {
        CombineInstMatcher {
            matchers,
            matcher_kind: CombineMatcherKind::OneOf,
        }
    }

    pub fn create_all_of(matchers: Vec<Box<dyn InstMatcher>>) -> Self {
        CombineInstMatcher {
            matchers,
            matcher_kind: CombineMatcherKind::AllOf,
        }
    }
}

impl InstMatcher for CombineInstMatcher {
    fn is_match(&self, instruction: LLVMValueRef) -> bool {
        let mut matches_count = 0;
        for matcher in self.matchers.iter() {
            let is_matches = matcher.is_match(instruction);

            // If kind is `oneOf` and one if matches, return true
            if is_matches && self.matcher_kind == CombineMatcherKind::OneOf {
                return true;
            }

            // If kind is `allOf` and one is not matches, return false
            if !is_matches && self.matcher_kind == CombineMatcherKind::AllOf {
                return false;
            }

            if is_matches {
                matches_count += 1;
            }
        }

        match self.matcher_kind {
            CombineMatcherKind::OneOf => matches_count > 1,
            CombineMatcherKind::AllOf => matches_count == self.matchers.len(),
        }
    }
}
