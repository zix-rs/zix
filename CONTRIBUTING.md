# Contributing to Zix

Thank you for considering contributing to Zix! Contributions, whether they're bug reports, feature requests, or pull requests, are greatly appreciated. To ensure a smooth collaboration, please follow the guidelines below.

---
## Getting Started

1. **Fork the Repository**
   Fork the repository to your GitHub account and clone it locally. Make sure you are working on the latest version of the main branch:
   ```bash
   git clone https://github.com/arkeasz/zix.git
   cd zix
   git pull origin main
   ```

3. **Install Dependencies**
   If the project has additional dependencies, make sure to install them as per the instructions in the README.

4. **Run Tests**
   Before making changes, ensure all tests pass:
   ```bash
   cargo test
   ```

---

## Contribution Guidelines

### Code Style

- Follow the Rust coding conventions outlined in [rustfmt](https://github.com/rust-lang/rustfmt). Run `cargo fmt` before submitting changes.
- Ensure your code is lint-free by running `cargo clippy`.

### Commit Messages

Use meaningful commit messages with one of the following prefixes to describe the purpose of your change:

| **Prefix**   | **Use Case**                                    |
|--------------|-------------------------------------------------|
| `feat:`      | Adding a new feature                            |
| `fix:`       | Fixing a bug                                   |
| `docs:`      | Documentation updates or improvements           |
| `refactor:`  | Code refactoring without changing functionality |
| `test:`      | Adding or improving tests                      |
| `chore:`     | Maintenance tasks (e.g., dependencies updates)  |
| `ci:`        | Changes to CI/CD pipelines                     |

**Example**:
```
feat: add support for environment variable configuration
```

You can also specify the feature your change is related to by including it in parentheses after the prefix. This makes it easier to track which feature the change impacts.

Examples:

For a new feature:
```
feat(installer): add workflow to update zix.json on release
```
```
fix(cli): correct parsing of command-line arguments
```

---

## Getting Started

1. **Fork the Repository**
   Fork the repository to your GitHub account and clone it locally. Make sure you are working on the latest version of the main branch:
   ```bash
   git clone https://github.com/your-username/zix.git
   cd zix
   git pull origin main


## Workflow

1. **Create a New Branch**
   Create a branch with a descriptive name for your contribution:
   ```bash
   git checkout -b feat-new-feature
   ```

2. **Make Changes and Commit**
   - Ensure your changes are properly tested.
   - Use the appropriate commit message prefix (see above).

3. **Push Your Changes**
   Push your branch to your fork:
   ```bash
   git push origin feat-new-feature
   ```

4. **Open a Pull Request**
   - Go to the original repository and open a pull request.
   - Provide a clear and concise description of your changes and their purpose.

---

## Reporting Issues

If you encounter any issues or bugs, please report them by opening a new issue in the repository. Include the following details:
- Steps to reproduce the issue
- Expected behavior
- Actual behavior
- Environment details (e.g., operating system, Rust version)
