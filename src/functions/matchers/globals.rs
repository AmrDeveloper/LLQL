use std::collections::HashMap;

use gitql_core::signature::Signature;
use gitql_core::signature::StandardFunction;
use gitql_core::values::Value;

use crate::functions::matcher_signature_without_parameters;
use crate::ir::values::InstMatcherValue;
use crate::matchers::globals::GlobalValueExprMatcher;
use crate::matchers::globals::GlobalVariableExprMatcher;

#[inline(always)]
pub fn register_globals_matchers_functions(map: &mut HashMap<&'static str, StandardFunction>) {
    map.insert("m_global_var", match_global_variable_expr_inst);
    map.insert("m_global_val", match_global_value_expr_inst);
}

#[inline(always)]
pub fn register_globals_matchers_function_signatures(map: &mut HashMap<&'static str, Signature>) {
    map.insert("m_global_var", matcher_signature_without_parameters());
    map.insert("m_global_val", matcher_signature_without_parameters());
}

fn match_global_variable_expr_inst(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let matcher = Box::new(GlobalVariableExprMatcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_global_value_expr_inst(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let matcher = Box::new(GlobalValueExprMatcher);
    Box::new(InstMatcherValue { matcher })
}
