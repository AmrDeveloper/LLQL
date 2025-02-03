### Arithmetic Instructions Matchers functions

| Function |               Parameters               |   Return    |                            Description                            |
| :------: | :------------------------------------: | :---------: | :---------------------------------------------------------------: |
|  m_add   | (lhs: InstMatcher?, rhs: InstMatcher?) | InstMatcher |           Build Inst Matcher that match Add Instruction           |
|  m_sub   | (lhs: InstMatcher?, rhs: InstMatcher?) | InstMatcher |           Build Inst Matcher that match Sub Instruction           |
|  m_mul   | (lhs: InstMatcher?, rhs: InstMatcher?) | InstMatcher |           Build Inst Matcher that match Mul Instruction           |
|  m_div   | (lhs: InstMatcher?, rhs: InstMatcher?) | InstMatcher |           Build Inst Matcher that match Div Instruction           |
|  m_rem   | (lhs: InstMatcher?, rhs: InstMatcher?) | InstMatcher | Build Inst Matcher that match Rem Instruction with commutatively  |
| m_c_add  | (lhs: InstMatcher?, rhs: InstMatcher?) | InstMatcher | Build Inst Matcher that match Add Instruction with commutatively  |
| m_c_sub  | (lhs: InstMatcher?, rhs: InstMatcher?) | InstMatcher | Build Inst Matcher that match Sub Instruction with commutatively  |
| m_c_mul  | (lhs: InstMatcher?, rhs: InstMatcher?) | InstMatcher | Build Inst Matcher that match Mul Instruction with commutatively  |
| m_c_div  | (lhs: InstMatcher?, rhs: InstMatcher?) | InstMatcher | Build Inst Matcher that match Div Instruction with commutatively  |
| m_c_rem  | (lhs: InstMatcher?, rhs: InstMatcher?) | InstMatcher | Build Inst Matcher that match Rem Instruction with commutatively  |
|  m_fadd  | (lhs: InstMatcher?, rhs: InstMatcher?) | InstMatcher |          Build Inst Matcher that match FAdd Instruction           |
|  m_fsub  | (lhs: InstMatcher?, rhs: InstMatcher?) | InstMatcher |          Build Inst Matcher that match FSub Instruction           |
|  m_fmul  | (lhs: InstMatcher?, rhs: InstMatcher?) | InstMatcher |          Build Inst Matcher that match FMul Instruction           |
|  m_fdiv  | (lhs: InstMatcher?, rhs: InstMatcher?) | InstMatcher |          Build Inst Matcher that match FDiv Instruction           |
|  m_frem  | (lhs: InstMatcher?, rhs: InstMatcher?) | InstMatcher | Build Inst Matcher that match FRem Instruction with commutatively |
| m_c_fadd | (lhs: InstMatcher?, rhs: InstMatcher?) | InstMatcher | Build Inst Matcher that match FAdd Instruction with commutatively |
| m_c_fsub | (lhs: InstMatcher?, rhs: InstMatcher?) | InstMatcher | Build Inst Matcher that match FSub Instruction with commutatively |
| m_c_fmul | (lhs: InstMatcher?, rhs: InstMatcher?) | InstMatcher | Build Inst Matcher that match FMul Instruction with commutatively |
| m_c_fdiv | (lhs: InstMatcher?, rhs: InstMatcher?) | InstMatcher | Build Inst Matcher that match FDiv Instruction with commutatively |
| m_c_frem | (lhs: InstMatcher?, rhs: InstMatcher?) | InstMatcher | Build Inst Matcher that match FRem Instruction with commutatively |