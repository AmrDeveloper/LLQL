use std::collections::HashMap;

use gitql_ast::types::varargs::VarargsType;
use gitql_core::signature::Signature;
use gitql_core::signature::StandardFunction;
use gitql_core::values::base::Value;
use inkwell::llvm_sys::prelude::LLVMValueRef;

use crate::functions::binary_matchers_sides;
use crate::ir::types::InstMatcherType;
use crate::ir::values::InstMatcherValue;
use crate::matchers::combine::CombineBinaryInstMatcher;
use crate::matchers::combine::CombineInstMatcher;
use crate::matchers::combine::CombineUnaryInstMatcher;
use crate::matchers::Matcher;

#[inline(always)]
pub fn register_combine_matchers_function(map: &mut HashMap<&'static str, StandardFunction>) {
    map.insert("m_inst_combine_oneof", match_oneof_combine_inst);
    map.insert("m_inst_combine_allof", match_allof_combine_inst);
    map.insert("m_inst_combine_noneof", match_noneof_combine_inst);

    map.insert("m_inst_combine_and", match_and_combine_inst);
    map.insert("m_inst_combine_or", match_or_combine_inst);
    map.insert("m_inst_combine_xor", match_xor_combine_inst);

    map.insert("m_inst_combine_not", match_not_combine_inst);
}

#[inline(always)]
pub fn register_combine_matchers_function_signatures(map: &mut HashMap<&'static str, Signature>) {
    map.insert(
        "m_inst_combine_oneof",
        Signature {
            parameters: vec![
                Box::new(InstMatcherType),
                Box::new(VarargsType {
                    base: Box::new(InstMatcherType),
                }),
            ],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_inst_combine_allof",
        Signature {
            parameters: vec![
                Box::new(InstMatcherType),
                Box::new(VarargsType {
                    base: Box::new(InstMatcherType),
                }),
            ],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_inst_combine_noneof",
        Signature {
            parameters: vec![
                Box::new(InstMatcherType),
                Box::new(VarargsType {
                    base: Box::new(InstMatcherType),
                }),
            ],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_inst_combine_and",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_inst_combine_or",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_inst_combine_xor",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_inst_combine_not",
        Signature {
            parameters: vec![Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );
}

fn match_oneof_combine_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let mut matchers: Vec<Box<dyn Matcher<LLVMValueRef>>> = vec![];
    for value in values.iter() {
        if let Some(inst_matcher) = value.as_any().downcast_ref::<InstMatcherValue>() {
            matchers.push(inst_matcher.matcher.to_owned());
        }
    }
    let matcher = Box::new(CombineInstMatcher::create_one_of(matchers));
    Box::new(InstMatcherValue { matcher })
}

fn match_allof_combine_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let mut matchers: Vec<Box<dyn Matcher<LLVMValueRef>>> = vec![];
    for value in values.iter() {
        if let Some(inst_matcher) = value.as_any().downcast_ref::<InstMatcherValue>() {
            matchers.push(inst_matcher.matcher.to_owned());
        }
    }
    let matcher = Box::new(CombineInstMatcher::create_all_of(matchers));
    Box::new(InstMatcherValue { matcher })
}

fn match_noneof_combine_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let mut matchers: Vec<Box<dyn Matcher<LLVMValueRef>>> = vec![];
    for value in values.iter() {
        if let Some(inst_matcher) = value.as_any().downcast_ref::<InstMatcherValue>() {
            matchers.push(inst_matcher.matcher.to_owned());
        }
    }
    let matcher = Box::new(CombineInstMatcher::create_none_of(matchers));
    Box::new(InstMatcherValue { matcher })
}

fn match_and_combine_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs, rhs) = binary_matchers_sides(values);
    let matcher = Box::new(CombineBinaryInstMatcher::create_and(lhs, rhs));
    Box::new(InstMatcherValue { matcher })
}

fn match_or_combine_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs, rhs) = binary_matchers_sides(values);
    let matcher = Box::new(CombineBinaryInstMatcher::create_or(lhs, rhs));
    Box::new(InstMatcherValue { matcher })
}

fn match_xor_combine_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let (lhs, rhs) = binary_matchers_sides(values);
    let matcher = Box::new(CombineBinaryInstMatcher::create_xor(lhs, rhs));
    Box::new(InstMatcherValue { matcher })
}

fn match_not_combine_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let rhs = values[0]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();
    let matcher = Box::new(CombineUnaryInstMatcher::create_not(rhs));
    Box::new(InstMatcherValue { matcher })
}
