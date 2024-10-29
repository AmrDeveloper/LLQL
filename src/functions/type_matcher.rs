use std::collections::HashMap;

use gitql_ast::types::integer::IntType;
use gitql_ast::types::optional::OptionType;
use gitql_core::signature::Function;
use gitql_core::signature::Signature;
use gitql_core::values::base::Value;

use crate::ir::types::TypeMatcherType;
use crate::ir::values::TypeMatcherValue;
use crate::matchers::type_matcher::ArrayTypeMatcher;
use crate::matchers::type_matcher::FloatTypeMatcher;
use crate::matchers::type_matcher::FloatTypeSize;
use crate::matchers::type_matcher::IntTypeMatcher;
use crate::matchers::type_matcher::IntTypeSize;
use crate::matchers::type_matcher::PointerTypeMatcher;
use crate::matchers::type_matcher::ScalableVectorTypeMatcher;
use crate::matchers::type_matcher::VectorTypeMatcher;
use crate::matchers::type_matcher::VoidTypeMatcher;

#[inline(always)]
pub fn register_type_matchers_functions(map: &mut HashMap<&'static str, Function>) {
    // Matchers for Integer types
    map.insert("m_int1", match_int1);
    map.insert("m_int8", match_int8);
    map.insert("m_int16", match_int16);
    map.insert("m_int32", match_int32);
    map.insert("m_int64", match_int64);

    // Matchers for Float types
    map.insert("m_f32", match_float32);
    map.insert("m_f64", match_float64);

    // Matcher for Void type
    map.insert("m_void", match_void);

    // Matcher for Pointer types
    map.insert("m_ptr", match_pointer);

    // Matcher for Array type
    map.insert("m_array", match_array);

    // Matcher for Vector type
    map.insert("m_vector", match_vector);
    map.insert("m_scalable_vector", match_scalable_vector);
}

#[inline(always)]
pub fn register_type_matchers_function_signatures(map: &mut HashMap<&'static str, Signature>) {
    map.insert(
        "m_void",
        Signature {
            parameters: vec![],
            return_type: Box::new(TypeMatcherType),
        },
    );
    map.insert(
        "m_int1",
        Signature {
            parameters: vec![],
            return_type: Box::new(TypeMatcherType),
        },
    );
    map.insert(
        "m_int8",
        Signature {
            parameters: vec![],
            return_type: Box::new(TypeMatcherType),
        },
    );
    map.insert(
        "m_int16",
        Signature {
            parameters: vec![],
            return_type: Box::new(TypeMatcherType),
        },
    );
    map.insert(
        "m_int32",
        Signature {
            parameters: vec![],
            return_type: Box::new(TypeMatcherType),
        },
    );
    map.insert(
        "m_int64",
        Signature {
            parameters: vec![],
            return_type: Box::new(TypeMatcherType),
        },
    );
    map.insert(
        "m_f32",
        Signature {
            parameters: vec![],
            return_type: Box::new(TypeMatcherType),
        },
    );
    map.insert(
        "m_f64",
        Signature {
            parameters: vec![],
            return_type: Box::new(TypeMatcherType),
        },
    );
    map.insert(
        "m_ptr",
        Signature {
            parameters: vec![],
            return_type: Box::new(TypeMatcherType),
        },
    );

    map.insert(
        "m_array",
        Signature {
            parameters: vec![
                Box::new(TypeMatcherType),
                Box::new(OptionType {
                    base: Some(Box::new(IntType)),
                }),
            ],
            return_type: Box::new(TypeMatcherType),
        },
    );
    map.insert(
        "m_vector",
        Signature {
            parameters: vec![
                Box::new(TypeMatcherType),
                Box::new(OptionType {
                    base: Some(Box::new(IntType)),
                }),
            ],
            return_type: Box::new(TypeMatcherType),
        },
    );
    map.insert(
        "m_scalable_vector",
        Signature {
            parameters: vec![],
            return_type: Box::new(TypeMatcherType),
        },
    );
}

fn match_void(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    Box::new(TypeMatcherValue {
        matcher: Box::new(VoidTypeMatcher),
    })
}

fn match_int1(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let int1_matcher = IntTypeMatcher {
        size: IntTypeSize::Size1,
    };

    Box::new(TypeMatcherValue {
        matcher: Box::new(int1_matcher),
    })
}

fn match_int8(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let int1_matcher = IntTypeMatcher {
        size: IntTypeSize::Size8,
    };

    Box::new(TypeMatcherValue {
        matcher: Box::new(int1_matcher),
    })
}

fn match_int16(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let int1_matcher = IntTypeMatcher {
        size: IntTypeSize::Size16,
    };

    Box::new(TypeMatcherValue {
        matcher: Box::new(int1_matcher),
    })
}

fn match_int32(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let int1_matcher = IntTypeMatcher {
        size: IntTypeSize::Size32,
    };

    Box::new(TypeMatcherValue {
        matcher: Box::new(int1_matcher),
    })
}

fn match_int64(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let int1_matcher = IntTypeMatcher {
        size: IntTypeSize::Size64,
    };

    Box::new(TypeMatcherValue {
        matcher: Box::new(int1_matcher),
    })
}

fn match_float32(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let int1_matcher = FloatTypeMatcher {
        size: FloatTypeSize::Size32,
    };

    Box::new(TypeMatcherValue {
        matcher: Box::new(int1_matcher),
    })
}

fn match_float64(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let int1_matcher = FloatTypeMatcher {
        size: FloatTypeSize::Size64,
    };

    Box::new(TypeMatcherValue {
        matcher: Box::new(int1_matcher),
    })
}

fn match_pointer(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let pointer_matcher = PointerTypeMatcher;

    Box::new(TypeMatcherValue {
        matcher: Box::new(pointer_matcher),
    })
}

fn match_array(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let base_matcher = &values[0]
        .as_any()
        .downcast_ref::<TypeMatcherValue>()
        .unwrap()
        .matcher;

    let length = if values.len() == 2 {
        Some(values[1].as_int().unwrap() as u32)
    } else {
        None
    };

    let array_type_matcher = ArrayTypeMatcher {
        base_matcher: base_matcher.clone(),
        length,
    };

    Box::new(TypeMatcherValue {
        matcher: Box::new(array_type_matcher),
    })
}

fn match_vector(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let base_matcher = &values[0]
        .as_any()
        .downcast_ref::<TypeMatcherValue>()
        .unwrap()
        .matcher;

    let length = if values.len() == 2 {
        Some(values[1].as_int().unwrap() as u32)
    } else {
        None
    };

    let array_type_matcher = VectorTypeMatcher {
        base_matcher: base_matcher.clone(),
        length,
    };

    Box::new(TypeMatcherValue {
        matcher: Box::new(array_type_matcher),
    })
}

fn match_scalable_vector(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    Box::new(TypeMatcherValue {
        matcher: Box::new(ScalableVectorTypeMatcher),
    })
}
