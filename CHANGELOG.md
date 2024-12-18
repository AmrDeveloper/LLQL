# Change Log

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