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

### Brew
```
brew tap AlexsJones/palette
brew install palette
```

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

## Managing Multiple Repositories in One Folder

One of palette's key strengths is organizing and managing multiple GitHub repositories within a single root directory. This approach allows you to work with dozens or hundreds of repositories efficiently.

### Example Project Structure

Here's how your workspace might look when using palette to manage multiple repositories:

```
~/workspace/
├── config.palette              # Palette configuration file
├── frontend-app/               # Repository 1
│   ├── .git/
│   ├── package.json
│   └── src/
├── backend-api/                # Repository 2  
│   ├── .git/
│   ├── Cargo.toml
│   └── src/
├── mobile-app/                 # Repository 3
│   ├── .git/
│   ├── android/
│   └── ios/
├── documentation/              # Repository 4
│   ├── .git/
│   ├── docs/
│   └── README.md
├── infrastructure/             # Repository 5
│   ├── .git/
│   ├── terraform/
│   └── kubernetes/
└── shared-libraries/           # Repository 6
    ├── .git/
    ├── lib1/
    └── lib2/
```

### Adding Repositories to Your Workspace

```bash
# Navigate to your workspace root
cd ~/workspace

# Add repositories one by one
palette add --organization myorg --name frontend-app
palette add --organization myorg --name backend-api  
palette add --organization myorg --name mobile-app
palette add --organization myorg --name documentation
palette add --organization myorg --name infrastructure
palette add --organization myorg --name shared-libraries

# Or add from different organizations
palette add --organization external-org --name useful-library
palette add --organization community --name open-source-tool
```

### Bulk Operations Across All Repositories

Once you have multiple repositories set up, palette makes it easy to perform operations across all of them:

```bash
# Check status of all repositories
palette list
# Output:
# frontend-app branch:main, commit:a1b2c3d4, ahead of remote: no, checked out: yes
# backend-api branch:develop, commit:e5f6g7h8, ahead of remote: yes, checked out: yes  
# mobile-app branch:main, commit:i9j0k1l2, ahead of remote: no, checked out: yes
# ...

# Pull latest changes for all repositories
palette pull

# Pull changes for a specific repository
palette pull --name frontend-app

# Switch all repositories to a new branch
palette switch --branch-name feature/new-feature --create

# Push all repositories that are ahead of remote
palette push

# Execute custom commands across all repositories
palette exec "npm install"           # Install dependencies in all Node.js projects
palette exec "cargo check"           # Check all Rust projects
palette exec "git log --oneline -5"  # Show last 5 commits in each repo
palette exec "find . -name '*.md' | wc -l"  # Count markdown files
```

### Real-World Scenarios

**Microservices Development:**
```bash
# You have 15 microservices, each in its own repository
palette add --organization mycompany --name user-service
palette add --organization mycompany --name payment-service
palette add --organization mycompany --name notification-service
# ... add all 15 services

# Update all services to latest
palette pull

# Create a new feature branch across all services  
palette switch --branch-name feature/add-logging --create

# Run tests across all services
palette exec "npm test"

# Push changes to all services
palette push
```

**Open Source Contribution:**
```bash
# Track multiple projects you contribute to
palette add --organization kubernetes --name kubernetes
palette add --organization docker --name docker
palette add --organization rust-lang --name rust
palette add --organization nodejs --name node

# Stay up to date with all projects
palette pull

# Check what needs attention
palette list
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
