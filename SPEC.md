# Axioms

## 0. There are only sets

The only type is `SET`. There are no built-in int, float, or boolean types. Instead,
you are encouraged to build your own using set theory! There are multiple ways of representing these
data types using sets, and set-lang does not typically prescribe to one specific approach.

In cases where set-lang needs to return a numerical or boolean value, like with `CARDINALITY` and 
`ELEMENT OF` operations, it returns it using von Neumann ordinals.

### Natural numbers as von Neumann ordinals

Natural numbers can be represented by sets. When set-lang needs to return a numeric value,
it uses sets in this manner to encode them.

```
0 = {}
1 = {0}
2 = {0, 1}
...
```

## 1. Empty set

To denote an empty set, use `{}`.

```
A = {}
```

## 2. Nesting

Sets can contain other sets. Each set must contain only unique values. Sets that contain 
two equivilent values in the set is an error.

```
A = {}
B = {{}}
B = {A, B}
```

## 3. Single Set Operations

### PRINT

Prints a set to stdout.

```
A = {}
PRINT A
```
The code block above will print the following to stdout:
```
{}
```

### CARDINALITY

Find the cardinality of a set.

```
A = {{}, {}, {}}
B = |A|
PRINT B
```

```
{{}, {{}}, {{{}}}}
```

## 4. Multi-set operations

### ELEMENT OF

Check if one set is an element of another set.

```
A = {}
B = {A}

A ELEMENT OF B
```

```
{{}}
```

### UNION

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

### INTERSECTION

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

### DIFFERENCE

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

### CARTESIAN PRODUCT

Perform the cartesian product of two sets.

```
0 = {}
1 = {0}
PRINT 0 CARTESION PRODUCT 1
```

```
{{0, {0, 0}}, {0, {0, 1}}, {1, {1, 0}}, {1, {1, 1}}}
```

# Compiler Architecture

## Lexer

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

## Parser

The set-parser uses the set-lexer to parse the syntax of a `.set` file.

| Syntax | Description | Example |
| --- | --- | --- |
| set | Initialize a set. Sets can be nested, and all elements of a set MUST be unique.| `{}`, `{{}}`, `{{}, {{}}}` |
| expression | A sequence of sets and operations. Expressions have return values (sets). An expression without a return value will return the empty set `{}`. | `{} UNION {{}}`, `PRINT {}` |
| assignment | Assign an expression to a variable. **Sets cannot be modified in-place** (e.g. without assignment). | `VAR = {}` |
| print (operation) | A special operation that prints the output of an expression to stdout | `PRINT {{}, {{}}}` |

## Code Generator

The set-code-generator consists of three projects: set-analyzer, set-ir-generator, and set-target-generator.

### 1. Analyzer

The set-analyzer inputs a `.set` file that has correct syntax (has already been run through the parser) and outputs an AST structure that describes the syntax of the file.



#### SET Type Structure and Algorithm

Because sets are the only type in set-lang, it is the only type that needs to be implemented!

00 (0) - {}
01 (1) - {{}}
10 (2) - {{}, {{}}}
11 (3) - {{}, {{}}, {{{}}}}

{{{}}, {}}

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

### 2. IR Generator

The set-ir-generator inputs the AST from set-analyzer and outputs a `.set_ir` file that represents the logic of the analyzer using set-ir, an Intermediate Representation for set logic.

All abstract code optimizations are made at this stage.

### 3. Target Generator

The set-target-generator inputs the IR from the set-ir-generator and outputs a file with assembly code for the target machine.
