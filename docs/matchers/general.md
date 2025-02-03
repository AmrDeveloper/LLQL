### General Instructions Matchers functions

|     Function      |              Parameters              |   Return    |                               Description                                |
| :---------------: | :----------------------------------: | :---------: | :----------------------------------------------------------------------: |
|      m_inst       |   (i: Instruction, m: InstMatcher)   |    Bool     |               Check if instruction is matched with Matcher               |
|  m_extract_value  | (m : InstMatcher?, indices: ...Int?) | InstMatcher |          Build Inst Matcher that match ExtractValue Instruction          |
|    m_inst_type    |          (m : TypeMatcher?)          | InstMatcher |         Build Inst Matcher that match instruction returned type          |
|    m_any_inst     |                                      | InstMatcher |              Build Inst Matcher that match any Instruction               |
|     m_poison      |                                      | InstMatcher |                Build Inst Matcher that match poison value                |
|      m_label      |             (n : Text?)              | InstMatcher |          Build Inst Matcher that match Label with optional name          |
|    m_argument     |    (n : Text?, m : TypeMatcher?)     | InstMatcher | Build Inst Matcher that match Argument value with optional name and type |
|     m_return      |          (m : InstMatcher?)          | InstMatcher |             Build Inst Matcher that match Return Instruction             |
|   m_unreachable   |                                      | InstMatcher |          Build Inst Matcher that match unreachable Instruction           |
|     m_unused      |          (m : InstMatcher?)          | InstMatcher |       Build Inst Matcher that match instruction that unused at all       |
|   m_has_one_use   |          (m : InstMatcher?)          | InstMatcher |    Build Inst Matcher that match instruction that has exactly on use     |
|   m_has_n_uses    |      (m : InstMatcher?, n: Int)      | InstMatcher |   Build Inst Matcher that match instruction that has n number of uses    |
| m_operands_number |               (n: Int)               | InstMatcher |       Built Inst Matcher that match number of instruction operands       |