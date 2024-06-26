# Quoth

![quoth raven](quoth-raven.svg)

[![Crates.io](https://img.shields.io/crates/v/quoth)](https://crates.io/crates/quoth)
[![docs.rs](https://img.shields.io/docsrs/quoth?label=docs)](https://docs.rs/quoth/latest/quoth/)
[![Build Status](https://img.shields.io/github/actions/workflow/status/sam0x17/quoth/ci.yaml)](https://github.com/sam0x17/quoth/actions/workflows/ci.yaml?query=branch%3Amain)
[![MIT License](https://img.shields.io/github/license/sam0x17/quoth)](https://github.com/sam0x17/quoth/blob/main/LICENSE)

**Quoth** is a _scannerless_ parsing library (meaning there is no lexing/tokenization step)
specifically designed for implementing languages and DSLs (domain specific languages) in Rust.
It is based on the admittedly [dtolnayian](https://crates.io/users/dtolnay) idea from
[syn](https://crates.io/crates/syn) that everything should implement the same `Parse` trait,
however quoth takes this idea further to the point where lexing is no longer necessary, and
what you are left with is something akin to "Object Oriented Parsing" where it is quite easy to
compose, combine, parse, and even "unparse" `Parsable`s in a myriad of ways.

## Parsing

In quoth, everything implements `Parsable`, which brings with it a large set of requirements
(and thus features) that are at best conventions in other parsing ecosystems. Some core
features of quoth include:
- anything that can be parsed with quoth can also be "unparsed" i.e. converted back to a string
- because there is no tokenization step, the unmodified `Span` source text for any `Parsable`
  is _always_ available and is cheap/free to access at any time during parsing
- `Span` itself is very lightweight and is just a reference-counted string slice into a
  `Source`
- because of this, `ParseStream` is also incredibly lightweight and provides normally expensive
  operations like forking virtually for free
- in quoth, you can peek by `Parsable` type, but you can also peek _by value_, and even _by
  regex_
- branching and ambiguity are much easier to deal with in quoth because forking is cheap and
  encouraged. This is a double-edged sword because it means you can efficiently parse ambiguous
  things that are normally inefficient to parse and hard to reason about, but now it is much
  easier to introduce ambiguity into your underlying grammar.

## Peeking

Quoth also takes special care to make anything that is `Parsable` also automatically
`Peekable`, making it even easier to implement `Parsable`. Furthermore, `Peekable` itself
allows peeking on both a type-basis, _and_ on a specific value basis. For example, you could
peek "is an `Ident` next?" or "is an `Ident` with _this specific value_ next?".

More information and docs will be coming in the next release