# Contributing to `tomu_usb_simple_client`

Thank you for your interest in contributing to this project!

## Scope

This library is for working with the [Tomu `usb_simple` sample app][usb-simple] in Rust. I'm not
interested in supporting other hardware / apps – you may want to write your own library instead. :)

This targets the `nusb` Rust USB library. Unless there is an _exceptionally compelling_ reason, I'm
not interested in supporting other USB libraries.

On the Rust side, this project targets the current stable version of Rust on
[64-bit platforms with Tier 1 with Host Tools][rust-tier]. Support for stable versions up to 1 year
old is on a "best effort" basis, and support for other platforms is on a "if you do the work, it's
simple and testable in CI" basis.

End-to-end cases are generally difficult to run in CI because they require physical hardware.

[usb-simple]: https://github.com/im-tomu/tomu-samples/tree/master/usb_simple

## Submitting issues

- Issues must be written in English.

- If you've found a bug, include a simple way to reproduce it. I don't have access to your code
  base, or your computer.

- [Be mindful of the scope](#scope).

- Don't report that a dependency is out of date. _Write a PR instead!_

- If you have a question about how something in _this_ repository works, please use
  [the discussions tab][discussions].

  The goal is that these will be answered by the documentation.

Issues and questions sent by email will attract my professional consulting rates. :)

## Submitting PRs

- _Write an issue report first for any non-trivial change._ This will help find if someone else is
  working on the issue, or if the work is even worth persuing (ie: in scope).
- PR descriptions must be written in English.
- All contributions **must be your own work**.
- Automated PRs and the use of generative AI **is not welcome here**.
- Keep your PR small and simple. Focus on one issue at a time.
- Include test cases.
- Avoid mixing whitespace and formatting changes with your change.

Examples of welcomed contributions:

- Expanding test cases
- Expanding CI coverage
- Fixing documentation issues
- Mocking hardware access for CI
- Updating outdated dependencies

[discussions]: https://github.com/micolous/tomu_usb_simple_client/discussions
[rust-tier]: https://doc.rust-lang.org/nightly/rustc/platform-support.html
