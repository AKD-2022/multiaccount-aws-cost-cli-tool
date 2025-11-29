# AWS Cost CLI

A command-line tool to analyze AWS costs across multiple accounts. Track spending, view trends, and identify top cost drivers.

## Features

- 📊 Analyze costs across multiple AWS accounts simultaneously
- 📈 Track month-over-month cost trends
- 🔍 Break down costs by AWS service
- 📁 Export to CSV, JSON, or PNG charts
- 🏷️ Filter costs by tags
- 🔐 Authenticate using `~/.aws/credentials` (supports multiple profiles)

## First Time Setup (Step-by-Step)

Follow these steps if you're setting up the tool for the first time:

### Step 1: Install Rust

See the [Prerequisites](#prerequisites) section below for installation instructions.

### Step 2: Configure AWS Credentials

**Linux/Mac:**
```bash
# Create .aws directory
mkdir -p ~/.aws

# Edit credentials file
nano ~/.aws/credentials
```

**Windows:**
```powershell
# Create .aws directory
New-Item -ItemType Directory -Force -Path $env:USERPROFILE\.aws

# Edit credentials file
notepad $env:USERPROFILE\.aws\credentials
```

Add your AWS credentials (see [Authentication](#authentication) section for format).

### Step 3: Verify AWS Credentials

```bash
# Test your credentials work
aws sts get-caller-identity

# List your profiles
aws configure list-profiles
```

### Step 4: Build the Tool

```bash
# Navigate to project directory
cd aws-cost-cli

# Build the project
cargo build --release
```

### Step 5: Test the Tool

**Linux/Mac:**
```bash
./target/release/aws-cost-cli --help
```

**Windows:**
```powershell
target\release\aws-cost-cli.exe --help
```

### Step 6: Run Your First Query

**Linux/Mac:**
```bash
# Analyze all your AWS accounts
./target/release/aws-cost-cli
```

**Windows:**
```powershell
# Analyze all your AWS accounts
target\release\aws-cost-cli.exe
```

## Prerequisites

### Install Rust

**Windows:**
```powershell
# Option 1: Download rustup-init.exe from https://rustup.rs/
# Option 2: Using winget
winget install Rustlang.Rustup

# Verify installation
rustc --version
cargo --version
```

**Linux/Mac:**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Follow the prompts, then reload your shell
source ~/.cargo/env

# Verify installation
rustc --version
cargo --version
```

### Install AWS CLI (Optional but Recommended)

```bash
# Windows (using winget)
winget install Amazon.AWSCLI

# Linux
curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip"
unzip awscliv2.zip
sudo ./aws/install

# Mac
brew install awscli

# Verify installation
aws --version
```

## Installation

### Step 1: Clone or Navigate to Project

```bash
# If you have the source code
cd aws-cost-cli

# Verify you're in the right directory (should see Cargo.toml)
ls Cargo.toml  # Linux/Mac
dir Cargo.toml  # Windows
```

### Step 2: Build the Project

```bash
# Build in release mode (optimized, recommended)
cargo build --release
```

**Build Output Location:**
- **Linux/Mac:** `target/release/aws-cost-cli`
- **Windows:** `target\release\aws-cost-cli.exe`

### Step 3: Verify Build

**Linux/Mac:**
```bash
./target/release/aws-cost-cli --help
```

**Windows:**
```powershell
target\release\aws-cost-cli.exe --help
```

If you see the help message, the build was successful!

## Authentication

The tool authenticates using AWS credentials from `~/.aws/credentials` (same as AWS CLI). This makes it easy to work with multiple AWS accounts.

### Credentials File Location

The credentials file location depends on your operating system:

- **Linux/Mac:** `~/.aws/credentials` (e.g., `/home/username/.aws/credentials`)
- **Windows:** `%USERPROFILE%\.aws\credentials` (e.g., `C:\Users\YourName\.aws\credentials`)

### Setup Credentials

**Primary Method: `~/.aws/credentials` (Recommended)**

Create or edit the credentials file to add your AWS accounts:

**Linux/Mac:**
```bash
# Create the .aws directory if it doesn't exist
mkdir -p ~/.aws

# Edit the credentials file
nano ~/.aws/credentials
# or
vim ~/.aws/credentials
```

**Windows:**
```powershell
# Create the .aws directory if it doesn't exist
New-Item -ItemType Directory -Force -Path $env:USERPROFILE\.aws

# Edit the credentials file (opens in Notepad)
notepad $env:USERPROFILE\.aws\credentials
```

Then add your credentials:

```ini
[default]
aws_access_key_id = YOUR_ACCESS_KEY
aws_secret_access_key = YOUR_SECRET_KEY
region = us-east-1

[prod]
aws_access_key_id = PROD_ACCESS_KEY
aws_secret_access_key = PROD_SECRET_KEY
region = us-east-1

[dev]
aws_access_key_id = DEV_ACCESS_KEY
aws_secret_access_key = DEV_SECRET_KEY
region = us-east-1
```

**Alternative Methods:**
- Environment variables: `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`
- IAM roles (when running on EC2/ECS)
- SSO: `aws sso login` (credentials cached in `~/.aws/credentials`)

### Multiple AWS Accounts

The tool automatically discovers and processes all profiles in `~/.aws/credentials`. Each profile can represent a different AWS account:

- **Automatic discovery**: Run without `--profiles` to analyze all configured profiles
- **Select specific accounts**: Use `--profiles prod,dev` to target specific accounts
- **Account filtering**: Use `--account-id` to filter by account ID within profiles

### Required Permissions

Your AWS credentials need:
- `ce:GetCostAndUsage` - Read cost data
- `organizations:ListAccounts` - List accounts (optional, falls back to STS)
- `sts:GetCallerIdentity` - Identify current account

## Quick Start

### Method 1: Using `cargo run` (Development/Quick Testing)

**Important:** When using `cargo run`, you must use `--` to separate cargo arguments from program arguments.

**Linux/Mac:**
```bash
# Basic usage - analyzes all AWS profiles from ~/.aws/credentials
cargo run --release

# With date range
cargo run --release -- --start-date 2025-01-01 --end-date 2025-01-31

# Analyze specific accounts
cargo run --release -- --profiles prod,dev,staging

# Export to CSV
cargo run --release -- --csv report

# JSON output
cargo run --release -- --json
```

**Windows:**
```powershell
# Basic usage - analyzes all AWS profiles from ~/.aws/credentials
cargo run --release

# With date range
cargo run --release -- --start-date 2025-01-01 --end-date 2025-01-31

# Analyze specific accounts
cargo run --release -- --profiles prod,dev,staging

# Export to CSV
cargo run --release -- --csv report

# JSON output
cargo run --release -- --json
```

### Method 2: Using the Built Binary (Production/Recommended)

**Linux/Mac:**
```bash
# Basic usage - analyzes all AWS profiles from ~/.aws/credentials
./target/release/aws-cost-cli

# With date range
./target/release/aws-cost-cli --start-date 2025-01-01 --end-date 2025-01-31

# Analyze specific accounts
./target/release/aws-cost-cli --profiles prod,dev,staging

# Export to CSV
./target/release/aws-cost-cli --csv report

# JSON output
./target/release/aws-cost-cli --json
```

**Windows:**
```powershell
# Basic usage - analyzes all AWS profiles from ~/.aws/credentials
target\release\aws-cost-cli.exe

# With date range
target\release\aws-cost-cli.exe --start-date 2025-01-01 --end-date 2025-01-31

# Analyze specific accounts
target\release\aws-cost-cli.exe --profiles prod,dev,staging

# Export to CSV
target\release\aws-cost-cli.exe --csv report

# JSON output
target\release\aws-cost-cli.exe --json
```

### Method 3: Install Globally (Optional)

After building, you can install it globally:

```bash
# Install to cargo bin directory
cargo install --path .

# Now you can run from anywhere
aws-cost-cli --help
aws-cost-cli --start-date 2025-01-01 --end-date 2025-01-31
```

## Command Options

| Option | Description | Example |
|--------|-------------|---------|
| `--start-date` | Start date (YYYY-MM-DD) | `--start-date 2025-01-01` |
| `--end-date` | End date (YYYY-MM-DD) | `--end-date 2025-01-31` |
| `--profiles` | Comma-separated AWS profile names | `--profiles prod,dev` |
| `--account-id` | Filter by account ID(s) | `--account-id 123456789012` |
| `--granularity` | `hourly`, `daily`, or `monthly` | `--granularity daily` |
| `--csv` | Export to CSV (filename prefix) | `--csv report` |
| `--json` | Output as JSON | `--json` |
| `--chart` | Generate PNG charts | `--chart` |
| `--tag-key` | Filter by tag key | `--tag-key Environment` |
| `--tag-value` | Filter by tag value | `--tag-value Production` |
| `--profile-account-map` | JSON file mapping profiles to account IDs | `--profile-account-map accounts.json` |

## Examples

### Example 1: Basic Usage - Analyze All Accounts

**Linux/Mac:**
```bash
# Using cargo run
cargo run --release

# Using built binary
./target/release/aws-cost-cli
```

**Windows:**
```powershell
# Using cargo run
cargo run --release

# Using built binary
target\release\aws-cost-cli.exe
```

This will automatically discover and analyze all AWS profiles in `~/.aws/credentials` (or `%USERPROFILE%\.aws\credentials` on Windows).

### Example 2: Analyze Specific Date Range

**Linux/Mac:**
```bash
# Using cargo run
cargo run --release -- --start-date 2025-01-01 --end-date 2025-01-31

# Using built binary
./target/release/aws-cost-cli --start-date 2025-01-01 --end-date 2025-01-31
```

**Windows:**
```powershell
# Using cargo run
cargo run --release -- --start-date 2025-01-01 --end-date 2025-01-31

# Using built binary
target\release\aws-cost-cli.exe --start-date 2025-01-01 --end-date 2025-01-31
```

### Example 3: Analyze Multiple Specific Accounts

**Linux/Mac:**
```bash
# Analyze specific profiles
./target/release/aws-cost-cli --profiles prod,dev,staging

# With date range
./target/release/aws-cost-cli \
  --profiles prod,dev \
  --start-date 2025-01-01 \
  --end-date 2025-03-31
```

**Windows:**
```powershell
# Analyze specific profiles
target\release\aws-cost-cli.exe --profiles prod,dev,staging

# With date range (PowerShell uses backtick for line continuation)
target\release\aws-cost-cli.exe `
  --profiles prod,dev `
  --start-date 2025-01-01 `
  --end-date 2025-03-31
```

### Example 4: Export to CSV

**Linux/Mac:**
```bash
# Export all accounts to CSV
./target/release/aws-cost-cli --csv q1-report

# With date range
./target/release/aws-cost-cli \
  --start-date 2025-01-01 \
  --end-date 2025-03-31 \
  --csv q1-report
```

**Windows:**
```powershell
# Export all accounts to CSV
target\release\aws-cost-cli.exe --csv q1-report

# With date range
target\release\aws-cost-cli.exe `
  --start-date 2025-01-01 `
  --end-date 2025-03-31 `
  --csv q1-report
```

This creates multiple CSV files in the current directory.

### Example 5: JSON Output

**Linux/Mac:**
```bash
# Output JSON to console
./target/release/aws-cost-cli --json

# Save JSON to file
./target/release/aws-cost-cli --json > costs.json

# With date range
./target/release/aws-cost-cli \
  --start-date 2025-01-01 \
  --end-date 2025-01-31 \
  --json > january-costs.json
```

**Windows:**
```powershell
# Output JSON to console
target\release\aws-cost-cli.exe --json

# Save JSON to file
target\release\aws-cost-cli.exe --json > costs.json

# With date range
target\release\aws-cost-cli.exe `
  --start-date 2025-01-01 `
  --end-date 2025-01-31 `
  --json > january-costs.json
```

### Example 6: Generate Charts

**Linux/Mac:**
```bash
# Generate PNG charts for all accounts
./target/release/aws-cost-cli --chart

# With date range
./target/release/aws-cost-cli \
  --start-date 2025-01-01 \
  --end-date 2025-03-31 \
  --chart
```

**Windows:**
```powershell
# Generate PNG charts for all accounts
target\release\aws-cost-cli.exe --chart

# With date range
target\release\aws-cost-cli.exe `
  --start-date 2025-01-01 `
  --end-date 2025-03-31 `
  --chart
```

### Example 7: Filter by Account ID

**Linux/Mac:**
```bash
# Filter by single account ID
./target/release/aws-cost-cli --account-id 123456789012

# Filter by multiple account IDs
./target/release/aws-cost-cli --account-id 123456789012,987654321098
```

**Windows:**
```powershell
# Filter by single account ID
target\release\aws-cost-cli.exe --account-id 123456789012

# Filter by multiple account IDs
target\release\aws-cost-cli.exe --account-id 123456789012,987654321098
```

### Example 8: Filter by Tags

**Linux/Mac:**
```bash
# Filter by tag key only
./target/release/aws-cost-cli --tag-key Environment

# Filter by tag key and value
./target/release/aws-cost-cli \
  --tag-key Environment \
  --tag-value Production
```

**Windows:**
```powershell
# Filter by tag key only
target\release\aws-cost-cli.exe --tag-key Environment

# Filter by tag key and value
target\release\aws-cost-cli.exe `
  --tag-key Environment `
  --tag-value Production
```

### Example 9: Change Granularity

**Linux/Mac:**
```bash
# Daily granularity
./target/release/aws-cost-cli --granularity daily

# Monthly granularity (default)
./target/release/aws-cost-cli --granularity monthly

# Hourly granularity (limited to 7 days)
./target/release/aws-cost-cli \
  --granularity hourly \
  --start-date 2025-01-01 \
  --end-date 2025-01-07
```

**Windows:**
```powershell
# Daily granularity
target\release\aws-cost-cli.exe --granularity daily

# Monthly granularity (default)
target\release\aws-cost-cli.exe --granularity monthly

# Hourly granularity (limited to 7 days)
target\release\aws-cost-cli.exe `
  --granularity hourly `
  --start-date 2025-01-01 `
  --end-date 2025-01-07
```

### Example 10: Complete Workflow - Multiple Accounts with CSV Export

**Linux/Mac:**
```bash
./target/release/aws-cost-cli \
  --profiles prod,dev,staging \
  --start-date 2025-01-01 \
  --end-date 2025-03-31 \
  --csv q1-report \
  --granularity monthly
```

**Windows:**
```powershell
target\release\aws-cost-cli.exe `
  --profiles prod,dev,staging `
  --start-date 2025-01-01 `
  --end-date 2025-03-31 `
  --csv q1-report `
  --granularity monthly
```

## Output Formats

### Console Tables (Default)
- Unified view across all accounts
- Cost trends per account
- Service breakdown per account
- Global summary

### CSV Export
When using `--csv filename`, creates:
- `filename_trend_profile_X_account_Y.csv` - Cost trends
- `filename_service_summary_profile_X_account_Y.csv` - Service costs
- `filename_global_summary.csv` - Totals
- `filename_unified_view.csv` - All accounts combined

### JSON
Machine-readable output with account data, unified view, and global summary.

### Charts
PNG images showing cost trends: `cost_trend_profile_X_account_Y.png`

## Account Discovery

The tool discovers accounts in this order:
1. **Profile mapping file** (if `--profile-account-map` provided)
2. **AWS Organizations** (if account is in an organization)
3. **Current account** (via STS `GetCallerIdentity`)

If Organizations access fails, it automatically falls back to the current account.

## Troubleshooting

### No AWS profiles found

**Problem:** The tool can't find any AWS profiles in your credentials file.

**Solution:**

**Linux/Mac:**
```bash
# Option 1: Set up credentials using AWS CLI
aws configure

# Option 2: Manually create ~/.aws/credentials file
mkdir -p ~/.aws
nano ~/.aws/credentials

# Option 3: Manually specify profile when running
./target/release/aws-cost-cli --profiles your-profile-name
```

**Windows:**
```powershell
# Option 1: Set up credentials using AWS CLI
aws configure

# Option 2: Manually create credentials file
New-Item -ItemType Directory -Force -Path $env:USERPROFILE\.aws
notepad $env:USERPROFILE\.aws\credentials

# Option 3: Manually specify profile when running
target\release\aws-cost-cli.exe --profiles your-profile-name
```

### Authentication error

**Problem:** AWS authentication is failing.

**Solution:**

**Linux/Mac:**
```bash
# Verify your credentials work with AWS CLI
aws sts get-caller-identity

# Test specific profile
aws sts get-caller-identity --profile prod

# List all available profiles
aws configure list-profiles

# For SSO users
aws sso login
```

**Windows:**
```powershell
# Verify your credentials work with AWS CLI
aws sts get-caller-identity

# Test specific profile
aws sts get-caller-identity --profile prod

# List all available profiles
aws configure list-profiles

# For SSO users
aws sso login
```

### Verify credentials file exists

**Linux/Mac:**
```bash
# Check if credentials file exists
ls -la ~/.aws/credentials

# View credentials file (be careful - contains secrets!)
cat ~/.aws/credentials
```

**Windows:**
```powershell
# Check if credentials file exists
Test-Path $env:USERPROFILE\.aws\credentials

# View credentials file location
$env:USERPROFILE\.aws\credentials
```

### No cost data retrieved
- Ensure Cost Explorer is enabled in your AWS account
- Check date range (Cost Explorer has data retention limits)
- Verify accounts have costs in the specified period

### Hourly granularity limited to 7 days
Hourly data only works for date ranges up to 7 days. Use `daily` or `monthly` for longer periods.

## Profile-Account Mapping

For explicit profile-to-account mapping, create a JSON file:

**accounts.json:**
```json
{
  "prod-profile": "123456789012",
  "dev-profile": "123456789013"
}
```

```bash
./target/release/aws-cost-cli --profile-account-map accounts.json
```

## Requirements

- Rust 1.70 or newer
- AWS account with Cost Explorer enabled
- AWS credentials in `~/.aws/credentials` or environment variables

## Verification Steps

After installation, verify everything works:

**Step 1: Check AWS credentials**
```bash
# Linux/Mac/Windows
aws sts get-caller-identity
```

**Step 2: List available profiles**
```bash
# Linux/Mac/Windows
aws configure list-profiles
```

**Step 3: Test the tool**
```bash
# Linux/Mac
./target/release/aws-cost-cli --help

# Windows
target\release\aws-cost-cli.exe --help
```

**Step 4: Run a test query**
```bash
# Linux/Mac - analyze last month
./target/release/aws-cost-cli --start-date 2025-01-01 --end-date 2025-01-31

# Windows - analyze last month
target\release\aws-cost-cli.exe --start-date 2025-01-01 --end-date 2025-01-31
```

## Development

### Running During Development

**Linux/Mac:**
```bash
# Development mode (faster compilation, slower execution)
cargo run

# Release mode (slower compilation, faster execution - recommended)
cargo run --release

# Important: Use -- to separate cargo args from program args
cargo run --release -- --start-date 2025-01-01 --end-date 2025-01-31
```

**Windows:**
```powershell
# Development mode
cargo run

# Release mode (recommended)
cargo run --release

# Important: Use -- to separate cargo args from program args
cargo run --release -- --start-date 2025-01-01 --end-date 2025-01-31
```

### Install Globally

**Linux/Mac/Windows:**
```bash
# Install to cargo bin directory (available system-wide)
cargo install --path .

# Now you can run from anywhere
aws-cost-cli --help
aws-cost-cli --start-date 2025-01-01 --end-date 2025-01-31
```

### Other Useful Commands

```bash
# Clean build artifacts
cargo clean

# Run tests
cargo test

# Check for errors without building
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy
```

---

**Note:** AWS Cost Explorer API calls are free, but ensure your credentials have the required permissions.
