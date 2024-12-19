use inkwell::llvm_sys::prelude::LLVMValueRef;

use super::InstMatcher;

#[derive(PartialEq, Clone)]
enum CompoundMatcherKind {
    OneOf,
    AllOf,
}

#[derive(Clone)]
pub struct CompoundInstMatcher {
    matchers: Vec<Box<dyn InstMatcher>>,
    matcher_kind: CompoundMatcherKind,
}

impl CompoundInstMatcher {
    pub fn create_one_of(matchers: Vec<Box<dyn InstMatcher>>) -> Self {
        CompoundInstMatcher {
            matchers,
            matcher_kind: CompoundMatcherKind::OneOf,
        }
    }

    pub fn create_all_of(matchers: Vec<Box<dyn InstMatcher>>) -> Self {
        CompoundInstMatcher {
            matchers,
            matcher_kind: CompoundMatcherKind::AllOf,
        }
    }
}

impl InstMatcher for CompoundInstMatcher {
    fn is_match(&self, instruction: LLVMValueRef) -> bool {
        let mut matches_count = 0;
        for matcher in self.matchers.iter() {
            let is_matches = matcher.is_match(instruction);

            // If kind is `oneOf` and one if matches, return true
            if is_matches && self.matcher_kind == CompoundMatcherKind::OneOf {
                return true;
            }

            // If kind is `allOf` and one is not matches, return false
            if !is_matches && self.matcher_kind == CompoundMatcherKind::AllOf {
                return false;
            }

            if is_matches {
                matches_count += 1;
            }
        }

        match self.matcher_kind {
            CompoundMatcherKind::OneOf => matches_count > 1,
            CompoundMatcherKind::AllOf => matches_count == self.matchers.len(),
        }
    }
}
