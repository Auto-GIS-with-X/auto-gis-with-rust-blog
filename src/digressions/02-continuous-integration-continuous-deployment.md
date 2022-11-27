---
title: "2: Continuous integration / Continuous deployment"
eleventyNavigation:
  key: "2: Continuous integration / Continuous deployment"
  parent: Digressions
  order: 1
---

Coming from Python, my approach to Continuous integration / Continuous deployment (CI/CD) is heavily influenced by Claudio Jolowicz' excellent [Hypermodern Python](https://cjolowicz.github.io/posts/hypermodern-python-01-setup/) series.

## For the Rust repo

This approach is very similar to the one that Luca Palmieri advocates for in the [continuous integration section](https://www.lpalmieri.com/posts/2020-06-06-zero-to-production-1-setup-toolchain-ides-ci/#6-continuous-integration) of the draft version of his [Zero To Production In Rust](https://www.zero2prod.com).

However, Luca uses [`actions-rs`](https://github.com/actions-rs), which is, unfortunately, [currently unmaintained](https://github.com/actions-rs/toolchain/issues/216).

[Like many others](https://www.reddit.com/r/rust/comments/vyx4oj/comment/ig54zv7/?utm_source=share&utm_medium=web2x&context=3), I have, therefore, switched to using David Tolnay's [Rust Toolchain GitHub Action](https://github.com/dtolnay/rust-toolchain).

### Continuous integration workflow

Format on push or pull with `cargo fmt`

```yml
# ci.yml

name: Continuous integration
on: [push, pull_request]

jobs:
  format:
    name: Format
    runs-on: ubuntu-latest
    steps:
      - name: Checkout repo
        uses: actions/checkout@v3

      - name: Install Rust
        uses: dtolnay/rust-toolchain@v1
        with:
          toolchain: stable
          components: rustfmt

      - name: Run `fmt`
        run: |
          cargo fmt --all
```

Commit any changes

```diff-yml
# ci.yml

# ...

      - name: Run `fmt`
        run: |
          cargo fmt --all
+        
+      - name: Commit and push
+        run: |
+          git config --global user.name 'Your name'
+          git config --global user.email 'Your@email.address'
+          git git commit -am "Format with `cargo fmt`"
+          git push
```

Use secrets instead of hard coding git user info

```diff-yml
# ci.yml

# ...

      - name: Commit and push
        run: |
-          git config --global user.name 'Your name'
+          git config --global user.name ${{ secrets.NAME }}
-          git config --global user.email 'Your@email.address'
+          git config --global user.email ${{ secrets.EMAIL }}
          git git commit -am "Format with `cargo fmt`"
          git push
```

Only commit if there were changes

```diff-yml
# ci.yml

# ...

      - name: Commit and push
        run: |
          git config --global user.name ${{ secrets.NAME }}
          git config --global user.email ${{ secrets.EMAIL }}
-          git git commit -am "Format with `cargo fmt`"
+          git add . && git diff --staged --quiet || git commit -m "Format with `cargo fmt`"
          git push
```

Add linting with `clippy`:

```diff-yml
# ci.yml

# ...

      - name: Commit and push
        run: |
          git config --global user.name ${{ secrets.NAME }}
          git config --global user.email ${{ secrets.EMAIL }}
          git add . && git diff --staged --quiet || git commit -m "Format with `cargo fmt`"
          git push
+  
+  lint:
+    name: Lint
+    runs-on: ubuntu-latest
+    needs: format
+    steps:
+      - name: Checkout repo
+        uses: actions/checkout@v3
+
+      - name: Install Rust
+        uses: dtolnay/rust-toolchain@v1
+        with:
+          toolchain: stable
+          components: clippy
+
+      - name: Run `clippy`
+        run: |
+          cargo clippy -- -D warnings
```

Add tests with `test`:

```diff-yml
# ci.yml

# ...

      - name: Run `clippy`
        run: |
          cargo clippy -- -D warnings

+  test:
+    name: Test
+    runs-on: ubuntu-latest
+    needs: format
+    steps:
+      - name: Checkout repo
+        uses: actions/checkout@v3
+
+      - name: Install Rust
+        uses: dtolnay/rust-toolchain@v1
+        with:
+          toolchain: stable
+
+      - name: Run tests
+        run: |
+          cargo test
```

### Security audit workflow

```yml
name: Security audit

on:
  push:
    paths:
      - '**/Cargo.toml'
      - '**/Cargo.lock'
  schedule:
    - cron: '0 0 * * *'

jobs:
  audit:
    name: Audit dependencies
    runs-on: ubuntu-latest
    steps:
      - name: Checkout repo
        uses: actions/checkout@v3

      - name: Install Rust
        uses: dtolnay/rust-toolchain@v1
        with:
          toolchain: stable

      - name: Run audit
        run: |
          cargo install cargo-audit
          cargo audit
```