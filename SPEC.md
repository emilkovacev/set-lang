# 1 Axioms

## 1.1 There are only sets

The only type is `SET`. There are no built-in int, float, or boolean types. Instead,
you are encouraged to build your own using set theory! There are multiple ways of representing these
data types using sets, and set-lang does not typically prescribe to one specific approach.

In cases where set-lang needs to return a numerical or boolean value, like with `CARDINALITY` and 
`ELEMENT OF` operations, it returns it using von Neumann ordinals.

### 1.1.1 Natural numbers as von Neumann ordinals

Natural numbers can be represented by sets. When set-lang needs to return a numeric value,
it uses sets in this manner to encode them.

```
0 = {}
1 = {0}
2 = {0, 1}
...
```

## 1.2 Empty set

To denote an empty set, use `{}`.

```
A = {}
```

## 1.3 Nesting

Sets can contain other sets. Each set must contain only unique values. Sets that contain 
two equivilent values in the set is an error.

```
A = {}
B = {{}}
B = {A, B}
```

## 1.4 Single Set Operations

### 1.4.1 PRINT

Prints a set to stdout.

```
A = {}
PRINT A
```
The code block above will print the following to stdout:
```
{}
```

### 1.4.2 CARDINALITY

Find the cardinality of a set.

```
A = {{}, {}, {}}
B = |A|
PRINT B
```

```
{{}, {{}}, {{{}}}}
```

## 1.5 Multi-set operations

### 1.5.1 ELEMENT OF

Check if one set is an element of another set.

```
A = {}
B = {A}

A ELEMENT OF B
```

```
{{}}
```

### 1.5.2 UNION

Perform a union of two sets.

```
0 = {}
1 = {0}
PRINT 0 UNION 1
PRINT 1 UNION 1
```

```
{0}
{0}
```

### 1.5.3 INTERSECTION

Perform an intersection of two sets.

```
0 = {}
1 = {0}
PRINT 0 INTERSECTION 1
PRINT 1 INTERSECTION 1
```

```
{}
{0}
```

### 1.5.4 DIFFERENCE

Perform the difference of two sets.

```
0 = {}
1 = {0}
PRINT 0 DIFFERENCE 1
PRINT 1 DIFFERENCE 1
PRINT 1 DIFFERENCE 0
```

```
{}
{}
{0}
```

### 1.5.5 CARTESIAN PRODUCT

Perform the cartesian product of two sets.

```
0 = {}
1 = {0}
PRINT 0 CARTESION PRODUCT 1
```

```
{{0, {0, 0}}, {0, {0, 1}}, {1, {1, 0}}, {1, {1, 1}}}
```

---

# 2 Compiler Architecture

## 2.1 Lexer

The set-lexer performs lexical analysis on a `.set` file and produces a stream of tuples with information about each 
token.

| Token Name | Description | Token values |
| --- | --- | --- |
| identifier | Variable names, must conform to `^[a-zA-Z0-9_]+$`. A cool side-effect of everything being sets is that we can use numbers in identifiers! | `A`, `Variable`, `1` |
| keyword | There are no reserved keywords in set-lang | |
| literal | Tokens used to reference sets (sets are the only literals in set-lang) | `{`, `}` |
| separator | Punctuation to separate logic | `\n` |
| operator | Operation symbols applied to sets and variables | `=` (assignment), `PRINT`, `CARDINALITY`, `ELEMENT OF`, `UNION`, `INTERSECTION`, `DIFFERENCE`, `CARTESIAN PRODUCT` |
| comment | Code descriptions, no-op | `// ` (no multiline comments) |
| whitespace | no-op | |

## 2.2 Parser

The set-parser uses the set-lexer to parse the syntax of a `.set` file.

| Syntax | Description | Example |
| --- | --- | --- |
| set | Initialize a set. Sets can be nested, and all elements of a set MUST be unique.| `{}`, `{{}}`, `{{}, {{}}}` |
| expression | A sequence of sets and operations. Expressions have return values (sets). An expression without a return value will return the empty set `{}`. | `{} UNION {{}}`, `PRINT {}` |
| assignment | Assign an expression to a variable. **Sets cannot be modified in-place** (e.g. without assignment). | `VAR = {}` |
| print (operation) | A special operation that prints the output of an expression to stdout | `PRINT {{}, {{}}}` |

## 2.3 Code Generator

The set-code-generator consists of three projects: set-analyzer, set-ir-generator, and set-target-generator.

## 2.4 Analyzer

The set-analyzer inputs a `.set` file that has correct syntax (has already been run through the parser) and outputs an AST structure that describes the syntax of the file.

### 2.4.1 SET Type Struct

Because sets are the only type in set-lang, it is the only type that needs to be implemented!

