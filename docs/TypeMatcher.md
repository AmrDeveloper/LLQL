### Type Matchers 

Types Matchers are functions that build a type matcher to match against LLVM Type

| Function |           Parameters            |   Return    |                          Description                           |
| :------: | :-----------------------------: | :---------: | :------------------------------------------------------------: |
|  m_int1  |               ()                | TypeMatcher |        Build Matcher for LLVM Type Int1 (like Boolean)         |
|  m_int8  |               ()                | TypeMatcher |                Build Matcher for LLVM Type Int8                |
| m_int16  |               ()                | TypeMatcher |               Build Matcher for LLVM Type Int16                |
| m_int32  |               ()                | TypeMatcher |               Build Matcher for LLVM Type Int32                |
| m_int64  |               ()                | TypeMatcher |               Build Matcher for LLVM Type Int64                |
|  m_f32   |               ()                | TypeMatcher |              Build Matcher for LLVM Type Float32               |
|  m_f64   |               ()                | TypeMatcher |              Build Matcher for LLVM Type Float64               |
|  m_void  |               ()                | TypeMatcher |                Build Matcher for LLVM Type void                |
|  m_ptr   |               ()                | TypeMatcher |              Build Matcher for LLVM Type Pointer               |
| m_array  | (type: TypeMatcher, size: Int?) | TypeMatcher | Build Matcher for LLVM Type Array with optional base and size  |
| m_vector | (type: TypeMatcher, size: Int?) | TypeMatcher | Build Matcher for LLVM Type Vector with optional base and size |