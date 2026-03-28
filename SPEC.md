# Axioms

## 0. There are only sets

The set-compiler is made **entirely up of sets**. There are no built-in int, float, or boolean types. Instead,
you are encouraged to build your own using set theory! There are multiple ways of representing these
data types using sets, and set-compiler does not prescribe to one specific approach.

### Natural numbers as von Neumann ordinals

Natural numbers can be represented by sets. When set-compiler needs to return a numeric value,
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

Sets can contain other sets.

```
A = {}
B = {A, A}
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
