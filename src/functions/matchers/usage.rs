use std::collections::HashMap;

use gitql_ast::types::integer::IntType;
use gitql_core::signature::Signature;
use gitql_core::signature::StandardFunction;
use gitql_core::values::base::Value;

use crate::ir::types::InstMatcherType;
use crate::ir::values::InstMatcherValue;
use crate::matchers::usage::UsageInstMatcher;

#[inline(always)]
pub fn register_usage_matchers_functions(map: &mut HashMap<&'static str, StandardFunction>) {
    map.insert("m_unused", match_unused);
    map.insert("m_has_one_use", match_has_one_use);
    map.insert("m_has_n_uses", match_has_n_uses);
}

#[inline(always)]
pub fn register_usage_matchers_function_signatures(map: &mut HashMap<&'static str, Signature>) {
    map.insert(
        "m_unused",
        Signature {
            parameters: vec![Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_has_one_use",
        Signature {
            parameters: vec![Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_has_n_uses",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(IntType)],
            return_type: Box::new(InstMatcherType),
        },
    );
}

fn match_unused(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let matcher = values[0]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .clone();

    let usage_matcher = UsageInstMatcher::create_unused_matcher(matcher);
    Box::new(InstMatcherValue {
        matcher: usage_matcher,
    })
}

fn match_has_one_use(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let matcher = values[0]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .clone();

    let usage_matcher = UsageInstMatcher::create_has_one_use_matcher(matcher);
    Box::new(InstMatcherValue {
        matcher: usage_matcher,
    })
}

fn match_has_n_uses(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let matcher = values[0]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .clone();

    let usage_count = values[1].as_int().unwrap() as usize;
    let usage_matcher = UsageInstMatcher::create_has_n_uses_matcher(matcher, usage_count);
    Box::new(InstMatcherValue {
        matcher: usage_matcher,
    })
}