To implement the data structure for a set, we use a bitwise lookup table:

```
| n subsets | 0 nests | 1 nests | ... | 16 nests | 17 nests | 18 nests | ... | 32 nests | ... |
| --------- | ---------------------------------- | ------------------------------------ |
      |                      |                                     |
     u16                    u16                                   u16
```

Why u16?

If we used u32, the maximum memory needed to store a set would be:
```
2^32 nests * 1 bit/ * 1 byte/ = 536,870,912 B = 0.5 GB
               nest   8 bits
```

When using u16, the maximum memory needed to store a set is:
```
2^16 nests * 1 bit/ * 1 byte/ = 8192 B = 8.2 KB
               nest   8 bits
```

This method is very efficient, since we can take advantage of the speed of bitwise operations, and use bitwise comparisons to check if an element exists, and compare sets with one another.

We can also create arbitrarily large sets, simply by extending the array of u16 elements in the struct. And best of all, the struct doesn't need any padding!

The number of elements in this implementation of a set is strictly bounded by 2^16 (65,536).

### 2. IR Generator

The set-ir-generator inputs the AST from set-analyzer and outputs a `.set_ir` file that represents the logic of the analyzer using set-ir, an Intermediate Representation for set logic.

All abstract code optimizations are made at this stage.

### 3. Target Generator

The set-target-generator inputs the IR from the set-ir-generator and outputs a file with assembly code for the target machine.

---

# 3 Appendix

## 3.1 Cardinality Bound

Because each element in a set must be unique, the cardinality is bounded by the number of nests in the deepest nested set.

```
Proof by contradiction
---
Let A be a set such that |A| is greater than the number of nests N of the deepest nested set D within A.

Given that N is the deepest nested set in A, and that every item in the set MUST be unique, every set in A must have <= N nests.

The largest combination of nested sets where each item in the set is unique and <= N nests is:

S = {0 nests (empty set), 1 nest, 2 nests, ..., N nests}

Because the empty set is not counted in cardinality, |S| = N, thus contradicting the initial claim.
```

## 3.2 SET Implementation Option 1 - Linked List

A `u8` represents a boolean type, `0` for a nested set and `1` for empty set. A nested set would include two pointers, one to the set that nests it, and another to the next set in the parent set. Each parent set takes up 96 bytes, and each empty set takes up 8 bytes.

```
----------------------    ---------------------
| set (u32 ptr)      | -> | type = EMPTY (u8) |
| ------------------ |    ---------------------
| next (u32 ptr)     |
| ------------------ |
| type = NESTED (u8) | 
| ------------------ |
| padding (24 bytes) |
| ------------------ |
|         .          |
|         .          |
|         .          |
| ------------------ |    ----------------------    ---------------------
| set (u32 ptr)      | -> | set (u32 ptr)      | -> | type = EMPTY (u8) |
| ------------------ |    | ------------------ |    ---------------------
| next = NULL        |    | next = NULL        |
| ------------------ |    | ------------------ |
| type = NESTED (u8) |    | type = NESTED (u8) |
| ------------------ |    | ------------------ |
| padding (24 bytes) |    | padding (24 bytes) |
----------------------    ----------------------
```

### 3.2.1 CARDINALITY

Memory:
```
Because each element in a set must be unique, the cardinality is bounded by the deepest nested set [Appendix 3.1].

Let S = a set
Let D = the deepest nested set within S

Because |S|<=D, the largest set that S could be is

S = {D nests, D-1 nests, D-2 nests, ..., 2 nests, 1 nest, 0 nests (empty set)}

The sum of all nests in S is at most
A = (D(D + 1)) / 2 = O(D^2)

using the Linked List implementation, each nest is 96 bytes, so this implementation takes O(96 * D^2) memory to store a set.
```

Runtime:
```
Let S = a set
Let D = the deepest nested set within S

Using the previous proof, we know that the sum of all nests in S is at most
A = (D(D + 1)) / 2 = O(D^2)

In order to calculate the cardinality using this implementation, we would have to iterate through every nest in a set, taking O(D^2) time.
```

### 3.2.2 UNION

## 3.3 SET Implementation Option 2 - Simple Nesting

A nested set can be represented by an integer. For example, `{}` is nest=0 (empty set), `{{}}` is nest=1, `{{{}}}` is nest=2, and so on...

```
| --------------- |
| m items (u32)   |
| --------------- |
| --------------- |
| n nests (u32)   |
| --------------- | ]
|        .        | ]
|        .        | ] m items
|        .        | ]
| --------------- | ]
| n nests (u32)   |
| --------------- |
```

## 3.4 SET Implementation Option 3 - Nesting + Bitwise Lookup Table
