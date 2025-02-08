# Change Log

## Version 0.5.0 _(2025-02-8)_

- Implement `m_operands_number` Matcher.
- Implement `m_trunc`, `m_fp_trunc`, `m_fp_to_ui` and `m_fp_to_si` Matchers.
- Implement `m_int_to_ptr`, `m_ptr_to_int` and `m_bit_cast` Matchers.
- Implement `m_addr_space_cast`, `m_zext` and `m_sext` Matchers.

## Version 0.4.0 _(2025-01-04)_

- Implement `m_or_disjoint`, `m_c_or_disjoint` Matchers.
- Implement `m_extract_value` with varargs number of indices Matcher.
- Implement Combine functions `m_inst_combine_oneof`, `m_inst_combine_allof` and `m_inst_combine_noneof` matchers.
- Implement Combine Binary functions `m_inst_combine_and`, `m_inst_combine_or`, `m_inst_combine_xor`.
- Implement Constants Matchers `m_zero`, `m_one`, `m_power2` and `m_specific_int` functions.
- Implement Combine Unary function `m_inst_combine_not`.
- Migrate to LLVM 18.0.0
- Migrate to GitQL 0.34.0.

## Version 0.3.0 _(2024-12-18)_

- Migrate to GitQL 0.32.0.
- Support Script mode.
- Add CLI option to enable scripting mode.
- Implement Logical and Arithmetic shifts matchers.
- Implement `m_half` Matchers.
- Implement `m_or`, `m_and`, `m_xor`, `m_c_or`, `m_c_and`, `m_c_xor`, Matchers.
- Implement `m_invoke`, `m_landingpad` Matchers.

## Version 0.2.0 _(2024-11-17)_

- Handle Parsing IR/BC files with invalid content.
- Migrate to GitQL SDK 0.31.0.
- Implement `m_inst_type` to match instruction depending only on type.
- Implement Unused instruction matcher.
- Implement `m_has_n_uses`, `m_has_one_use`, `m_unused` Instruction matcher.
- Implement `m_argument(str)` to match argument value.
- Implement `m_const_null` Matcher for constants pointer null.
- Implement Matchers for constants integer, Float.
- Implement m_label to Match label with optional text.

## Version 0.1.0 _(2024-10-30)_

- The first release of LLQL.
