# Build

[Back to Workflow](index.md)

## Steps

```bash

# Sync the repository
git fetch
git checkout main
git pull

# Build this package's and its dependencies' documentation
# cargo doc
cargo d

# rustdoc
```

Use cargo doc to build documentation in target/doc, cargo doc --open will automatically open it in your web browser.

Use cargo test to run all tests (including documentation tests), and cargo test --doc to only run documentation tests.

These commands will appropriately invoke rustdoc (and rustc) as required.
