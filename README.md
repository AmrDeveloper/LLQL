<h1 align="center">LLQL - LLVM IR/BC Query Language</h1></br>

<p align="center">
<img src="media/llql_logo.svg" width="20%" height="20%"/>
</p>

<p align="center">
  <img alt="Crates.io" src="https://img.shields.io/crates/v/llql?style=flat-square">
  <img alt="Deps" src="https://deps.rs/repo/github/amrdeveloper/llql/status.svg">
  <img alt="GitHub issues" src="https://img.shields.io/github/issues/amrdeveloper/llql">
  <img alt="GitHub" src="https://img.shields.io/github/license/amrdeveloper/llql">
</p>

<p align="center">
LLQL is a tool that allow you to run SQL-like query with Pattern matching functions inspired by LLVM InstCombine Pattern Matchers on LLVM IR/BitCode files instead of database files using the GitQL SDK.
</p>

<p align="center">
  <img src="media/llql_demo.PNG" alt="animated" width="100%"/>
</p>

---

### Sample

If we have LLVM IR function like this, and we want to match `add` instruction that has result of sub instruction as Left hand side and result of mul instruction as Right hand side.

```ir
define i32 @function(i32 %a, i32 %b) {
  %sub = sub i32 %a, %b
  %mull = mul i32 %a, %b
  %add = add i32 %sub, %mull
  ret i32 %add
}
```

We can query to print the instruction with this query

```sql
SELECT instruction FROM instructions WHERE m_inst(instruction, m_add(m_sub(m_any_inst(), m_any_inst()), m_mul(m_any_inst(), m_any_inst())))
```

Or you can count the number of times this pattern exists in your inputs files

```sql
SELECT COUNT() FROM instructions WHERE m_inst(instruction, m_add(m_sub(m_any_inst(), m_any_inst()), m_mul(m_any_inst(), m_any_inst())))
```

---

### List of available functions

- [Instructions Matchers Functions](docs/InstructionMatcher.md)
- [Types Matchers functions](docs/TypeMatcher.md)

---

### Tables structures

#### Instructions table

| Name             | Type      | Description                     |
| ---------------- | --------- | ------------------------------- |
| function_name    | Text      | Instruction function name       |
| basic_block_name | Text      | Basic block of this instruction |
| instruction      | LLVMValue | LLVM Instruction                |

---

### Download or Install

Note that Building from source or installing from Cargo.io requires LibClang 16 to be installed

- Install from Cargo.io

```
cargo install llql
```

- Build from source code

```
git clone https://github.com/AmrDeveloper/LLQL.git
cd LLQL
cargo build
```

### Run LLQL

```
LLQL is a SQL like query language to run on LLVM IR/BitCode files
Usage: LLQL [OPTIONS]

Options:
  -f,  --files <paths>        Path for local files to run query on
  -q,  --query <GQL Query>    LLQL query to run on selected files
  -p,  --pagination           Enable print result with pagination
  -ps, --pagesize             Set pagination page size [default: 10]
  -o,  --output               Set output format [render, json, csv]
  -a,  --analysis             Print Query analysis
  -e,  --editor               Enable GitQL LineEditor
  -h,  --help                 Print LLQL help
  -v,  --version              Print LLQL Current Version
```

### License

```
MIT License

Copyright (c) 2024 Amr Hesham

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```