
We welcomes contribution from everyone in the form of suggestions, bug reports, pull requests, and
feedback. This document gives some guidance if you are thinking of helping us.

## Submitting bug reports and feature requests

You can create issues [here](https://github.com/zeenix/gimoji/issues/new). When reporting a bug or
asking for help, please include enough details so that the people helping you can reproduce the
behavior you are seeing.

When making a feature request, please make it clear what problem you intend to solve with the
feature, any ideas for how the crate in question could support solving that problem, any possible
alternatives, and any disadvantages.

## Submitting Pull Requests

Same rules apply here as for bug reports and feature requests. Plus:

* We prefer atomic commits. Please read
  [this excellent blog post](https://www.aleksandrhovhannisyan.com/blog/atomic-git-commits/) for
  more information, including the rationale.
* Please try your best to follow [these guidelines](https://wiki.gnome.org/Git/CommitMessages) for
  commit messages.
* Add details to each commit about the changes it contains. PR description is for summarizing the
  overall changes in the PR, while commit logs are for describing the specific changes of the
  commit in question.
* When addressesing review comments, fix the existing commits in the PR (rather than adding
  additional commits) and force push (as in `git push -f`) to your branch. You may find
  [`git-absorb`](https://github.com/tummychow/git-absorb) and
  [`git-revise`](https://github.com/mystor/git-revise) extremely useful, especially if you're not
  very familiar with interactive rebasing and modifying commits in git.

### Legal Notice

When contributing to this project, you **implicitly** declare that you:

* have authored 100% of the content.
* have the necessary rights to the content.
* agree to providing the content under the [project's license](LICENSE-MIT).

## Running the test suite

We encourage you to check that the test suite passes locally before submitting a pull request with
your changes. If anything does not pass, typically it will be easier to iterate and fix it locally
than waiting for the CI servers to run tests for you.

```sh
# Run the full test suite, including doc test and compile-tests
cargo test --all-features
```

Also please ensure that code is formatted correctly by running:

```sh
cargo +nightly fmt --all
```

and clippy doesn't see anything wrong with the code:

```sh
cargo clippy -- -D warnings
```

Please not that there are times when clippy is wrong and you know what you are doing. In such cases,
it's acceptable to tell clippy to
[ignore the specific error or warning in the code](https://github.com/rust-lang/rust-clippy#allowingdenying-lints).

If you intend to contribute often or think that's very likely, we recommend you setup the following git
hooks:

* Pre-commit hook that goes in the `.git/hooks/pre-commit` file:

  ```sh
  if ! which rustup &> /dev/null; then
      curl https://sh.rustup.rs -sSf  | sh -s -- -y
      export PATH=$PATH:$HOME/.cargo/bin
      if ! which rustup &> /dev/null; then
          echo "Failed to install rustup"
      fi
  fi

  if ! rustup component list --toolchain nightly|grep 'rustfmt-preview.*(installed)' &> /dev/null; then
      echo "Installing nightly rustfmt.."
      rustup component add rustfmt-preview --toolchain nightly
      echo "rustfmt installed."
  fi

  echo "--Checking style--"
  cargo +nightly fmt --all -- --check
  if test $? != 0; then
      echo "--Checking style fail--"
      echo "Please fix the above issues, either manually or by running: cargo +nightly fmt --all"

      exit -1
  else
      echo "--All very stylish 😎--"
  fi
  ```

* Pre-push hook that goes in the `.git/hooks/pre-push` file:

  ```sh
  if ! which rustup &> /dev/null; then
      curl https://sh.rustup.rs -sSf  | sh -s -- -y
      export PATH=$PATH:$HOME/.cargo/bin
      if ! which rustup &> /dev/null; then
          echo "Failed to install rustup"
      fi
  fi

  if ! rustup component list --toolchain stable|grep 'clippy.*(installed)' &> /dev/null; then
      echo "Installing clippy.."
      rustup component add clippy
      echo "clippy installed."
  fi

  echo "--Analysing code 🔍--"
  cargo clippy -- -D warnings
  if test $? != 0; then
      echo "--Issues with code. See warnings/errors above--"

      exit -1
  else
      echo "--Code looks good 👍--"
  fi
  ```

## Conduct

We follow the [Rust Code of Conduct](https://www.rust-lang.org/conduct.html). For escalation or
moderation issues please contact Zeeshan (zeeshanak@gnome.org) instead of the Rust moderation team.
