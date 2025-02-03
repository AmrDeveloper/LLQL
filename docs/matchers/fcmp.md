### FCMP Instructions Matchers functions

|   Function   |              Parameters              |   Return    |                              Description                              |
| :----------: | :----------------------------------: | :---------: | :-------------------------------------------------------------------: |
|  m_fcmp_eq   | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher |           Build Inst Matcher that match fcmp eg Instruction           |
|  m_fcmp_ne   | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher |           Build Inst Matcher that match fcmp ne Instruction           |
|  m_fcmp_gt   | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher |           Build Inst Matcher that match fcmp gt Instruction           |
|  m_fcmp_ge   | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher |           Build Inst Matcher that match fcmp ge Instruction           |
|  m_fcmp_lt   | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher |           Build Inst Matcher that match fcmp lt Instruction           |
|  m_fcmp_le   | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher |           Build Inst Matcher that match fcmp le Instruction           |
|  m_fcmp_ord  | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher |          Build Inst Matcher that match fcmp ord Instruction           |
|  m_fcmp_uno  | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher |          Build Inst Matcher that match fcmp uno Instruction           |
|  m_fcmp_ueq  | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher |          Build Inst Matcher that match fcmp ueg Instruction           |
|  m_fcmp_une  | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher |          Build Inst Matcher that match fcmp une Instruction           |
|  m_fcmp_ugt  | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher |          Build Inst Matcher that match fcmp ugt Instruction           |
|  m_fcmp_uge  | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher |          Build Inst Matcher that match fcmp uge Instruction           |
|  m_fcmp_ult  | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher |          Build Inst Matcher that match fcmp ult Instruction           |
|  m_fcmp_ule  | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher |          Build Inst Matcher that match fcmp ule Instruction           |
| m_c_fcmp_eq  | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher | Build Inst Matcher that match fcmp eg Instruction with commutatively  |
| m_c_fcmp_ne  | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher | Build Inst Matcher that match fcmp ne Instruction with commutatively  |
| m_c_fcmp_gt  | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher | Build Inst Matcher that match fcmp gt Instruction with commutatively  |
| m_c_fcmp_ge  | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher | Build Inst Matcher that match fcmp ge Instruction with commutatively  |
| m_c_fcmp_lt  | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher | Build Inst Matcher that match fcmp lt Instruction with commutatively  |
| m_c_fcmp_le  | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher | Build Inst Matcher that match fcmp le Instruction with commutatively  |
| m_c_fcmp_ord | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher | Build Inst Matcher that match fcmp ord Instruction with commutatively |
| m_c_fcmp_uno | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher | Build Inst Matcher that match fcmp uno Instruction with commutatively |
| m_c_fcmp_ueq | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher | Build Inst Matcher that match fcmp ueg Instruction with commutatively |
| m_c_fcmp_une | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher | Build Inst Matcher that match fcmp une Instruction with commutatively |
| m_c_fcmp_ugt | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher | Build Inst Matcher that match fcmp ugt Instruction with commutatively |
| m_c_fcmp_uge | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher | Build Inst Matcher that match fcmp uge Instruction with commutatively |
| m_c_fcmp_ult | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher | Build Inst Matcher that match fcmp ult Instruction with commutatively |
| m_c_fcmp_ule | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher | Build Inst Matcher that match fcmp ule Instruction with commutatively |