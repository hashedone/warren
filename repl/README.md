Very simple prolog-like language repl, as an example usage of [warren](https://github.com/hashedone/warren) crate.

### Syntax
#### Identifiers
Identifiers are any alphanumeric strings, possibly containing `_`, but
not starting with number.

#### Terms
Terms are just identifiers. Structured terms are identifiers followed
by their subterms enclosed in bractets (like `a(foo, bar)`)

#### Variables
Variables are identifiers like terms, but are starting with `?`, eg. `?X`.
Variables are substitutions for terms, and can be used in most context
where `Term` can be used.

#### Queries
Queries are top-level terms ending with `?` mark, eg. `a(foo, ?X)?`.
