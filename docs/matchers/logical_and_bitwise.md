### Logical and Bitwise Instructions Matchers functions

|    Function     |              Parameters              |   Return    |                         Description                         |
| :-------------: | :----------------------------------: | :---------: | :---------------------------------------------------------: |
|      m_or       | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher |        Build Inst Matcher that match or Instruction         |
|  m_or_disjoint  | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher |    Build Inst Matcher that match or disjoint Instruction    |
|      m_and      | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher |        Build Inst Matcher that match and Instruction        |
|      m_xor      | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher |        Build Inst Matcher that match xor Instruction        |
|     m_c_or      | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher | Build Inst Matcher that match or Instruction commutatively  |
| m_c_or_disjoint | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher |    Build Inst Matcher that match or disjoint Instruction    |
|     m_c_and     | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher | Build Inst Matcher that match and Instruction commutatively |
|     m_c_xor     | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher | Build Inst Matcher that match xor Instruction commutatively |

---

### Shifts Instructions Matchers functions

| Function |              Parameters              |   Return    |                            Description                            |
| :------: | :----------------------------------: | :---------: | :---------------------------------------------------------------: |
|  m_shl   | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher |           Build Inst Matcher that match shl Instruction           |
|  m_shr   | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher |           Build Inst Matcher that match shr Instruction           |
|  m_ashr  | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher |          Build Inst Matcher that match ashr Instruction           |
| m_c_shl  | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher | Build Inst Matcher that match shl Instruction with commutatively  |
| m_c_shr  | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher | Build Inst Matcher that match shr Instruction with commutatively  |
| m_c_ashr | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher | Build Inst Matcher that match ashr Instruction with commutatively |