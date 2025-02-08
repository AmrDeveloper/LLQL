### General Binary Instructions Matchers functions

| Function  |               Parameters               |   Return    |                             Description                             |
| :-------: | :------------------------------------: | :---------: | :-----------------------------------------------------------------: |
|  m_binop  | (lhs: InstMatcher?, rhs: InstMatcher?) | InstMatcher |          Build Inst Matcher that match binary Instruction           |
| m_c_binop | (lhs: InstMatcher?, rhs: InstMatcher?) | InstMatcher | Build Inst Matcher that match binary Instruction with commutatively |