### Instructions Matchers 

Instructions Matchers are functions that build a instruction matcher to match against LLVM Instructions

### General Instructions Matchers functions

|   Function    |            Parameters            |   Return    |                      Description                       |
| :-----------: | :------------------------------: | :---------: | :----------------------------------------------------: |
|    m_inst     | (i: Instruction, m: InstMatcher) |    Bool     |      Check if instruction is matched with Matcher      |
|  m_any_inst   |                ()                | InstMatcher |     Build Inst Matcher that match any Instruction      |
|  m_const_int  |                ()                | InstMatcher |   Build Inst Matcher that match constants int value    |
|  m_const_fp   |                ()                | InstMatcher |  Build Inst Matcher that match constants float value   |
| m_const_null  |                ()                | InstMatcher |  Build Inst Matcher that match constants pointer null  |
|   m_poison    |                ()                | InstMatcher |       Build Inst Matcher that match poison value       |
|    m_label    |           (n : Text?)            | InstMatcher | Build Inst Matcher that match Label with optional name |
|   m_return    |        (m : InstMatcher?)        | InstMatcher |    Build Inst Matcher that match Return Instruction    |
| m_unreachable |                ()                | InstMatcher | Build Inst Matcher that match unreachable Instruction  |

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
