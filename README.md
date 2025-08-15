<img src="logo.png" width="200">

A powerful command-line tool for managing multiple GitHub repositories with ease. 

Palette simplifies the process of working with numerous repositories by providing a unified interface for common Git operations across all your projects.

## Features

- **Bulk Operations**: Execute Git commands across multiple repositories simultaneously
- **Repository Discovery**: Automatically find and manage all your GitHub repositories
- **Custom Filtering**: Target specific repositories using patterns or filters
- **Parallel Processing**: Speed up operations with parallel execution
- **Configuration**: Customize behavior through a simple configuration file
- **Interactive Confirmations**: Safe push operations with user confirmation prompts
- **Automatic Repository Cloning**: Missing repositories are automatically cloned during pull operations

## Installation

### Prerequisites
- Rust (latest stable version)
- Git
- A GitHub account with repositories to manage

### Brew
```
brew tap AlexsJones/palette
brew install palette
```

### From Source

```bash
git clone https://github.com/AlexsJones/palette.git
cd palette
cargo install --path .
```

## Usage

### Basic Commands

```bash
# List all tracked repositories with their status
# Shows branch, commit hash, ahead status, and checkout status
palette list

# Pull latest changes for all repositories
# Automatically clones missing repositories
palette pull

# Pull changes for a specific repository
palette pull --name repository-name

# Push changes for repositories that are ahead of remote
# Shows confirmation prompt before pushing
palette push

# Switch all repositories to a specific branch
palette switch --branch-name feature-branch

# Create and switch to a new branch across all repositories
palette switch --branch-name new-feature --create true

# Execute a command in all tracked repositories
palette exec --run-command "git status"
```

### Repository Management

```bash
# Add a new repository to track and clone it
palette add --organization AlexsJones --name my-repo

# Remove a repository from tracking
palette remove --name repository-name
```

## Configuration

Palette uses a JSON configuration file named `config.palette` that tracks your repositories. Here's an example configuration:

```json
{
  "configuration_path": ".",
  "configuration_file_name": "config.palette", 
  "configuration_full_path": "./config.palette",
  "repository": [
    {
      "name": "repository-name",
      "organization": "org-name",
      "cloned_locally": true,
      "checkout_info": {
        "branch_name": "refs/heads/main",
        "commit_sha": "commit-hash"
      }
    }
  ]
}
```

### Configuration Fields

- `configuration_path`: The directory where the configuration file is stored
- `configuration_file_name`: The name of the configuration file
- `configuration_full_path`: The full path to the configuration file
- `repository`: Array of tracked repositories

### Repository Fields

- `name`: Repository name
- `organization`: GitHub organization or username
- `cloned_locally`: Whether the repository is cloned on your local machine
- `checkout_info`: Information about the current checkout state
  - `branch_name`: Current branch (with refs/heads/ prefix)
  - `commit_sha`: Current commit SHA

## Command Details

### List Command

The `palette list` command provides a comprehensive overview of all tracked repositories:

```bash
palette list
```

Output format:
```
repo-name branch:refs/heads/main, commit:abcd1234, ahead of remote: yes/no, checked out: yes/no
```

- Shows current branch name
- Displays first 8 characters of commit SHA
- Indicates if local branch is ahead of remote (colored output)
- Shows if repository is cloned locally (colored output)

### Push Command

The `palette push` command safely pushes changes with interactive confirmation:

```bash
palette push
```

- Identifies repositories that are ahead of their remote
- Shows a list of repositories to be pushed
- Prompts for confirmation before proceeding
- Pushes repositories in parallel for efficiency

### Pull Command

The `palette pull` command intelligently handles repository updates:

```bash
# Pull all repositories
palette pull

# Pull specific repository
palette pull --name my-repo
```

- Automatically clones missing repositories during bulk pull operations
- Updates configuration with latest checkout information
- Handles individual repository pulls when specified

### Switch Command

The `palette switch` command manages branch operations across repositories:

```bash
# Switch to existing branch
palette switch --branch-name feature-branch

# Create and switch to new branch
palette switch --branch-name new-feature --create true
```

- Switches all tracked repositories to the specified branch
- Optionally creates new branches when `--create true` is used
- Updates configuration with new checkout information

### Exec Command

The `palette exec` command allows you to execute arbitrary commands across all tracked repositories:

```bash
# Execute git status in all repositories
palette exec --run-command "git status"

# Check for uncommitted changes
palette exec --run-command "git diff --stat"

# Run tests in all repositories
palette exec --run-command "npm test"
```

- Executes the specified command in each tracked repository's directory
- Displays output from all repositories sequentially
- Useful for running checks, builds, or any command across your entire repository collection

### Managing Repositories

You can manage repositories using the following commands:

```bash
# Add a new repository to track
palette add --organization k8sgpt-ai --name k8sgpt

# Remove a repository from tracking
palette remove --name repo

# List all tracked repositories
palette list
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Status

This project is actively being developed. Current version is 0.1.0 and uses Rust edition 2024.

### Known Limitations

- The `remove` command is currently under development
- Push operations require manual confirmation for safety

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
