# Merging Feature Branches into Master

**TLDR:**
* This document outlines the process for merging multiple feature branches into the `master` branch while maintaining a clean and readable commit history.
* The process involves using a temporary integration branch to combine the feature branches before squashing them into a single commit on `master`.

## Prerequisites

*   Ensure you have the `gh` (GitHub CLI) tool installed and configured.
*   Ensure your local `master` branch is up-to-date with the remote `origin/master`.

## The Merge and Clean-up Workflow

This workflow is designed to safely consolidate multiple feature branches into the `master` branch.

### 1. Create an Integration Branch

Start by creating a temporary integration branch from the `master` branch. This provides a safe space to combine all the feature branches without affecting `master` directly.

```bash
# Switch to the master branch
git checkout master

# Ensure master is up-to-date
git pull origin master

# Create a new integration branch
git checkout -b integration-branch
```

### 2. Merge Feature Branches

Merge each feature branch into the `integration-branch` one by one. Using `--no-ff` creates a merge commit for each branch, which helps to keep track of where changes came from.

```bash
# Merge a feature branch, providing a commit message
git merge --no-ff -m "Merge branch '<branch-name>'" <branch-name>
```
*Repeat this command for each feature branch you want to merge.*

### 3. Merge into `master`

Once all feature branches are merged into the `integration-branch`, switch back to `master` and perform a "squash" merge. This flattens all the changes into a single commit on `master`.

```bash
# Switch back to the master branch
git checkout master

# Squash merge the integration branch
git merge --squash integration-branch

# Commit the squashed changes with a descriptive message
git commit -m "feat: A summary of all the merged features"
```

### 4. Push to GitHub

Push the updated `master` branch to the central repository.

```bash
git push origin master
```

### 5. Clean Up Branches

Finally, delete the temporary integration branch and all the now-merged feature branches from both your local machine and the central repository.

```bash
# Delete the local integration branch
git branch -D integration-branch

# Delete a local feature branch
git branch -d <branch-name>

# Delete a remote feature branch
git push origin --delete <branch-name>
```
*Repeat the delete commands for each branch you want to clean up.*
