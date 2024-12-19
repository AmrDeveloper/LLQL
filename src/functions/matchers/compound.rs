use std::collections::HashMap;

use gitql_ast::types::varargs::VarargsType;
use gitql_core::signature::Signature;
use gitql_core::signature::StandardFunction;
use gitql_core::values::base::Value;

use crate::ir::types::InstMatcherType;
use crate::ir::values::InstMatcherValue;
use crate::matchers::compound::CompoundInstMatcher;
use crate::matchers::InstMatcher;

#[inline(always)]
pub fn register_compound_matchers_function(map: &mut HashMap<&'static str, StandardFunction>) {
    map.insert("m_inst_oneof", match_oneof_compound_inst);
    map.insert("m_inst_allof", match_allof_compound_inst);
}

#[inline(always)]
pub fn register_compound_matchers_function_signatures(map: &mut HashMap<&'static str, Signature>) {
    map.insert(
        "m_inst_oneof",
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
        "m_inst_allof",
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
}

fn match_oneof_compound_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let mut matchers: Vec<Box<dyn InstMatcher>> = vec![];
    for value in values.iter() {
        if let Some(inst_matcher) = value.as_any().downcast_ref::<InstMatcherValue>() {
            matchers.push(inst_matcher.matcher.to_owned());
        }
    }
    let matcher = Box::new(CompoundInstMatcher::create_one_of(matchers));
    Box::new(InstMatcherValue { matcher })
}

fn match_allof_compound_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let mut matchers: Vec<Box<dyn InstMatcher>> = vec![];
    for value in values.iter() {
        if let Some(inst_matcher) = value.as_any().downcast_ref::<InstMatcherValue>() {
            matchers.push(inst_matcher.matcher.to_owned());
        }
    }
    let matcher = Box::new(CompoundInstMatcher::create_all_of(matchers));
    Box::new(InstMatcherValue { matcher })
}
