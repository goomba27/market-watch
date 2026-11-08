# Market Watch

A stock watchlist applet for the COSMIC™ desktop. Group ticker symbols into watchlists and check their prices from the panel.

I am building this after finishing the Rust book, as a good next step.

> [!NOTE]
> This is still a work in progress.

<img width="350" height="350" alt="Screenshot_2026-11-07_19-53-52" src="https://github.com/user-attachments/assets/1f0ee6a8-5450-4f9e-b985-97937c6a9017" />
<img width="350" height="350" alt="Screenshot_2026-11-07_19-53-42" src="https://github.com/user-attachments/assets/fb1a6550-7ae1-4d07-9817-815c09c58cb5" />
<img width="350" height="350" alt="Screenshot_2026-11-07_19-53-36" src="https://github.com/user-attachments/assets/4f0c968a-b121-4df9-831d-7fc5db201a53" />

## Installation

A [justfile](./justfile) is included by default for the [casey/just][just] command runner.

- `just` builds the application with the default `just build-release` recipe
- `just run` builds and runs the application
- `just install` installs the project into the system
- `just vendor` creates a vendored tarball
- `just build-vendored` compiles with vendored dependencies from that tarball
- `just check` runs clippy on the project to check for linter warnings
- `just check-json` can be used by IDEs that support LSP

## Translators

[Fluent][fluent] is used for localization of the software. Fluent's translation files are found in the [i18n directory](./i18n). New translations may copy the [English (en) localization](./i18n/en) of the project, rename `en` to the desired [ISO 639-1 language code][iso-codes], and then translations can be provided for each [message identifier][fluent-guide]. If no translation is necessary, the message may be omitted.

## Packaging

If packaging for a Linux distribution, vendor dependencies locally with the `vendor` rule, and build with the vendored sources using the `build-vendored` rule. When installing files, use the `rootdir` and `prefix` variables to change installation paths.

```sh
just vendor
just build-vendored
just rootdir=debian/market-watch prefix=/usr install
```

It is recommended to build a source tarball with the vendored dependencies, which can typically be done by running `just vendor` on the host system before it enters the build environment.

## Developers

Developers should install [rustup][rustup] and configure their editor to use [rust-analyzer][rust-analyzer].

[fluent]: https://projectfluent.org/
[fluent-guide]: https://projectfluent.org/fluent/guide/hello.html
[iso-codes]: https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes
[just]: https://github.com/casey/just
[rustup]: https://rustup.rs/
[rust-analyzer]: https://rust-analyzer.github.io/
[sccache]: https://github.com/mozilla/sccache
