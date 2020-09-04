## Contribution Guidelines

Please note that this project is released with a [Contributor Code of Conduct](code-of-conduct.md). By participating in this
project you agree to abide by its terms.

---

### Why contribute?

ShowMe emerged as a personal project to practice the programming language Rust. For this reason, we encourage everyone who is on the same path to contribute code, ideas or suggestions. Let's do it!

### Propose an idea, suggestion or report a bug

If you want to propose an idea or report a bug, go to the [issues section](https://github.com/PandaFoss/showme/issues). Try to be as explicit as possible and add a title that summarizes it, putting the tag `[Bug]`, `[Idea]`, `[Question]`, etc. as appropriate. Oh, I almost forgot! Remember to search through the closed issues to see if your question/bug/idea was already made by another user before.

### About Rust code

Rust provides two extremely interesting tools when polishing our code (besides the compiler itself, which does an excellent job). I refer to `rustfmt` and `clippy`.
Once you have made the pertinent changes, try to pass both checks in a satisfactory way:

- For code linting:

`cargo fmt --all`

- For syntax correction:

`cargo clippy --all-targets --all-features`
