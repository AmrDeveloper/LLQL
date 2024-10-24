use std::collections::HashMap;

use gitql_core::signature::Function;
use gitql_core::signature::Signature;
use gitql_core::values::base::Value;

use crate::ir::types::InstMatcherType;
use crate::ir::values::InstMatcherValue;

use crate::matchers::instruction_matcher::IntComparisonInstMatcher;

#[inline(always)]
pub fn register_int_comparisons_matchers_functions(map: &mut HashMap<&'static str, Function>) {
    map.insert("m_icmp_eq", match_icmp_eq_inst);
    map.insert("m_icmp_ne", match_icmp_ne_inst);
    map.insert("m_icmp_ugt", match_icmp_ugt_inst);
    map.insert("m_icmp_ueg", match_icmp_uge_inst);
    map.insert("m_icmp_ult", match_icmp_ult_inst);
    map.insert("m_icmp_ule", match_icmp_ule_inst);
    map.insert("m_icmp_sgt", match_icmp_sgt_inst);
    map.insert("m_icmp_sge", match_icmp_sge_inst);
    map.insert("m_icmp_slt", match_icmp_slt_inst);
    map.insert("m_icmp_sle", match_icmp_sle_inst);

    map.insert("m_c_icmp_eq", match_commutatively_icmp_eq_inst);
    map.insert("m_c_icmp_ne", match_commutatively_icmp_ne_inst);
    map.insert("m_c_icmp_ugt", match_commutatively_icmp_ugt_inst);
    map.insert("m_c_icmp_ueg", match_commutatively_icmp_uge_inst);
    map.insert("m_c_icmp_ult", match_commutatively_icmp_ult_inst);
    map.insert("m_c_icmp_ule", match_commutatively_icmp_ule_inst);
    map.insert("m_c_icmp_sgt", match_commutatively_icmp_sgt_inst);
    map.insert("m_c_icmp_sge", match_commutatively_icmp_sge_inst);
    map.insert("m_c_icmp_slt", match_commutatively_icmp_slt_inst);
    map.insert("m_c_icmp_sle", match_commutatively_icmp_sle_inst);
}

#[inline(always)]
pub fn register_int_comparisons_matchers_function_signatures(
    map: &mut HashMap<&'static str, Signature>,
) {
    map.insert(
        "m_icmp_eq",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_icmp_ne",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_icmp_ugt",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_icmp_ueg",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_icmp_ult",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_icmp_ule",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_icmp_sgt",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_icmp_sge",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_icmp_slt",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_icmp_sle",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_c_icmp_eq",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_c_icmp_ne",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_c_icmp_ugt",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_c_icmp_ueg",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_c_icmp_ult",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_c_icmp_ule",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_c_icmp_sgt",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_c_icmp_sge",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_c_icmp_slt",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_c_icmp_sle",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );
}

fn match_icmp_eq_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
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
        matcher: IntComparisonInstMatcher::create_icmp_eq(lhs_matcher, rhs_matcher),
    })
}

fn match_icmp_ne_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
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
        matcher: IntComparisonInstMatcher::create_icmp_ne(lhs_matcher, rhs_matcher),
    })
}

fn match_icmp_ugt_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
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
        matcher: IntComparisonInstMatcher::create_icmp_ugt(lhs_matcher, rhs_matcher),
    })
}

fn match_icmp_uge_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
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
        matcher: IntComparisonInstMatcher::create_icmp_uge(lhs_matcher, rhs_matcher),
    })
}

fn match_icmp_ult_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
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
        matcher: IntComparisonInstMatcher::create_icmp_ult(lhs_matcher, rhs_matcher),
    })
}

fn match_icmp_ule_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
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
        matcher: IntComparisonInstMatcher::create_icmp_ule(lhs_matcher, rhs_matcher),
    })
}

fn match_icmp_sgt_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
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
        matcher: IntComparisonInstMatcher::create_icmp_sgt(lhs_matcher, rhs_matcher),
    })
}

fn match_icmp_sge_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
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
        matcher: IntComparisonInstMatcher::create_icmp_sge(lhs_matcher, rhs_matcher),
    })
}

fn match_icmp_slt_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
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
        matcher: IntComparisonInstMatcher::create_icmp_slt(lhs_matcher, rhs_matcher),
    })
}

fn match_icmp_sle_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
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
        matcher: IntComparisonInstMatcher::create_icmp_sle(lhs_matcher, rhs_matcher),
    })
}

fn match_commutatively_icmp_eq_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
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
        matcher: IntComparisonInstMatcher::create_commutatively_icmp_eq(lhs_matcher, rhs_matcher),
    })
}

fn match_commutatively_icmp_ne_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
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
        matcher: IntComparisonInstMatcher::create_commutatively_icmp_ne(lhs_matcher, rhs_matcher),
    })
}

fn match_commutatively_icmp_ugt_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
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
        matcher: IntComparisonInstMatcher::create_commutatively_icmp_ugt(lhs_matcher, rhs_matcher),
    })
}

fn match_commutatively_icmp_uge_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
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
        matcher: IntComparisonInstMatcher::create_commutatively_icmp_uge(lhs_matcher, rhs_matcher),
    })
}

fn match_commutatively_icmp_ult_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
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
        matcher: IntComparisonInstMatcher::create_commutatively_icmp_ult(lhs_matcher, rhs_matcher),
    })
}

fn match_commutatively_icmp_ule_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
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
        matcher: IntComparisonInstMatcher::create_commutatively_icmp_ule(lhs_matcher, rhs_matcher),
    })
}

fn match_commutatively_icmp_sgt_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
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
        matcher: IntComparisonInstMatcher::create_commutatively_icmp_sgt(lhs_matcher, rhs_matcher),
    })
}

fn match_commutatively_icmp_sge_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
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
        matcher: IntComparisonInstMatcher::create_commutatively_icmp_sge(lhs_matcher, rhs_matcher),
    })
}

fn match_commutatively_icmp_slt_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
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
        matcher: IntComparisonInstMatcher::create_commutatively_icmp_slt(lhs_matcher, rhs_matcher),
    })
}

fn match_commutatively_icmp_sle_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
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
        matcher: IntComparisonInstMatcher::create_commutatively_icmp_sle(lhs_matcher, rhs_matcher),
    })
}
