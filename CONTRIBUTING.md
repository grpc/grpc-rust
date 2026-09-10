# How to contribute

We welcome your patches and contributions to gRPC! Please read the gRPC
organization's [governance
rules](https://github.com/grpc/grpc-community/blob/master/governance.md) before
proceeding.

If you are new to GitHub, please start by reading [Pull Request howto](https://help.github.com/articles/about-pull-requests/)

## Legal requirements

In order to protect both you and ourselves, you will need to sign the
[Contributor License
Agreement](https://identity.linuxfoundation.org/projects/cncf). When you create
your first PR, a link will be added as a comment that contains the steps needed
to complete this process.

## Getting Started

A great way to start is by searching through our open issues. [Unassigned issues
labeled as "help
wanted"](https://github.com/grpc/grpc-rust/issues?q=state%3Aopen%20label%3AE-help-wanted)
are especially nice for first-time contributors, as they should be well-defined
problems that already have agreed-upon solutions.

## Conduct

The `grpc-rust` project adheres to the [Rust Code of Conduct][coc]. This
describes the _minimum_ behavior expected from all contributors.

[coc]: https://rust-lang.org/policies/code-of-conduct/

## Generative AI Policy

AI tools have the ability to produce more code than is possible for the gRPC
team to read, understand, review, and accept into the repository.  For this
reason, we request that all contributions adhere to the following rules:

1. **No AI-Generated Interactions:** All communication in the repo must be
   authored by a human.  _Exception: AIs may be used for directed writing
   assistance or translation._  Absolutely no automated agents are allowed to
   directly publish to GitHub.

2. **Author Ownership and Accountability:** Code contributions are expected to
   be fully owned and understood by the human contributor.  If the code was
   produced by generative AI, the author is expected to have reviewed and
   understood it in its entirety before submitting it for review.  This includes
   all content: production code, tests, examples, tools, etc.

In addition to the above requirements, any AI-assisted contributions must also
comply with the [Linux Foundation Generative AI
Policy](https://www.linuxfoundation.org/legal/generative-ai).  This includes
confirming that all contributions are legally allowed to be contributed to the
gRPC project under the applicable license terms.

## Guidelines for Pull Requests

Please read the following carefully to ensure your contributions can be merged
smoothly and quickly.

### PR Contents

- Create **small PRs** that are narrowly focused on **addressing a single
  concern**. We often receive PRs that attempt to fix several things at the same
  time, and if one part of the PR has a problem, that will hold up the entire
  PR.

- If your change does not address an **open issue** with an **agreed
  resolution**, consider opening an issue and discussing it first. If you are
  suggesting a behavioral or API change, consider starting with a [gRFC
  proposal](https://github.com/grpc/proposal). Many new features that are not
  bug fixes will require cross-language agreement.

- If you want to fix **formatting or style**, consider whether your changes are
  an obvious improvement or might be considered a personal preference. If a
  style change is based on preference, it likely will not be accepted. If it
  corrects widely agreed-upon anti-patterns, then please do create a PR and
  explain the benefits of the change.

- For correcting **misspellings**, please be aware that we use some terms that
  are sometimes flagged by spell checkers. As an example, "if an only if" is
  often written as "iff". Please do not make spelling correction changes unless
  you are certain they are misspellings.

- **All tests need to be passing** before your change can be merged.  You can
  run many tests locally using `cargo`, but to ensure all tests are fully
  passing before opening your PR you can use github actions on your fork.

- If you are adding a **new file**, make sure it has the **copyright message**
  template at the top as a comment. You can copy the message from an existing
  file and update the year.

### PR Descriptions

- Read and follow the **guidelines for PR titles and descriptions** here:
  https://google.github.io/eng-practices/review/developer/cl-descriptions.html

  *particularly* the sections "First Line" and "Body is Informative".

  Note: your PR description will be used as the git commit message in a
  squash-and-merge if your PR is approved. We may make changes to this as
  necessary.

- **Does this PR relate to an open issue?** On the first line, please use the
  tag `Fixes #<issue>` to ensure the issue is closed when the PR is merged. Or
  use `Updates #<issue>` if the PR is related to an open issue, but does not fix
  it. Consider filing an issue if one does not already exist.

### PR Process

- Please **self-review** your code changes before sending your PR. This will
  prevent simple, obvious errors from causing delays.

- Maintain a **clean commit history** and use **meaningful commit messages**.
  PRs with messy commit histories are difficult to review and won't be merged.

- Before sending your PR, ensure your changes are based on top of the latest
  `upstream/master` commits, and avoid rebasing in the middle of a code review.
  You should **never use `git push -f`** unless absolutely necessary during a
  review, as it can interfere with GitHub's tracking of comments.

- Unless your PR is trivial, you should **expect reviewer comments** that you
  will need to address before merging.
