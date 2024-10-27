use std::collections::HashMap;

use gitql_core::signature::Function;
use gitql_core::signature::Signature;
use gitql_core::values::base::Value;

use crate::ir::values::InstMatcherValue;
use crate::matchers::instruction_matcher::FloatComparisonInstMatcher;

use super::binary_matcher_signature;
use super::binary_matchers_sides;

#[inline(always)]
pub fn register_float_comparisons_matchers_functions(map: &mut HashMap<&'static str, Function>) {
    map.insert("m_fcmp_eq", match_fcmp_eq_inst);
    map.insert("m_fcmp_ne", match_fcmp_ne_inst);
    map.insert("m_fcmp_gt", match_fcmp_gt_inst);
    map.insert("m_fcmp_ge", match_fcmp_ge_inst);
    map.insert("m_fcmp_lt", match_fcmp_lt_inst);
    map.insert("m_fcmp_le", match_fcmp_le_inst);
    map.insert("m_fcmp_ord", match_fcmp_ord_inst);
    map.insert("m_fcmp_uno", match_fcmp_uno_inst);
    map.insert("m_fcmp_ueq", match_fcmp_ueg_inst);
    map.insert("m_fcmp_une", match_fcmp_une_inst);
    map.insert("m_fcmp_ugt", match_fcmp_ugt_inst);
    map.insert("m_fcmp_uge", match_fcmp_uge_inst);
    map.insert("m_fcmp_ult", match_fcmp_ult_inst);
    map.insert("m_fcmp_ule", match_fcmp_ule_inst);

    map.insert("m_c_fcmp_eq", match_commutatively_fcmp_eq_inst);
    map.insert("m_c_fcmp_ne", match_commutatively_fcmp_ne_inst);
    map.insert("m_c_fcmp_gt", match_commutatively_fcmp_gt_inst);
    map.insert("m_c_fcmp_ge", match_commutatively_fcmp_ge_inst);
    map.insert("m_c_fcmp_lt", match_commutatively_fcmp_lt_inst);
    map.insert("m_c_fcmp_le", match_commutatively_fcmp_le_inst);
    map.insert("m_c_fcmp_ord", match_commutatively_fcmp_ord_inst);
    map.insert("m_c_fcmp_uno", match_commutatively_fcmp_uno_inst);
    map.insert("m_c_fcmp_ueq", match_commutatively_fcmp_ueg_inst);
    map.insert("m_c_fcmp_une", match_commutatively_fcmp_une_inst);
    map.insert("m_c_fcmp_ugt", match_commutatively_fcmp_ugt_inst);
    map.insert("m_c_fcmp_uge", match_commutatively_fcmp_uge_inst);
    map.insert("m_c_fcmp_ult", match_commutatively_fcmp_ult_inst);
    map.insert("m_c_fcmp_ule", match_commutatively_fcmp_ule_inst);
}

#[inline(always)]
pub fn register_float_comparisons_matchers_function_signatures(
    map: &mut HashMap<&'static str, Signature>,
) {
    map.insert("m_fcmp_eq", binary_matcher_signature());
    map.insert("m_fcmp_ne", binary_matcher_signature());
    map.insert("m_fcmp_gt", binary_matcher_signature());
    map.insert("m_fcmp_ge", binary_matcher_signature());
    map.insert("m_fcmp_lt", binary_matcher_signature());
    map.insert("m_fcmp_le", binary_matcher_signature());
    map.insert("m_fcmp_ord", binary_matcher_signature());
    map.insert("m_fcmp_uno", binary_matcher_signature());
    map.insert("m_fcmp_ueq", binary_matcher_signature());
    map.insert("m_fcmp_une", binary_matcher_signature());
    map.insert("m_fcmp_ugt", binary_matcher_signature());
    map.insert("m_fcmp_uge", binary_matcher_signature());
    map.insert("m_fcmp_ult", binary_matcher_signature());
    map.insert("m_fcmp_ule", binary_matcher_signature());

    map.insert("m_c_fcmp_eq", binary_matcher_signature());
    map.insert("m_c_fcmp_ne", binary_matcher_signature());
    map.insert("m_c_fcmp_gt", binary_matcher_signature());
    map.insert("m_c_fcmp_ge", binary_matcher_signature());
    map.insert("m_c_fcmp_lt", binary_matcher_signature());
    map.insert("m_c_fcmp_le", binary_matcher_signature());
    map.insert("m_c_fcmp_ord", binary_matcher_signature());
    map.insert("m_c_fcmp_uno", binary_matcher_signature());
    map.insert("m_c_fcmp_ueq", binary_matcher_signature());
    map.insert("m_c_fcmp_une", binary_matcher_signature());
    map.insert("m_c_fcmp_ugt", binary_matcher_signature());
    map.insert("m_c_fcmp_uge", binary_matcher_signature());
    map.insert("m_c_fcmp_ult", binary_matcher_signature());
    map.insert("m_c_fcmp_ule", binary_matcher_signature());
}

