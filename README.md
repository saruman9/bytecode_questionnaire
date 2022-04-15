# Answers

- [Answers](#answers)
  - [Bytecode interpreter](#bytecode-interpreter)
    - [Base](#base)
    - [Loops](#loops)
    - [Spawn and message passing](#spawn-and-message-passing)
  - [Custom `walkdir`](#custom-walkdir)
  - [Blockchain](#blockchain)

## Bytecode interpreter

### Base

- [Commit](https://github.com/saruman9/bytecode_questionnaire/commit/aaab3b179a2dd76d57a641f75a2135f3157286db)
- [Tree](https://github.com/saruman9/bytecode_questionnaire/tree/aaab3b179a2dd76d57a641f75a2135f3157286db)

This is probably the worst code I've written in the past year. It has no documentation, is full of bugs, and is difficult to read. I didn't have enough time to fix it and optimize it.

### Loops

- [Compare](https://github.com/saruman9/bytecode_questionnaire/compare/aaab3b1..2e784ae)
- [Tree](https://github.com/saruman9/bytecode_questionnaire/tree/2e784ae)

See tests:

- [`jump_ret_x`](https://github.com/saruman9/bytecode_questionnaire/blob/dbaa4f9f1826752d5d4558cd817b3ac6cde27c4d/bytecode_interpreter/src/lib.rs#L200)
- [`jump_ret_sum_of_xyw`](https://github.com/saruman9/bytecode_questionnaire/blob/dbaa4f9f1826752d5d4558cd817b3ac6cde27c4d/bytecode_interpreter/src/lib.rs#L251)
- [`jump_and_jump`](https://github.com/saruman9/bytecode_questionnaire/blob/dbaa4f9f1826752d5d4558cd817b3ac6cde27c4d/bytecode_interpreter/src/lib.rs#L310)
- [`jump_less_than_0`](https://github.com/saruman9/bytecode_questionnaire/blob/dbaa4f9f1826752d5d4558cd817b3ac6cde27c4d/bytecode_interpreter/src/lib.rs#L377)
- [`jump_less_than_1`](https://github.com/saruman9/bytecode_questionnaire/blob/dbaa4f9f1826752d5d4558cd817b3ac6cde27c4d/bytecode_interpreter/src/lib.rs#L420)
- [`jump_greater_than_0`](https://github.com/saruman9/bytecode_questionnaire/blob/dbaa4f9f1826752d5d4558cd817b3ac6cde27c4d/bytecode_interpreter/src/lib.rs#L463)
- [`jump_greater_than_1`](https://github.com/saruman9/bytecode_questionnaire/blob/dbaa4f9f1826752d5d4558cd817b3ac6cde27c4d/bytecode_interpreter/src/lib.rs#L506)
- [`jump_equal`](https://github.com/saruman9/bytecode_questionnaire/blob/dbaa4f9f1826752d5d4558cd817b3ac6cde27c4d/bytecode_interpreter/src/lib.rs#L549)
- [`pow`](https://github.com/saruman9/bytecode_questionnaire/blob/dbaa4f9f1826752d5d4558cd817b3ac6cde27c4d/bytecode_interpreter/src/lib.rs#L592) (`while` loop statement)
- [`fibonacci_space_optimized`](https://github.com/saruman9/bytecode_questionnaire/blob/dbaa4f9f1826752d5d4558cd817b3ac6cde27c4d/bytecode_interpreter/src/lib.rs#L638) (`for` loop statement)

### Spawn and message passing

> "Talk is cheap. Show me the code."

― Linus Torvalds

- [Compare](https://github.com/saruman9/bytecode_questionnaire/compare/2e784ae..dbaa4f9)
- [Tree](https://github.com/saruman9/bytecode_questionnaire/tree/dbaa4f9)

At first, I misunderstood the task and started implementing concurrency at the stack machine level. I ran into a lot of problems, for example the ABA problem, I needed to implement the interrupt system and thread local storage or something similar. Then I understood, that I should implement it at interpreter level.

See tests:

- [`spawn`](https://github.com/saruman9/bytecode_questionnaire/blob/dbaa4f9f1826752d5d4558cd817b3ac6cde27c4d/bytecode_interpreter/src/lib.rs#L709)
- [`send_recv`](https://github.com/saruman9/bytecode_questionnaire/blob/dbaa4f9f1826752d5d4558cd817b3ac6cde27c4d/bytecode_interpreter/src/lib.rs#L735)
- I start to implement [`fibonacci_multithreaded_without_caching`](https://github.com/saruman9/bytecode_questionnaire/blob/dbaa4f9f1826752d5d4558cd817b3ac6cde27c4d/bytecode_interpreter/src/lib.rs#L774), but time is up

## Custom `walkdir`

- [Tree](https://github.com/saruman9/bytecode_questionnaire/tree/269d8de)

It is not clear from the text of the task how exactly I should solve the task (to parallelize the workers, to optimize of counting of lines or something else), so I did it as simply as possible, without using third-party libraries.

## Blockchain

I know the answers to some questions. I googled the rest of the questions and got acquainted, it was interesting. I see no point in copying information from the Internet here.
