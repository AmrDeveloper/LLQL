# LLQL

LLQL is a tool that enable you to run a SQL-like query on one or multiple LLVM IR/BC files and perform pattern matching functions
that designed with SQL syntax to be like a DSL so you can search for patterns by matching against the IR node as tree node not just like a text.

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
SELECT instruction FROM instructions WHERE m_inst(instruction, m_add(m_sub(), m_mul()))
```

Or for example you can query how many times this pattern exists in each function

```sql
SELECT function_name, count() FROM instructions WHERE m_inst(instruction, m_add(m_sub(), m_mul())) GROUP BY function_name
```

You can also filter by number of times the value is used for example for not used values

```IR
define i32 @function(i32 %a, i32 %b) {
  %unused_add = add i32 %a, 1

  %used_twice = add i32 %a, %b
  %add2 = add i32 %used_twice, %b
  %add3 = add i32 %used_twice, %add2
  ret i32 %add3
}
```

```sql
SELECT instruction FROM instructions WHERE m_inst(instruction, m_unused(m_add()))
```

and for value that used only time

```sql
SELECT instruction FROM instructions WHERE m_inst(instruction, m_has_one_use(m_add()))
```

and for value that used n times

```sql
SELECT instruction FROM instructions WHERE m_inst(instruction, m_has_n_uses(m_add(), 2))
```