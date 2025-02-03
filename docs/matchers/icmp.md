### ICMP Instructions Matchers functions

|   Function   |              Parameters              |   Return    |                              Description                              |
| :----------: | :----------------------------------: | :---------: | :-------------------------------------------------------------------: |
|  m_icmp_eq   | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher |           Build Inst Matcher that match icmp eg Instruction           |
|  m_icmp_ne   | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher |           Build Inst Matcher that match icmp ne Instruction           |
|  m_icmp_ugt  | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher |          Build Inst Matcher that match icmp ugt Instruction           |
|  m_icmp_ueg  | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher |          Build Inst Matcher that match icmp ueg Instruction           |
|  m_icmp_ult  | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher |          Build Inst Matcher that match icmp ult Instruction           |
|  m_icmp_ule  | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher |          Build Inst Matcher that match icmp ule Instruction           |
|  m_icmp_sgt  | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher |          Build Inst Matcher that match icmp sgt Instruction           |
|  m_icmp_sge  | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher |          Build Inst Matcher that match icmp sge Instruction           |
|  m_icmp_slt  | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher |          Build Inst Matcher that match icmp slt Instruction           |
|  m_icmp_sle  | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher |          Build Inst Matcher that match icmp sle Instruction           |
| m_c_icmp_eq  | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher | Build Inst Matcher that match icmp eg Instruction with commutatively  |
| m_c_icmp_ne  | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher | Build Inst Matcher that match icmp ne Instruction with commutatively  |
| m_c_icmp_ugt | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher | Build Inst Matcher that match icmp ugt Instruction with commutatively |
| m_c_icmp_ueg | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher | Build Inst Matcher that match icmp ueg Instruction with commutatively |
| m_c_icmp_ult | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher | Build Inst Matcher that match icmp ult Instruction with commutatively |
| m_c_icmp_ule | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher | Build Inst Matcher that match icmp ule Instruction with commutatively |
| m_c_icmp_sgt | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher | Build Inst Matcher that match icmp sgt Instruction with commutatively |
| m_c_icmp_sge | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher | Build Inst Matcher that match icmp sge Instruction with commutatively |
| m_c_icmp_slt | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher | Build Inst Matcher that match icmp slt Instruction with commutatively |
| m_c_icmp_sle | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher | Build Inst Matcher that match icmp sle Instruction with commutatively |