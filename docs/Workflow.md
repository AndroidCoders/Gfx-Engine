# Agile Development Workflow

**TLDR:**
* This document outlines the Agile development workflow for the project.
* We work in short, iterative **Sprints** (1-2 weeks).
* The **Product Backlog** (`docs/Tasks.md`) is our master list of work.
* A **Definition of Done** ensures quality for all completed work.
* All new work is done on separate branches and merged via Pull Requests.

## Agile Philosophy

To build the best possible game, we embrace an Agile development approach. This means we prioritize:
- **Individuals and interactions** over processes and tools.
- **Working software** over comprehensive documentation.
- **Customer collaboration** over contract negotiation.
- **Responding to change** over following a rigid plan.

We work in short, iterative cycles called **Sprints**, allowing us to get feedback early, adapt to new ideas, and consistently deliver a working, playable game.

## The Product Backlog

The `docs/Tasks.md` file serves as our **Product Backlog**. It is the single source of truth for all work to be done on the project.

- The backlog is a living document, constantly being refined and re-prioritized.
- Items at the top of the backlog are the highest priority and should be the most detailed.
- **Backlog Refinement:** To keep the backlog healthy, the team should regularly review upcoming tasks to ensure they are well-defined and ready for a future sprint.
- Anyone can add ideas to the backlog, but the **Product Owner** (the person responsible for the game's vision) has the final say on prioritization.

## Working in Sprints

Our development is organized into **Sprints**, which are short, time-boxed periods (e.g., 1-2 weeks) where we work to complete a small set of tasks from the backlog.

1.  **Sprint Planning:** At the beginning of a sprint, the team reviews the top items in the Product Backlog and selects a realistic amount of work to complete. This becomes the **Sprint Goal**. The selected tasks are moved to the "Current" phase in `docs/Tasks.md`.

2.  **Development:** During the sprint, the team works on the selected tasks, following the technical workflow below. Each task must meet our **Definition of Done**.

3.  **Sprint Review:** At the end of the sprint, the team demonstrates the completed, "Done" work. The goal is to have a new, potentially shippable version of the game.

4.  **Sprint Retrospective:** After the review, the team discusses what went well, what could be improved, and how to make the next sprint even better.

## Bug Handling

In the spirit of the Pragmatic Programmer's advice to "not tolerate broken windows," bugs are addressed with priority:
- **Critical Bugs:** A bug that prevents the game from being played or causes major data loss should be fixed immediately. This may require interrupting the current sprint's work.
- **Minor Bugs:** All other bugs should be added to the Product Backlog, prioritized, and scheduled for a future sprint.

## Definition of Done

A task or feature is considered **"Done"** only when it meets all of the following criteria:
- The code is complete and implements the required functionality.
- All automated checks (`cargo check`, `clippy`) pass without errors.
- New unit or integration tests have been written to cover the new functionality.
- All tests in the project (`cargo test`) are passing.
- Any relevant documentation (`docs/`, code comments) has been updated.
- The changes have been reviewed and approved by at least one other team member via a Pull Request.

## Technical Workflow

The following technical practices support our Agile workflow.

### Branching Strategy

All new work (features, bugfixes, etc.) must be done on a separate branch. This keeps the `master` branch stable and always in a potentially releasable state. Branch names should be descriptive and follow this convention:

- **Features:** `feature/<short-description>`
- **Bugfixes:** `bugfix/<short-description>`
- **Refactoring:** `refactor/<short-description>`
- **Documentation:** `docs/<short-description>`

### Development Process

1.  **Create a Branch:** Before starting a task, create a new branch from `master`.
    ```bash
    git checkout -b <branch-name>
    ```

2.  **Implement Changes:** Make your code changes, following the project's coding style.

3.  **Run Checks and Tests:** Before committing, ensure all checks and tests pass. This reflects the "Test Ruthlessly" philosophy; testing is not an afterthought but an integral part of ensuring quality and meeting our **Definition of Done**.
    ```bash
    cargo check
    cargo clippy
    cargo test
    ```

4.  **Commit Changes:** Commit your changes with a clear, descriptive message that follows the [Conventional Commits](https://www.conventionalcommits.org/) specification.
    ```bash
    git commit -m "feat: Add player jump" -m "Implements variable jump height based on button press duration."
    ```
> **Note:** When using `git commit -m`, it's best to use short, single-line messages. Long or multi-line messages can sometimes be misinterpreted by the shell, especially when using multiple `-m` flags. For automated processes like those in the Gemini CLI, it is crucial to avoid interactive prompts. Therefore, always provide the full commit message directly within the `git commit -m` command, ensuring it is properly quoted to prevent parsing issues.

### Automated Testing (Continuous Integration)

To guarantee that our `master` branch is always stable, we automate our testing process using **Continuous Integration (CI)** provided by GitHub Actions.

The `cargo run` command is used for quick, local playtesting and does not run the test suite. The `cargo test` command is used to run our full suite of unit, integration, and documentation tests.

Our CI workflow is configured to automatically run `cargo test` for every Pull Request. Here is how it works:
1.  A developer pushes a new feature branch and opens a Pull Request to merge it into `master`.
2.  GitHub Actions automatically detects the PR and begins the CI process.
3.  It checks out the code into a clean, virtual environment.
4.  It runs `cargo check`, `cargo clippy`, and, most importantly, `cargo test`.
5.  The results are reported back to the Pull Request page on GitHub.

A PR **cannot be merged** until all tests pass and it has a green checkmark from the CI system. This provides a critical safety net, ensuring that no new code breaks existing functionality.

### Pull Requests (PRs)

- **Push Your Branch:**
  Push your branch to the remote repository on GitHub.

  ```bash
  git push --set-upstream origin <branch-name>
  ```

- **Create a Pull Request:**
  You can create a pull request using the GitHub web interface or the GitHub CLI.

  - **Using the Web Interface:**
    Go to the project's GitHub page and create a new pull request from your branch
    to the `master` branch.

  - **Using the GitHub CLI:**
    You can use the `gh pr create` command to create a pull request from the command line.

    ```bash
    gh pr create --title "feat: Add new feature" --body "Detailed description of the new feature."
    ```

- **Code Review:**
  At least one other person should review your pull request before it is merged.
  The reviewer should check for correctness, style, and adherence to the
  project's standards.

### Merging

Once the pull request has been approved, it can be merged into the `master` branch.

- **Using the Web Interface:**
  Use the "Squash and merge" option on GitHub to keep the `master` branch history clean.
  After merging, the feature branch should be deleted.

- **Using the GitHub CLI:**
  You can use the `gh pr merge` command to merge a pull request from the command line.
  It is recommended to use the `--squash` option to keep the commit history clean.

  ```bash
  gh pr merge --squash
  ```
  After merging, you can delete the branch both locally and on the remote.
  ```bash
  git branch -d <branch-name>
  git push origin --delete <branch-name>
  ```

---
**Note To Self:** Remember that the 'main' code branch in Fedora Linux Git is called 'master'.