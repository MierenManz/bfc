# (probably) Common BrainF*ck Loop Patterns

`A` and `T*` are magic variables. A is the memory ptr that was used when the instruction got called.\
`T*` is a temporary variable that can be optmized out of wanted

## Table of content
- [Math Operations](#math-operations)

## Math Operations

### movadd
Add A to B by moving A into B

```rs
movadd(B):
[
  A  ValDecr
  A  MovePtr(dest = B)

  B  ValIncr
  B  MovePtr(dest = A)
]
```

### add value
Add A to B by copying A into B

First part of the instruction
```rs
add(B):
[
  A  ValDecr
  A  MovePtr(dest = T)

  T  ValIncr
  T  MovePtr(dest = B)

  B  ValIncr
  B  MovePtr(dest = A)
]

A  MovePtr(dest = T)
T  movadd(A)
```

### movsub
Subtract A from B by negating both until A = 0

```rs
movsub(B):
[
  A  ValDecr
  A  MovePtr(dest = B)

  B  ValDecr
  B  MovePtr(dest = A)
]
```

### sub
Subtract A from B while only negating B
[0, 3, 0] 
```rs
sub(B):
[
  A  ValDecr
  A  MovePtr(dest = T)

  T  ValIncr
  T  MovePtr(dest = B)

  B  ValDecr
  B  MovePtr(dest = A)
]

A  MovePtr(dest = T)
T  movadd(A)
```

## Idk yet

### szero
```rs
szero():
[
  A  ValDecr
]
```

### semptyf
```rs
semptyf():
[
  T  PtrInc  
]
```

### semptyb
```rs
semptyb():
[
  T  PtrDecr  
]
```