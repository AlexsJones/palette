# Palette

A powerful command-line tool for managing multiple GitHub repositories with ease. Palette simplifies the process of working with numerous repositories by providing a unified interface for common Git operations across all your projects.

## Features

- **Bulk Operations**: Execute Git commands across multiple repositories simultaneously
- **Repository Discovery**: Automatically find and manage all your GitHub repositories
- **Custom Filtering**: Target specific repositories using patterns or filters
- **Parallel Processing**: Speed up operations with parallel execution
- **Configuration**: Customize behavior through a simple configuration file

## Installation

### Prerequisites
- Rust (latest stable version)
- Git

### From Source

```bash
git clone https://github.com/yourusername/palette.git
cd palette
cargo install --path .
```

## Usage

```bash
# List all repositories
palette list

# Pull latest changes for all repositories
palette pull

# Execute a custom git command across repositories
palette exec "git status"

# Clone all repositories from a GitHub organization
palette clone org/your-org

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

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
