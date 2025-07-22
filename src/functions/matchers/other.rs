use std::collections::HashMap;

use gitql_ast::types::boolean::BoolType;
use gitql_ast::types::integer::IntType;
use gitql_ast::types::optional::OptionType;
use gitql_ast::types::text::TextType;
use gitql_ast::types::varargs::VarargsType;
use gitql_core::signature::Signature;
use gitql_core::signature::StandardFunction;
use gitql_core::values::array::ArrayValue;
use gitql_core::values::boolean::BoolValue;
use gitql_core::values::Value;

use crate::ir::types::InstMatcherType;
use crate::ir::types::LLVMInstType;
use crate::ir::types::TypeMatcherType;
use crate::ir::values::InstMatcherValue;
use crate::ir::values::LLVMInstValue;
use crate::ir::values::TypeMatcherValue;
use crate::matchers::other::AnyInstMatcher;
use crate::matchers::other::ArgumentMatcher;
use crate::matchers::other::ExtractValueInstMatcher;
use crate::matchers::other::InstTypeMatcher;
use crate::matchers::other::LabelInstMatcher;
use crate::matchers::other::OperandCountMatcher;
use crate::matchers::other::PoisonValueMatcher;
use crate::matchers::other::ReturnInstMatcher;
use crate::matchers::other::UnreachableInstMatcher;

#[inline(always)]
pub fn register_other_inst_matchers_functions(map: &mut HashMap<&'static str, StandardFunction>) {
    map.insert("m_inst", match_inst);
    map.insert("m_extract_value", match_extract_value);
    map.insert("m_inst_type", match_inst_type);
    map.insert("m_any_inst", match_any_inst);
    map.insert("m_poison", match_poison_inst);
    map.insert("m_label", match_label_inst);
    map.insert("m_argument", match_argument_inst);
    map.insert("m_return", match_return_inst);
    map.insert("m_unreachable", match_unreachable_inst);
    map.insert("m_operands_number", match_operands_number);
}

#[inline(always)]
pub fn register_other_inst_matchers_function_signatures(
    map: &mut HashMap<&'static str, Signature>,
) {
    map.insert(
        "m_inst",
        Signature {
            parameters: vec![Box::new(LLVMInstType), Box::new(InstMatcherType)],
            return_type: Box::new(BoolType),
        },
    );

    map.insert(
        "m_extract_value",
        Signature {
            parameters: vec![
                Box::new(OptionType::new(Some(Box::new(InstMatcherType)))),
                Box::new(VarargsType::new(Box::new(IntType))),
            ],
            return_type: Box::new(InstMatcherType),
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
        "m_poison",
        Signature {
            parameters: vec![],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_label",
        Signature {
            parameters: vec![Box::new(OptionType {
                base: Some(Box::new(TextType)),
            })],
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
        "m_argument",
        Signature {
            parameters: vec![
                Box::new(OptionType {
                    base: Some(Box::new(TextType)),
                }),
                Box::new(OptionType {
                    base: Some(Box::new(TypeMatcherType)),
                }),
            ],
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

    map.insert(
        "m_operands_number",
        Signature::with_return(Box::new(InstMatcherType)).add_parameter(Box::new(IntType)),
    );
}

fn match_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let inst = values[0].as_any().downcast_ref::<LLVMInstValue>().unwrap();
    let matcher = values[1]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap();

    let is_match = matcher.matcher.is_match(&inst.llvm_value);
    Box::new(BoolValue { value: is_match })
}

fn match_extract_value(values: &[Box<dyn Value>]) -> Box<dyn Value> {
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

    let mut indices: Option<Vec<i64>> = None;
    if let Some(indices_array) = values.get(1) {
        let array = indices_array.as_any().downcast_ref::<ArrayValue>().unwrap();
        let mut i64_indices: Vec<i64> = Vec::with_capacity(array.values.len());
        for element in array.values.iter() {
            i64_indices.push(element.as_int().unwrap());
        }
        indices = Some(i64_indices);
    }

    let inst_matcher: Box<ExtractValueInstMatcher> =
        Box::new(ExtractValueInstMatcher { matcher, indices });

    Box::new(InstMatcherValue {
        matcher: inst_matcher,
    })
}

fn match_inst_type(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let matcher = values[0]
        .as_any()
        .downcast_ref::<TypeMatcherValue>()
        .unwrap()
        .matcher
        .clone();
    let inst_matcher = Box::new(InstTypeMatcher { matcher });

    Box::new(InstMatcherValue {
        matcher: inst_matcher,
    })
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
        matcher: Box::new(PoisonValueMatcher),
    })
}

fn match_argument_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let name = if values.is_empty() {
        None
    } else {
        values[0].as_text()
    };

    let type_matcher = if values.len() == 2 {
        Some(
            values[1]
                .as_any()
                .downcast_ref::<TypeMatcherValue>()
                .unwrap()
                .matcher
                .clone(),
        )
    } else {
        None
    };

    Box::new(InstMatcherValue {
        matcher: Box::new(ArgumentMatcher { name, type_matcher }),
    })
}

fn match_unreachable_inst(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let matcher = Box::new(UnreachableInstMatcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_operands_number(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let expected_number = values[0].as_int().unwrap() as i32;
    let matcher = Box::new(OperandCountMatcher::has_n_operands(expected_number));
    Box::new(InstMatcherValue { matcher })
}
