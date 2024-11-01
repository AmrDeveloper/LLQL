use std::collections::HashMap;

use gitql_ast::types::boolean::BoolType;
use gitql_ast::types::optional::OptionType;
use gitql_core::signature::Function;
use gitql_core::signature::Signature;
use gitql_core::values::base::Value;
use gitql_core::values::boolean::BoolValue;

use crate::ir::types::InstMatcherType;
use crate::ir::types::LLVMInstType;
use crate::ir::values::InstMatcherValue;
use crate::ir::values::LLVMInstValue;
use crate::matchers::instruction_matcher::AnyInstMatcher;
use crate::matchers::instruction_matcher::ConstIntMatcher;
use crate::matchers::instruction_matcher::LabelInstMatcher;
use crate::matchers::instruction_matcher::PoisonInstMatcher;
use crate::matchers::instruction_matcher::ReturnInstMatcher;
use crate::matchers::instruction_matcher::UnreachableInstMatcher;

use super::arithmetic_matchers::register_arithmetic_matchers_function_signatures;
use super::arithmetic_matchers::register_arithmetic_matchers_functions;
use super::fcmp_matchers::register_float_comparisons_matchers_function_signatures;
use super::fcmp_matchers::register_float_comparisons_matchers_functions;
use super::icmp_matchers::register_int_comparisons_matchers_function_signatures;
use super::icmp_matchers::register_int_comparisons_matchers_functions;

#[inline(always)]
pub fn register_inst_matchers_functions(map: &mut HashMap<&'static str, Function>) {
    map.insert("m_inst", match_inst);
    map.insert("m_any_inst", match_any_inst);
    map.insert("m_const_int", match_const_int_inst);
    map.insert("m_poison", match_poison_inst);
    map.insert("m_label", match_label_inst);
    map.insert("m_return", match_return_inst);
    map.insert("m_unreachable", match_unreachable_inst);

    register_arithmetic_matchers_functions(map);
    register_int_comparisons_matchers_functions(map);
    register_float_comparisons_matchers_functions(map);
}

#[inline(always)]
pub fn register_inst_matchers_function_signatures(map: &mut HashMap<&'static str, Signature>) {
    map.insert(
        "m_inst",
        Signature {
            parameters: vec![Box::new(LLVMInstType), Box::new(InstMatcherType)],
            return_type: Box::new(BoolType),
        },
    );
    map.insert(
        "m_any_inst",
        Signature {
            parameters: vec![],
            return_type: Box::new(InstMatcherType),
        },
    );
    map.insert(
        "m_return",
        Signature {
            parameters: vec![Box::new(OptionType {
                base: Some(Box::new(InstMatcherType)),
            })],
            return_type: Box::new(InstMatcherType),
        },
    );
    map.insert(
        "m_unreachable",
        Signature {
            parameters: vec![],
            return_type: Box::new(InstMatcherType),
        },
    );

    register_arithmetic_matchers_function_signatures(map);
    register_int_comparisons_matchers_function_signatures(map);
    register_float_comparisons_matchers_function_signatures(map);
}

fn match_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let inst = values[0].as_any().downcast_ref::<LLVMInstValue>().unwrap();
    let matcher = values[1]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap();

    let is_match = matcher.matcher.is_match(inst.llvm_value);
    Box::new(BoolValue { value: is_match })
}

fn match_any_inst(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    Box::new(InstMatcherValue {
        matcher: Box::new(AnyInstMatcher),
    })
}

fn match_return_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let matcher = if values.is_empty() {
        Box::new(AnyInstMatcher)
    } else {
        values[0]
            .as_any()
            .downcast_ref::<InstMatcherValue>()
            .unwrap()
            .matcher
            .clone()
    };

    Box::new(InstMatcherValue {
        matcher: Box::new(ReturnInstMatcher { matcher }),
    })
}

fn match_label_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let name = if values.is_empty() {
        None
    } else {
        values[0].as_text()
    };

    Box::new(InstMatcherValue {
        matcher: Box::new(LabelInstMatcher { name }),
    })
}

fn match_poison_inst(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    Box::new(InstMatcherValue {
        matcher: Box::new(PoisonInstMatcher),
    })
}

fn match_const_int_inst(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    Box::new(InstMatcherValue {
        matcher: Box::new(ConstIntMatcher),
    })
}

fn match_unreachable_inst(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    Box::new(InstMatcherValue {
        matcher: Box::new(UnreachableInstMatcher),
    })
}
