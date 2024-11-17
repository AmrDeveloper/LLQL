# Change Log

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