# Creating a Release

[GitHub](https://github.com/orhun/binsider/releases) and [crates.io](https://crates.io/crates/binsider/) releases are automated via [GitHub actions](.github/workflows/cd.yml) and triggered by pushing a tag.

1. Bump the version in [Cargo.toml](Cargo.toml) according to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
2. Update [Cargo.lock](Cargo.lock) by building the project: `cargo build`
3. Update [CHANGELOG.md](CHANGELOG.md) by running [`git-cliff`](https://git-cliff.org): `git cliff -u -t v[X.Y.Z] -p CHANGELOG.md`
4. Commit your changes: `git add -A && git commit -m "chore(release): bump version"`
5. Pull existing tags: `git fetch --tags`
6. Create a new tag: `git tag v[X.Y.Z]`
7. Push the tag: `git push --tags`
8. Announce the release! 🥳
