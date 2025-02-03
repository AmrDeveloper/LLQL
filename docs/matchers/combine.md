### Combine Instructions Matchers functions

|       Function        |              Parameters              |   Return    |                                      Description                                       |
| :-------------------: | :----------------------------------: | :---------: | :------------------------------------------------------------------------------------: |
| m_inst_combine_oneof  |        (inst: ...InstMatcher)        | InstMatcher |    Build Inst Matcher from list of matchers that return true if one of them matches    |
| m_inst_combine_allof  |        (inst: ...InstMatcher)        | InstMatcher |    Build Inst Matcher from list of matchers that return true if all of them matches    |
| m_inst_combine_noneof |        (inst: ...InstMatcher)        | InstMatcher | Build Inst Matcher from list of matchers that return true if all of them don't matches |
|  m_inst_combine_and   | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher |        Build Inst Matcher two matchers that return true if (lhs and rhs) = true        |
|   m_inst_combine_or   | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher |        Build Inst Matcher two matchers that return true if (lhs or rhs) = true         |
|  m_inst_combine_xor   | (lhs: InstMatcher, rhs: InstMatcher) | InstMatcher |        Build Inst Matcher two matchers that return true if (lhs xor rhs )= true        |
|  m_inst_combine_not   |          (rhs: InstMatcher)          | InstMatcher |           Build Inst Matcher two matchers that return true if (!rhs) = true            |


### Combine Instructions Matchers Operators

| Operator | Description                             |
| -------- | --------------------------------------- |
| AND      | A syntax sugar for `m_inst_combine_and` |
| OR       | A syntax sugar for `m_inst_combine_or`  |
| XOR      | A syntax sugar for `m_inst_combine_xor` |
| !        | A syntax sugar for `m_inst_combine_not` |
