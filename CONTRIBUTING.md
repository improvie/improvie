# Contribution Guidelines

Thank you for your interest in contributing to the `improvie` project! This guide explains how you can contribute to the project.

## 🛠 Types of Contributions

You can contribute to the project in the following ways:

1. **Bug Reports**
   If you find any issues with the application, please report them on [GitHub Issues](https://github.com/improvie/improvie/issues).

2. **Feature Suggestions**
   If you have new feature ideas or improvements, please propose them on [GitHub Issues](https://github.com/improvie/improvie/issues).

3. **Code Improvements**
   You can fix bugs or implement new features and submit a pull request.

4. **Documentation Improvements**
   Corrections of typos or additions to the documentation are also welcome.

5. **Adding Tests**
   Contribute to improving the quality of the application by adding test cases.

## 🛠 Coding Style

- **JavaScript/TypeScript (Svelte)**
  - Use `eslint` to check code style. Run `bun run lint` and use `bun run lint:fix` for automatic fixes.
  - Use `svelte-check` for type checking. Run `bun run check` to verify.

- **Rust**
  - Use `rustfmt` to format code. Run `cargo fmt` for automatic formatting.
  - Use `cargo clippy` for code linting.

- **Editor Settings**
  - Follow the `.editorconfig` for tab and indent settings.

## 📚 Documentation

- If you add features or change specifications, be sure to update or add related documentation.
- Write documentation clearly and concisely, including sample code if necessary.

## 🧪 Testing

- Ensure all tests pass before pushing changes.
  - Use `bun test` to run tests under `src`.
  - Use `cargo ci` to run tests under `src-tauri`.

## 🔄 Git & GitHub

- Write commit messages that clearly describe the changes.
- When creating a pull request, link related issues and clearly explain the purpose of the changes.
