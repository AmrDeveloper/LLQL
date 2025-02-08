use std::collections::HashMap;

use gitql_core::signature::Signature;
use gitql_core::signature::StandardFunction;
use gitql_core::values::base::Value;

use crate::functions::binary_matcher_signature;
use crate::functions::binary_matchers_sides;
use crate::ir::values::InstMatcherValue;
use crate::matchers::binary::BinaryInstMatcher;

#[inline(always)]
pub fn register_binary_inst_matchers_functions(map: &mut HashMap<&'static str, StandardFunction>) {
    map.insert("m_binop", match_binary_inst);
    map.insert("m_or", match_or_inst);
    map.insert("m_or_disjoint", match_or_disjoint_inst);
    map.insert("m_and", match_and_inst);
    map.insert("m_xor", match_xor_inst);

    map.insert("m_c_binop", match_commutatively_binary_inst);
    map.insert("m_c_or", match_commutatively_or_inst);
    map.insert("m_c_or_disjoint", match_commutatively_or_disjoint_inst);
    map.insert("m_c_and", match_commutatively_and_inst);
    map.insert("m_c_xor", match_commutatively_xor_inst);
}

#[inline(always)]
pub fn register_binary_inst_matchers_function_signatures(
    map: &mut HashMap<&'static str, Signature>,
) {
    map.insert("m_binop", binary_matcher_signature());
    map.insert("m_or", binary_matcher_signature());
    map.insert("m_or_disjoint", binary_matcher_signature());
    map.insert("m_and", binary_matcher_signature());
    map.insert("m_xor", binary_matcher_signature());

    map.insert("m_c_binop", binary_matcher_signature());
    map.insert("m_c_or", binary_matcher_signature());
    map.insert("m_c_or_disjoint", binary_matcher_signature());
    map.insert("m_c_and", binary_matcher_signature());
    map.insert("m_c_xor", binary_matcher_signature());
}

fn match_or_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = BinaryInstMatcher::create_or(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_or_disjoint_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = BinaryInstMatcher::create_or_disjoint(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_and_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = BinaryInstMatcher::create_and(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_xor_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = BinaryInstMatcher::create_xor(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_commutatively_or_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = BinaryInstMatcher::create_commutatively_or(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_commutatively_or_disjoint_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = BinaryInstMatcher::create_commutatively_or_disjoint(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_commutatively_and_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = BinaryInstMatcher::create_commutatively_and(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_commutatively_xor_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = BinaryInstMatcher::create_commutatively_xor(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_binary_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = BinaryInstMatcher::create_any(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_commutatively_binary_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = BinaryInstMatcher::create_commutatively_any(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}
