use std::collections::HashMap;

use gitql_core::signature::Signature;
use gitql_core::signature::StandardFunction;
use gitql_core::values::base::Value;

use crate::functions::binary_matcher_signature;
use crate::functions::binary_matchers_sides;
use crate::ir::values::InstMatcherValue;
use crate::matchers::arithmetic::ArithmeticInstMatcher;

#[inline(always)]
pub fn register_arithmetic_matchers_functions(map: &mut HashMap<&'static str, StandardFunction>) {
    map.insert("m_add", match_add_inst);
    map.insert("m_sub", match_sub_inst);
    map.insert("m_mul", match_mul_inst);
    map.insert("m_div", match_div_inst);
    map.insert("m_rem", match_rem_inst);

    map.insert("m_c_add", match_commutatively_add_inst);
    map.insert("m_c_sub", match_commutatively_sub_inst);
    map.insert("m_c_mul", match_commutatively_mul_inst);
    map.insert("m_c_div", match_commutatively_div_inst);
    map.insert("m_c_rem", match_commutatively_rem_inst);

    map.insert("m_fadd", match_float_add_inst);
    map.insert("m_fsub", match_float_sub_inst);
    map.insert("m_fmul", match_float_mul_inst);
    map.insert("m_fdiv", match_float_div_inst);
    map.insert("m_frem", match_float_rem_inst);

    map.insert("m_c_fadd", match_commutatively_float_add_inst);
    map.insert("m_c_fsub", match_commutatively_float_sub_inst);
    map.insert("m_c_fmul", match_commutatively_float_mul_inst);
    map.insert("m_c_fdiv", match_commutatively_float_div_inst);
    map.insert("m_c_frem", match_commutatively_float_rem_inst);
}

#[inline(always)]
pub fn register_arithmetic_matchers_function_signatures(
    map: &mut HashMap<&'static str, Signature>,
) {
    map.insert("m_add", binary_matcher_signature());
    map.insert("m_sub", binary_matcher_signature());
    map.insert("m_mul", binary_matcher_signature());
    map.insert("m_div", binary_matcher_signature());
    map.insert("m_rem", binary_matcher_signature());

    map.insert("m_c_add", binary_matcher_signature());
    map.insert("m_c_sub", binary_matcher_signature());
    map.insert("m_c_mul", binary_matcher_signature());
    map.insert("m_c_div", binary_matcher_signature());
    map.insert("m_c_rem", binary_matcher_signature());

    map.insert("m_fadd", binary_matcher_signature());
    map.insert("m_fsub", binary_matcher_signature());
    map.insert("m_fmul", binary_matcher_signature());
    map.insert("m_fdiv", binary_matcher_signature());
    map.insert("m_frem", binary_matcher_signature());

    map.insert("m_c_fadd", binary_matcher_signature());
    map.insert("m_c_fsub", binary_matcher_signature());
    map.insert("m_c_fmul", binary_matcher_signature());
    map.insert("m_c_fdiv", binary_matcher_signature());
    map.insert("m_c_frem", binary_matcher_signature());
}

fn match_add_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = ArithmeticInstMatcher::create_add(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_sub_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = ArithmeticInstMatcher::create_sub(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_mul_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = ArithmeticInstMatcher::create_mul(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_div_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = ArithmeticInstMatcher::create_div(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_rem_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = ArithmeticInstMatcher::create_div(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_commutatively_add_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = ArithmeticInstMatcher::create_commutatively_add(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_commutatively_sub_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = ArithmeticInstMatcher::create_commutatively_sub(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_commutatively_mul_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = ArithmeticInstMatcher::create_commutatively_mul(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_commutatively_div_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = ArithmeticInstMatcher::create_commutatively_div(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_commutatively_rem_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = ArithmeticInstMatcher::create_commutatively_rem(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_float_add_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = ArithmeticInstMatcher::create_float_add(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_float_sub_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = ArithmeticInstMatcher::create_float_sub(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_float_mul_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = ArithmeticInstMatcher::create_float_mul(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_float_div_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = ArithmeticInstMatcher::create_float_div(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_float_rem_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = ArithmeticInstMatcher::create_float_div(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_commutatively_float_add_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = ArithmeticInstMatcher::create_commutatively_float_add(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_commutatively_float_sub_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = ArithmeticInstMatcher::create_commutatively_float_sub(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_commutatively_float_mul_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = ArithmeticInstMatcher::create_commutatively_float_mul(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_commutatively_float_div_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = ArithmeticInstMatcher::create_commutatively_float_div(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_commutatively_float_rem_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs_matcher, rhs_matcher) = binary_matchers_sides(values);
    let matcher = ArithmeticInstMatcher::create_commutatively_float_rem(lhs_matcher, rhs_matcher);
    Box::new(InstMatcherValue { matcher })
}
