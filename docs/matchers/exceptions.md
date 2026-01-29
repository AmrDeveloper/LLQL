### Exceptions Instructions Matchers functions

|      Function      | Parameters |   Return    |                           Description                            |
| :----------------: | :--------: | :---------: | :--------------------------------------------------------------: |
|      m_invoke      |            | InstMatcher |         Build Inst Matcher that match invoke Instruction         |
|    m_landingpad    |            | InstMatcher |       Build Inst Matcher that match landingpad Instruction       |
|      m_resume      |            | InstMatcher |         Build Inst Matcher that match resume Instruction         |
|      m_throw       |            | InstMatcher |       Build Inst Matcher that match call to `__cxa_throw`        |
|     m_rethrow      |            | InstMatcher |      Build Inst Matcher that match call to `__cxa_rethrow`       |
|    m_eh_typeid     |            | InstMatcher |    Build Inst Matcher that match call to `llvm.eh.typeid.for`    |
| m_alloca_exception |            | InstMatcher | Build Inst Matcher that match call to `__cxa_allocate_exception` |
|  m_free_exception  |            | InstMatcher |   Build Inst Matcher that match call to `__cxa_free_exception`   |
