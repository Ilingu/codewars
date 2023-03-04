Write a function that returns a string in which firstname is swapped with last name.

**Example(Input --> Output)**

```
"john McClane" --> "McClane john"
```

~~~if:riscv
RISC-V: The function signature is:

```c
char *name_shuffler(char *shuffled, const char *name);
```

`name` is the input string and `shuffled` is the output buffer you should write to. You may assume the output buffer is large enough to hold the result. Return the output buffer.
~~~