fn match_fcmp_eq_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = FloatComparisonInstMatcher::create_fcmp_eq(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_fcmp_ne_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = FloatComparisonInstMatcher::create_fcmp_ne(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_fcmp_gt_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = FloatComparisonInstMatcher::create_fcmp_gt(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_fcmp_ge_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = FloatComparisonInstMatcher::create_fcmp_ge(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_fcmp_lt_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = FloatComparisonInstMatcher::create_fcmp_lt(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_fcmp_le_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = FloatComparisonInstMatcher::create_fcmp_le(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_fcmp_ord_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = FloatComparisonInstMatcher::create_fcmp_ord(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_fcmp_uno_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = FloatComparisonInstMatcher::create_fcmp_uno(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_fcmp_ueg_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = FloatComparisonInstMatcher::create_fcmp_ueq(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_fcmp_une_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = FloatComparisonInstMatcher::create_fcmp_une(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_fcmp_ugt_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = FloatComparisonInstMatcher::create_fcmp_ugt(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_fcmp_uge_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = FloatComparisonInstMatcher::create_fcmp_uge(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_fcmp_ult_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = FloatComparisonInstMatcher::create_fcmp_ult(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_fcmp_ule_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = FloatComparisonInstMatcher::create_fcmp_ule(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_commutatively_fcmp_eq_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher =
        FloatComparisonInstMatcher::create_commutatively_fcmp_eq(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_commutatively_fcmp_ne_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher =
        FloatComparisonInstMatcher::create_commutatively_fcmp_ne(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_commutatively_fcmp_gt_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher =
        FloatComparisonInstMatcher::create_commutatively_fcmp_gt(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_commutatively_fcmp_ge_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher =
        FloatComparisonInstMatcher::create_commutatively_fcmp_ge(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_commutatively_fcmp_lt_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher =
        FloatComparisonInstMatcher::create_commutatively_fcmp_lt(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_commutatively_fcmp_le_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher =
        FloatComparisonInstMatcher::create_commutatively_fcmp_le(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_commutatively_fcmp_ord_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher =
        FloatComparisonInstMatcher::create_commutatively_fcmp_ord(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_commutatively_fcmp_uno_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher =
        FloatComparisonInstMatcher::create_commutatively_fcmp_uno(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_commutatively_fcmp_ueg_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher =
        FloatComparisonInstMatcher::create_commutatively_fcmp_ueq(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_commutatively_fcmp_une_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher =
        FloatComparisonInstMatcher::create_commutatively_fcmp_une(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_commutatively_fcmp_ugt_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher =
        FloatComparisonInstMatcher::create_commutatively_fcmp_ugt(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_commutatively_fcmp_uge_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher =
        FloatComparisonInstMatcher::create_commutatively_fcmp_uge(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_commutatively_fcmp_ult_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher =
        FloatComparisonInstMatcher::create_commutatively_fcmp_ult(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_commutatively_fcmp_ule_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher =
        FloatComparisonInstMatcher::create_commutatively_fcmp_ule(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}
