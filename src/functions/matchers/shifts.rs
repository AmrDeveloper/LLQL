use std::collections::HashMap;

use gitql_core::signature::Signature;
use gitql_core::signature::StandardFunction;
use gitql_core::values::Value;

use crate::functions::binary_matcher_signature;
use crate::functions::binary_matchers_sides;
use crate::ir::values::InstMatcherValue;
use crate::matchers::binary::BinaryInstMatcher;

#[inline(always)]
pub fn register_shift_matchers_functions(map: &mut HashMap<&'static str, StandardFunction>) {
    map.insert("m_shl", match_shl_inst);
    map.insert("m_shr", match_shr_inst);
    map.insert("m_ashr", match_arithmetic_shr_inst);

    map.insert("m_c_shl", match_commutatively_shl_inst);
    map.insert("m_c_shr", match_commutatively_shr_inst);
    map.insert("m_c_ashr", match_commutatively_arithmetic_shr_inst);
}

#[inline(always)]
pub fn register_shift_matchers_function_signatures(map: &mut HashMap<&'static str, Signature>) {
    map.insert("m_shl", binary_matcher_signature());
    map.insert("m_shr", binary_matcher_signature());
    map.insert("m_ashr", binary_matcher_signature());

    map.insert("m_c_shl", binary_matcher_signature());
    map.insert("m_c_shr", binary_matcher_signature());
    map.insert("m_c_ashr", binary_matcher_signature());
}

fn match_shl_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = BinaryInstMatcher::create_logical_shl(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_shr_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = BinaryInstMatcher::create_logical_shr(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_arithmetic_shr_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = BinaryInstMatcher::create_arithmetic_shr(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_commutatively_shl_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = BinaryInstMatcher::create_commutatively_logical_shl(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_commutatively_shr_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = BinaryInstMatcher::create_commutatively_logical_shr(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_commutatively_arithmetic_shr_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = BinaryInstMatcher::create_commutatively_arithmetic_shr(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}
