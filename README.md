## What's that?

A syntax analyzer written in Rust as part of one of my university course subjects.

The current version knows how to define chains of the `xyz` kind with any number (greater than 0) of `x`, `y` and `z`.
That is, the chain `xyz` is the minimal possible chain.

Анализатор использует состояния для определения действий над следующим символом в цепочке.
Здесь состояния `S`, `F` и `E` отвечают за старт, финиш и ошибку соответственно.
`A`, `B` и `C` определяют текущий символ цепочки (`x`, `y` и `z` соответственно).

На графе это выглядит следующим образом:

![graph](docs/graph.png)
