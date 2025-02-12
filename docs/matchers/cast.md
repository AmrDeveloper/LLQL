## Cast Instructions Matchers

|     Function      |      Parameters       |   Return    |                           Description                            |
| :---------------: | :-------------------: | :---------: | :--------------------------------------------------------------: |
|      m_cast       | (value: InstMatcher?) | InstMatcher |      Build Inst Matcher that matchers any cast instruction       |
|      m_trunc      | (value: InstMatcher?) | InstMatcher |        Build Inst Matcher that matchers trunc instruction        |
|    m_fp_to_ui     | (value: InstMatcher?) | InstMatcher |      Build Inst Matcher that matchers fp to ui instruction       |
|    m_fp_to_si     | (value: InstMatcher?) | InstMatcher |      Build Inst Matcher that matchers fp to si instruction       |
|    m_fp_trunc     | (value: InstMatcher?) | InstMatcher |        Build Inst Matcher that matchers trunc instruction        |
|   m_int_to_ptr    | (value: InstMatcher?) | InstMatcher |      Build Inst Matcher that matchers IntToPtr instruction       |
|   m_ptr_to_int    | (value: InstMatcher?) | InstMatcher |      Build Inst Matcher that matchers PtrToInt instruction       |
|       m_ext       | (value: InstMatcher?) | InstMatcher | Build Inst Matcher that matchers zext, sext or fpext instruction |
|      m_fpext      | (value: InstMatcher?) | InstMatcher |        Build Inst Matcher that matchers fpext instruction        |
|      m_zext       | (value: InstMatcher?) | InstMatcher |        Build Inst Matcher that matchers zext instruction         |
|      m_sext       | (value: InstMatcher?) | InstMatcher |        Build Inst Matcher that matchers sext instruction         |
|    m_bit_cast     | (value: InstMatcher?) | InstMatcher |      Build Inst Matcher that matchers Bit cast instruction       |
| m_addr_space_cast | (value: InstMatcher?) | InstMatcher |    Build Inst Matcher that matchers AddrSpaceCast instruction    |
