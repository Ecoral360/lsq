# Lisp Query

Like [json query](https://github.com/jqlang/jq) but for Lisp.

**Table of Contents**

- [Installation](#installation)
- [How to use](#how-to-use)
- [The lsq language](#the-lsq-language)
- [Examples](#examples)
- [License](#license)

## Installation

```sh
git clone https://github.com/Ecoral360/lsq.git
cargo install --path ./lsq
lsq --version  # will display the version if the installation was successful
```

## How to use

```sh
lsq 'query' file-name.scm
# OR
some-op | lsq 'query'
```

## The lsq language

The `lsq` language is a query language made to access and traverse s-expression with
ease.

A `lsq` query is made up of **filters** separated by the `|` symbol. The list of

> While the number of builtin filters and builtin functions provided by `lsp` is low at the moment, it is rapidly expanding.
> If the language doesn't support something you need, feel free to open an Issue!

## Examples

You have a `people.scm` file:

```scm
((name "Mathis" info (age 20))
 (name "Jean" info (age 28))
 (name "Alice" info (age 12))
 (name "Enric" info (age 20))
 (name "Claudia" info (age 18))
 (name "Amelie" info (age 21))
 (name "Bob" info (age 16)))

```

and you want to get the name of everyone who is major (18+). You could write this query:
```sh
cat people.scm | lsq ';() | select ;(;info;age) >=? 18'
```

## License

`lsq` is distributed under the terms of the [MIT](https://spdx.org/licenses/MIT.html) license.
