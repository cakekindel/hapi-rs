# cargo-make
This crate uses [`cargo-make`] for script consistency, in Makefile.toml you'll find:
  - `cargo make fmt`: Format all files according to configured style `rustfmt.toml`
  - `cargo make test`: Run all tests
  - `cargo make doctest`: Run doc tests only
  - `cargo make tdd`: Watch files for changes, and run `cargo make test` on each change
  - `cargo make ci`: Run tests, check that code is formatted and no lint violations.
                     This is run as a quality gate for all pull requests.
  - `cargo make update-readme`: Regenerate README.md based on `src/lib.rs` and `./README.tpl`.

# CI/CD
> Note: requires following [conventional commits].

On Pull Request -> main:
  - run `cargo make ci` (test && rustfmt --check && clippy)

On Pull Request merge -> main:
  - Uses [`standard-version`] (bump version & update CHANGELOG)
  - Pushes `chore(release): vX.X.X`
  - Pushes tag `vX.X.X`

On tag push:
  - Publish new GitHub release
  - Publish new version to crates.io

[`cargo-make`]: https://github.com/sagiegurari/cargo-make/
[`cargo-readme`]: https://github.com/livioribeiro/cargo-readme
[`standard-version`]: https://www.npmjs.com/package/standard-version
[conventional commits]: https://www.conventionalcommits.org/en/v1.0.0/
