### Constants Instructions Matchers functions

|    Function    |  Parameters  |   Return    |                           Description                            |
| :------------: | :----------: | :---------: | :--------------------------------------------------------------: |
|  m_const_int   |              | InstMatcher |        Build Inst Matcher that match constants int value         |
|     m_zero     |              | InstMatcher |     Build Inst Matcher that match constants int with value 0     |
|     m_one      |              | InstMatcher |     Build Inst Matcher that match constants int with value 1     |
|    m_power2    |              | InstMatcher | Build Inst Matcher that match constants int if it's power of two |
| m_specific_int | (value: Int) | InstMatcher |    Build Inst Matcher that match specific constants int value    |
|   m_const_fp   |              | InstMatcher |       Build Inst Matcher that match constants float value        |
|  m_const_null  |              | InstMatcher |       Build Inst Matcher that match constants pointer null       |
