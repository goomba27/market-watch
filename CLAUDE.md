# Rust Style: Google Rust Style Guide

Source: https://google.github.io/styleguide/rustguide.html

**Following this guide is encouraged, not required.** Apply it by default when writing or reviewing Rust. Deviate when a rule makes the code worse in context, and say why in the change. Project rules in `AGENTS.md` (and nested `AGENTS.md` files) take precedence wherever they conflict with this file.

This is an application, so the "application code" variant of each rule applies: favor simplicity and directness, build only what the immediate need requires, and avoid premature generalization.

## Baseline

- Follow the official Rust Style Guide and Rust API Guidelines unless a rule below says otherwise.
- `rustfmt` owns formatting. Suggested `rustfmt.toml` (nightly options; skip them if the project must build on stable):
  ```toml
  max_width = 100
  imports_granularity = "Module"
  hex_literal_case = "Lower"
  format_macro_matchers = true
  ```
- Use `#[rustfmt::skip]` only for deliberate manual alignment (e.g. a matrix) or generated code, never for personal preference.

## Modules and imports

- Name a leaf module and its file after its primary item: `Foo` lives in `mod foo` / `foo.rs`.
- Non-leaf modules hold only `mod` declarations and re-exports, plus at most a small amount of glue code.
- Avoid glob imports. Allowed: `use super::*` in inline (test) modules, `use MyEnum::*` inside functions centered on that enum, and `pub use submodule::*` in non-leaf modules.
- One module per `use` statement: `use foo::bar::{Bar, Baz};`, not `use foo::{Foo, bar::Bar};`.
- Import types and traits and use them unqualified (`Regex`). Qualify or rename ambiguous names (`io::Error` or `use std::io::Error as IoError`).
- Call free functions through their module: `mem::take(..)`, not a bare `take(..)`.
- Import traits inside the function that uses them, unless the file uses them heavily.

## Ownership and borrowing

- Prefer unique ownership and borrowing over `Rc` / `Arc`. Shared async state (such as `AppState` handed to Axum) is a legitimate `Arc` case.
- Clone only when a copy is genuinely needed. Cloning is fine when borrowing gets unreasonably complex.

## Function signatures

- Take the most flexible borrowed form: `&str` over `&String`, `&[T]` over `&Vec<T>`, `&T` over `&Box<T>`, `Option<&T>` over `&Option<T>`.
- Returning `Vec<T>` or accepting `&[T]` is fine in application code when it is simpler than iterators.
- Avoid `impl Into<T>` parameters in application code.
- Prefer an `enum` over a `bool` parameter when the bool's meaning is unclear at the call site. If a bool stays, label it inline: `connect(addr, /* retry= */ true)`.

## Errors and panics

- `Option` for simple, obvious absence. `Result` for recoverable failures the caller needs to distinguish. `panic!` only for bugs and violated invariants.
- When a panic is truly necessary, use `expect("why this cannot fail")` rather than `unwrap()`. (Request-handling code still follows the `AGENTS.md` rule: no `unwrap` or `expect` at all.)
- Use pattern matching (`if let`, `let ... else`) instead of `is_some()` followed by `unwrap()`.
- Never read the contents of a poisoned mutex.
- Do not call `panic::catch_unwind`.
- Give `assert!` a context message when it explains intent, logs a useful value, or distinguishes caller misuse from an internal bug. The message adds information; it does not restate the condition.

## Types

- Use a newtype rather than a type alias when the type has distinct meaning, needs its own API, may change representation, or is easily confused with a sibling value (`Day`, `Month`, `Year`).
- Prefer named fields over tuples and multi-field tuple structs. Single-field newtypes are fine.

## Traits

- Add trait impls when they are needed, not speculatively.
- Implement the standard trait instead of a look-alike method: `impl FromStr`, not an inherent `from_str`.
- Use `#[derive(..)]` whenever it produces the right behavior.
- Define a trait only for generic bounds, `impl Trait` / `dyn Trait` usage, or an extension trait. Use inherent impls otherwise; traits are not a code-organization tool.
- Name extension traits with an `Ext` suffix (`ResponseExt`).
- Trait checklist:
  - `Clone` for most types, except those representing external resources.
  - `Copy` when the type will clearly stay `Copy` (including error types where possible).
  - `Default` when the type has `new()` or a natural default state. Options structs always get `Default`.
  - `Hash` + `Eq` (and `Ord` if ordering is obvious) for likely map keys.
  - Prefer `PartialEq` / `PartialOrd` unless reflexivity or total ordering is actually wanted.
- If a `From` impl is a main way to build a type, also provide an inherent constructor (`new`, `from_foo`).
- Implementing `len()` means also implementing `is_empty()`.

## Naming

- Follow std conventions for `as_`, `to_`, `into_` with the matching `self` receiver (Clippy `wrong_self_convention`).
- Constructors: `new()`, `new_with_foo()`, and `try_new()` when returning `Result`. Conversions: `from_foo()` paired with a `From` impl.
- Accessors: `foo()`, `foo_mut()`, `set_foo()`, `take_foo()`, `replace_foo()`, and closure-based `with_foo()` / `with_mut_foo()` for values behind `Mutex` or `RefCell`.
- Use descriptive lifetime names. Single letters are fine for `'_`, simple input/output relationships, or `for<'a>` bounds (up to two).
- Generic parameters: short names that hint at the bound (`R: Read`); `T` / `U` when unbounded.
- Spell words out. Use only abbreviations an outsider would recognize (`IpAddr`, `Http`).

## Macros

- Order of preference: plain functions, types, and traits, then `macro_rules!`, then other code generation, then proc macros. Each step down costs IDE support and readability.

## Documentation and comments

- Document public items with `///` and modules with `//!` at the top of the file, in Markdown with full sentences.
- Always document panic conditions and the invariants of `unsafe` functions and traits.
- Comments add information the code does not carry. Skip comments that restate a name or explain language basics.
- Use `//` for implementation notes and TODOs.

## Other features

- Write `unsafe` only when no safe implementation exists. Wrap it in a small safe abstraction away from business logic, and put a `// SAFETY:` comment on each block explaining why it is sound.
- Stay on stable Rust: no unstable features.
- Overload operators only for their mathematical or logical meaning.
- Mark a public `fn` as `const` only if it will stay `const`.
- `#[must_use]` on functions whose return value must not be ignored (unless the return type is already `#[must_use]`, like `Result`), and on lazily evaluated types.
- When matching a `#[non_exhaustive]` enum, use a wildcard arm for ignored variants, or `if let` / `let ... else` when only one variant matters.
- Do not run code before `main()` (no `ctor`-style crates).

## Expressions

- Annotate types where a reader could not easily infer them, especially after `.into()`, `.parse()`, `.collect()`, and long iterator chains. Prefer `let x: Vec<_> = ..` over turbofish.
- Prefer a `for` loop over long combinator chains with conditionals or side effects, and over `Iterator::for_each`. Short chains and encapsulating combinators (`any`, `find`) are good.
- Bind an iterator chain to a named variable rather than building it in a `for` header.
- Use `format!` / `write!` to join more than two strings or to substitute into a template, rather than `+` or `str::replace`.
