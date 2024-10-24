use std::collections::HashMap;

use gitql_core::signature::Function;
use gitql_core::signature::Signature;
use gitql_core::values::base::Value;

use crate::ir::types::InstMatcherType;

use crate::ir::values::InstMatcherValue;

use crate::matchers::instruction_matcher::ArithmeticInstMatcher;

#[inline(always)]
pub fn register_arithmetic_matchers_functions(map: &mut HashMap<&'static str, Function>) {
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
    map.insert(
        "m_add",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_sub",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_mul",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_div",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_rem",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_c_add",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_c_sub",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_c_mul",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_c_div",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_c_rem",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_fadd",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_fsub",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_fmul",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_fdiv",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_frem",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_c_fadd",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_c_fsub",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_c_fmul",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_c_fdiv",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_c_frem",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );
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
        matcher: ArithmeticInstMatcher::create_add(lhs_matcher, rhs_matcher),
    })
}

fn match_sub_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
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
        matcher: ArithmeticInstMatcher::create_sub(lhs_matcher, rhs_matcher),
    })
}

fn match_mul_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
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
        matcher: ArithmeticInstMatcher::create_mul(lhs_matcher, rhs_matcher),
    })
}

fn match_div_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
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
        matcher: ArithmeticInstMatcher::create_div(lhs_matcher, rhs_matcher),
    })
}

fn match_rem_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
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
        matcher: ArithmeticInstMatcher::create_div(lhs_matcher, rhs_matcher),
    })
}

fn match_commutatively_add_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
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
        matcher: ArithmeticInstMatcher::create_commutatively_add(lhs_matcher, rhs_matcher),
    })
}

fn match_commutatively_sub_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
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
        matcher: ArithmeticInstMatcher::create_commutatively_sub(lhs_matcher, rhs_matcher),
    })
}

fn match_commutatively_mul_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
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
        matcher: ArithmeticInstMatcher::create_commutatively_mul(lhs_matcher, rhs_matcher),
    })
}

fn match_commutatively_div_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
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
        matcher: ArithmeticInstMatcher::create_commutatively_div(lhs_matcher, rhs_matcher),
    })
}

fn match_commutatively_rem_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
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
        matcher: ArithmeticInstMatcher::create_commutatively_rem(lhs_matcher, rhs_matcher),
    })
}

fn match_float_add_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
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
        matcher: ArithmeticInstMatcher::create_float_add(lhs_matcher, rhs_matcher),
    })
}

fn match_float_sub_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
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
        matcher: ArithmeticInstMatcher::create_float_sub(lhs_matcher, rhs_matcher),
    })
}

fn match_float_mul_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
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
        matcher: ArithmeticInstMatcher::create_float_mul(lhs_matcher, rhs_matcher),
    })
}

fn match_float_div_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
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
        matcher: ArithmeticInstMatcher::create_float_div(lhs_matcher, rhs_matcher),
    })
}

fn match_float_rem_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
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
        matcher: ArithmeticInstMatcher::create_float_div(lhs_matcher, rhs_matcher),
    })
}

fn match_commutatively_float_add_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
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
        matcher: ArithmeticInstMatcher::create_commutatively_float_add(lhs_matcher, rhs_matcher),
    })
}

fn match_commutatively_float_sub_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
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
        matcher: ArithmeticInstMatcher::create_commutatively_float_sub(lhs_matcher, rhs_matcher),
    })
}

fn match_commutatively_float_mul_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
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
        matcher: ArithmeticInstMatcher::create_commutatively_float_mul(lhs_matcher, rhs_matcher),
    })
}

fn match_commutatively_float_div_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
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
        matcher: ArithmeticInstMatcher::create_commutatively_float_div(lhs_matcher, rhs_matcher),
    })
}

fn match_commutatively_float_rem_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
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
        matcher: ArithmeticInstMatcher::create_commutatively_float_rem(lhs_matcher, rhs_matcher),
    })
}
