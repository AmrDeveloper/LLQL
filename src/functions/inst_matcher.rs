use gitql_ast::types::boolean::BoolType;
use gitql_ast::types::optional::OptionType;
use gitql_core::signature::Function;
use gitql_core::signature::Signature;
use gitql_core::values::base::Value;
use gitql_core::values::boolean::BoolValue;
use inkwell::llvm_sys::LLVMOpcode;
use std::collections::HashMap;

use crate::ir::types::InstMatcherType;
use crate::ir::types::LLVMInstType;
use crate::ir::types::TypeMatcherType;
use crate::ir::values::InstMatcherValue;
use crate::ir::values::LLVMInstValue;
use crate::ir::values::TypeMatcherValue;
use crate::matchers::instruction_matcher::AnyInstMatcher;
use crate::matchers::instruction_matcher::BinaryInstMatcher;
use crate::matchers::instruction_matcher::ReturnInstMatcher;
use crate::matchers::instruction_matcher::UnreachableInstMatcher;
use crate::matchers::type_matcher::AnyTypeMatcher;

#[inline(always)]
pub fn register_inst_matchers_functions(map: &mut HashMap<&'static str, Function>) {
    map.insert("m_inst", match_inst);
    map.insert("m_any_inst", match_any_inst);
    map.insert("m_add", match_add_inst);
    map.insert("m_return", match_return_inst);
    map.insert("m_unreachable", match_unreachable_inst);
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
        "m_add",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );
    map.insert(
        "m_return",
        Signature {
            parameters: vec![Box::new(OptionType {
                base: Some(Box::new(TypeMatcherType)),
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

fn match_add_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let lhs_matcher = values[0]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    let rhs_matcher = values[1]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    Box::new(InstMatcherValue {
        matcher: Box::new(BinaryInstMatcher {
            opcode: LLVMOpcode::LLVMAdd,
            lhs_matcher,
            rhs_matcher,
        }),
    })
}

fn match_return_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let type_matcher = if values.is_empty() {
        Box::new(AnyTypeMatcher)
    } else {
        values[0]
            .as_any()
            .downcast_ref::<TypeMatcherValue>()
            .unwrap()
            .matcher
            .clone()
    };

    Box::new(InstMatcherValue {
        matcher: Box::new(ReturnInstMatcher { type_matcher }),
    })
}

fn match_unreachable_inst(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    Box::new(InstMatcherValue {
        matcher: Box::new(UnreachableInstMatcher),
    })
}
