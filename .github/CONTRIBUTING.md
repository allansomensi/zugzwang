# Contributing 🚀

Thank you for considering contributing to **Zugzwang**! Your contributions help improve the game and make it better for everyone. Below is a guide to help you get started with the contribution process.

## How to Contribute

### 1. Fork the Repository

To contribute, fork the repository to your GitHub account. This allows you to make changes freely without affecting the main project until you're ready to submit them.

### 2. Create a New Branch 🌱

Create a new branch for your changes to keep your work isolated and make it easier to submit a pull request later. You can create a new branch using the following command:

```bash
git checkout -b your-branch
```

### 3. Linting and Code Quality 🧹

Before submitting your changes, please make sure that your code follows the project's coding standards. We recommend using the `just` command to perform linting and check your code with **Clippy**. To do this, run:


```elixir
just lint
```

This will ensure your code is free of common issues and adheres to the project's style guidelines.

### 4. Writing Tests 🧪

If you are adding new functionality or modifying existing code, please write tests to ensure that everything works as expected. The tests help maintain the general reliability and stability.

### 5. Commit Style 💬

Please follow the [Conventional Commits](https://www.conventionalcommits.org/) format. Example:

```
feat(pieces): draw pieces on the board
```

Optionally, you can include emojis to add expressiveness and visual cues to your commits:

```
feat(pieces): :sparkles: draw pieces on the board
```

If you're using **VS Code**, you can simplify this process by installing the [Conventional Commits extension](https://marketplace.visualstudio.com/items?itemName=vivaxy.vscode-conventional-commits), which assists you in formatting your commit messages consistently.

### 5. Submit a Pull Request 📤

Once you've made your changes, it's time to submit a pull request. When doing so, make sure to:

- Provide a clear and descriptive title for your pull request.
- Include a summary of the changes you've made and why they are necessary.
- Reference any issues that your pull request addresses (if applicable).
- Make sure your pull request is targeting the main branch.

We truly appreciate your contributions! Every improvement, big or small, helps make a better game for everyone. If you have any questions or need help, feel free to ask. We're here to help!

Happy coding! 🎉
