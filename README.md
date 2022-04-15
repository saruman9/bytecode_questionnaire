# Answers

## Bytecode interpreter

### Base

- [Commit](./commit/aaab3b179a2dd76d57a641f75a2135f3157286db)
- [Tree](./tree/aaab3b179a2dd76d57a641f75a2135f3157286db)

This is probably the worst code I've written in the past year. It has no documentation, is full of bugs, and is difficult to read. I didn't have enough time to fix it and optimize it.

### Loops

- [Compare](./compare/aaab3b1..2e784ae)
- [Tree](./tree/2e784ae)

See tests:

- [`jump_ret_x`](./blob/dbaa4f9f1826752d5d4558cd817b3ac6cde27c4d/bytecode_interpreter/src/lib.rs#L200)
- [`jump_ret_sum_of_xyw`](./blob/dbaa4f9f1826752d5d4558cd817b3ac6cde27c4d/bytecode_interpreter/src/lib.rs#L251)
- [`jump_and_jump`](./blob/dbaa4f9f1826752d5d4558cd817b3ac6cde27c4d/bytecode_interpreter/src/lib.rs#L310)
- [`jump_less_than_0`](./blob/dbaa4f9f1826752d5d4558cd817b3ac6cde27c4d/bytecode_interpreter/src/lib.rs#L377)
- [`jump_less_than_1`](./blob/dbaa4f9f1826752d5d4558cd817b3ac6cde27c4d/bytecode_interpreter/src/lib.rs#L420)
- [`jump_greater_than_0`](./blob/dbaa4f9f1826752d5d4558cd817b3ac6cde27c4d/bytecode_interpreter/src/lib.rs#L463)
- [`jump_greater_than_1`](./blob/dbaa4f9f1826752d5d4558cd817b3ac6cde27c4d/bytecode_interpreter/src/lib.rs#L506)
- [`jump_equal`](./blob/dbaa4f9f1826752d5d4558cd817b3ac6cde27c4d/bytecode_interpreter/src/lib.rs#L549)
- [`pow`](./blob/dbaa4f9f1826752d5d4558cd817b3ac6cde27c4d/bytecode_interpreter/src/lib.rs#L592) (`while` loop statement)
- [`fibonacci_space_optimized`](./blob/dbaa4f9f1826752d5d4558cd817b3ac6cde27c4d/bytecode_interpreter/src/lib.rs#L638) (`for` loop statement)

### Spawn and message passing

> "Talk is cheap. Show me the code."

― Linus Torvalds

- [Compare](./compare/2e784ae..dbaa4f9)
- [Tree](./tree/dbaa4f9)

At first, I misunderstood the task and started implementing concurrency at the stack machine level. I ran into a lot of problems, for example the ABA problem, I needed to implement the interrupt system and thread local storage or something similar. Then I understood, that I should implement it at interpreter level.

See tests:

- [`spawn`](./blob/dbaa4f9f1826752d5d4558cd817b3ac6cde27c4d/bytecode_interpreter/src/lib.rs#L709)
- [`send_recv`](./blob/dbaa4f9f1826752d5d4558cd817b3ac6cde27c4d/bytecode_interpreter/src/lib.rs#L735)
- I start to implement [`fibonacci_multithreaded_without_caching`](./blob/dbaa4f9f1826752d5d4558cd817b3ac6cde27c4d/bytecode_interpreter/src/lib.rs#L774), but time is up

## Custom `walkdir`

- [Tree](./tree/269d8de)

It is not clear from the text of the task how exactly I should solve the task (to parallelize the workers, to optimize of counting of lines or something else), so I did it as simply as possible, without using third-party libraries.

## Blockchain

I know the answers to some questions. I googled the rest of the questions and got acquainted, it was interesting. I see no point in copying information from the Internet here.
