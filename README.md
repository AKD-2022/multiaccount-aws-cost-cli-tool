# AWS Cost CLI

A simple command-line tool to analyze AWS costs across multiple accounts. See how much you're spending, track trends, and find which services cost the most.

## What It Does

- 📊 Shows costs for multiple AWS accounts at once
- 📈 Tracks cost trends month-over-month
- 🔍 Breaks down costs by AWS service
- 📁 Exports data to CSV, JSON, or charts
- 🏷️ Filters costs by tags

## Quick Start

### Install

```bash
# Build the project
cargo build --release

# The tool will be at: target/release/aws-cost-cli
```

### Run

**Option 1: Using `cargo run` (Development/Quick Testing)**
```bash
# Basic usage - analyze all your AWS profiles
cargo run --release

# Specify date range
cargo run --release -- --start-date 2025-01-01 --end-date 2025-01-31

# Export to CSV
cargo run --release -- --csv my-report

# Get JSON output
cargo run --release -- --json

# Note: Use -- after cargo run to pass arguments to your program
```

**Option 2: Using the built binary (Production)**
```bash
# Basic usage - analyze all your AWS profiles
./target/release/aws-cost-cli

# Specify date range
./target/release/aws-cost-cli --start-date 2025-01-01 --end-date 2025-01-31

# Export to CSV
./target/release/aws-cost-cli --csv my-report

# Get JSON output
./target/release/aws-cost-cli --json
```

**Windows Users:**
```bash
# Using cargo run
cargo run --release -- --start-date 2025-01-01 --end-date 2025-01-31

# Using the built binary
target\release\aws-cost-cli.exe --start-date 2025-01-01 --end-date 2025-01-31
```

## Commands Reference

### Setup Commands

#### Install Rust (if not already installed)
```bash
# Windows - Download and run rustup-init.exe from https://rustup.rs/
# Or use winget:
winget install Rustlang.Rustup

# Linux/Mac:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Verify Rust Installation
```bash
rustc --version
cargo --version
```

#### Configure AWS Credentials
```bash
# Option 1: Use AWS CLI to configure
aws configure

# Option 2: Set environment variables
export AWS_ACCESS_KEY_ID=your_key
export AWS_SECRET_ACCESS_KEY=your_secret
export AWS_REGION=us-east-1

# Option 3: For SSO
aws sso login
```

### Build Commands

#### Build the Project
```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Clean build artifacts
cargo clean

# Run tests
cargo test
```

#### Install as Global Command (Optional)
```bash
# Install to cargo bin directory
cargo install --path .

# Then run from anywhere:
aws-cost-cli --help
```

### Run Commands (Using `cargo run`)

**Important:** When using `cargo run`, you need to use `--` to separate cargo arguments from your program arguments.

#### Development Mode (Faster compilation, slower execution)
```bash
# Basic usage
cargo run

# With arguments - note the -- separator
cargo run -- --start-date 2025-01-01 --end-date 2025-01-31

# Multiple arguments
cargo run -- --profiles prod,dev --csv report
```

#### Release Mode (Slower compilation, faster execution - Recommended)
```bash
# Basic usage
cargo run --release

# With date range
cargo run --release -- --start-date 2025-01-01 --end-date 2025-01-31

# Export to CSV
cargo run --release -- --csv my-report

# JSON output
cargo run --release -- --json

# Multiple profiles
cargo run --release -- --profiles prod,dev,staging

# Full example with all options
cargo run --release -- \
  --start-date 2025-01-01 \
  --end-date 2025-03-31 \
  --profiles prod,dev \
  --csv q1-report \
  --granularity monthly
```

**Why use `cargo run`?**
- ✅ No need to build separately - cargo builds and runs in one command
- ✅ Great for development and quick testing
- ✅ Automatically rebuilds if code changes
- ⚠️ Slightly slower startup time compared to running the binary directly

**When to use the built binary instead:**
- ✅ Production deployments
- ✅ Faster execution (no cargo overhead)
- ✅ Can be copied to other machines
- ✅ Better for scripts and automation

### Basic Usage Commands

#### Run with Default Settings
```bash
# Using cargo run (development)
cargo run --release

# Using the built binary
./target/release/aws-cost-cli
# Windows: target\release\aws-cost-cli.exe
```

#### Specify Date Range
```bash
# Using cargo run
cargo run --release -- --start-date 2025-01-01 --end-date 2025-01-31

# Using the built binary
./target/release/aws-cost-cli --start-date 2025-01-01 --end-date 2025-01-31
# Windows: target\release\aws-cost-cli.exe --start-date 2025-01-01 --end-date 2025-01-31
```

#### Select Specific AWS Profiles
```bash
# Using cargo run
cargo run --release -- --profiles prod
cargo run --release -- --profiles prod,dev,staging

# Using the built binary
./target/release/aws-cost-cli --profiles prod
./target/release/aws-cost-cli --profiles prod,dev,staging
```

#### Filter by Account ID
```bash
# Using cargo run
cargo run --release -- --account-id 123456789012
cargo run --release -- --account-id 123456789012,987654321098

# Using the built binary
./target/release/aws-cost-cli --account-id 123456789012
./target/release/aws-cost-cli --account-id 123456789012,987654321098
```

### Output Format Commands

#### Export to CSV
```bash
# Using cargo run
cargo run --release -- --csv report
cargo run --release -- --start-date 2025-01-01 --end-date 2025-03-31 --csv q1-report

# Using the built binary
./target/release/aws-cost-cli --csv report
./target/release/aws-cost-cli --start-date 2025-01-01 --end-date 2025-03-31 --csv q1-report
```

#### JSON Output
```bash
# Using cargo run
cargo run --release -- --json
cargo run --release -- --json > costs.json
cargo run --release -- --start-date 2025-01-01 --end-date 2025-01-31 --json

# Using the built binary
./target/release/aws-cost-cli --json
./target/release/aws-cost-cli --json > costs.json
./target/release/aws-cost-cli --start-date 2025-01-01 --end-date 2025-01-31 --json
```

#### Generate Charts
```bash
# Using cargo run
cargo run --release -- --chart
cargo run --release -- --start-date 2025-01-01 --end-date 2025-03-31 --chart

# Using the built binary
./target/release/aws-cost-cli --chart
./target/release/aws-cost-cli --start-date 2025-01-01 --end-date 2025-03-31 --chart
```

### Advanced Commands

#### Change Granularity
```bash
# Using cargo run
cargo run --release -- --granularity daily
cargo run --release -- --granularity monthly
cargo run --release -- --granularity hourly --start-date 2025-01-01 --end-date 2025-01-07

# Using the built binary
./target/release/aws-cost-cli --granularity daily
./target/release/aws-cost-cli --granularity monthly
./target/release/aws-cost-cli --granularity hourly --start-date 2025-01-01 --end-date 2025-01-07
```

#### Filter by Tags
```bash
# Using cargo run
cargo run --release -- --tag-key Environment
cargo run --release -- --tag-key Environment --tag-value Production

# Using the built binary
./target/release/aws-cost-cli --tag-key Environment
./target/release/aws-cost-cli --tag-key Environment --tag-value Production
```

#### Use Profile-Account Mapping
```bash
# Using cargo run
cargo run --release -- --profile-account-map accounts.json

# Using the built binary
./target/release/aws-cost-cli --profile-account-map accounts.json
```

### Combined Commands

#### Full Example: Multiple Profiles with CSV Export
```bash
# Using cargo run
cargo run --release -- \
  --profiles prod,dev,staging \
  --start-date 2025-01-01 \
  --end-date 2025-03-31 \
  --csv q1-report \
  --granularity monthly

# Using the built binary
./target/release/aws-cost-cli \
  --profiles prod,dev,staging \
  --start-date 2025-01-01 \
  --end-date 2025-03-31 \
  --csv q1-report \
  --granularity monthly
```

#### Full Example: JSON Output with Filters
```bash
# Using cargo run
cargo run --release -- \
  --start-date 2025-01-01 \
  --end-date 2025-01-31 \
  --account-id 123456789012 \
  --tag-key Environment \
  --tag-value Production \
  --json > prod-costs.json

# Using the built binary
./target/release/aws-cost-cli \
  --start-date 2025-01-01 \
  --end-date 2025-01-31 \
  --account-id 123456789012 \
  --tag-key Environment \
  --tag-value Production \
  --json > prod-costs.json
```

#### Full Example: Charts with Specific Accounts
```bash
# Using cargo run
cargo run --release -- \
  --profiles prod,dev \
  --start-date 2025-01-01 \
  --end-date 2025-03-31 \
  --granularity daily \
  --chart

# Using the built binary
./target/release/aws-cost-cli \
  --profiles prod,dev \
  --start-date 2025-01-01 \
  --end-date 2025-03-31 \
  --granularity daily \
  --chart
```

### Help Commands

#### Show Help
```bash
# Using cargo run
cargo run --release -- --help
# Or simply:
cargo run -- --help

# Using the built binary
./target/release/aws-cost-cli --help

# If installed globally
aws-cost-cli --help
```

### Verification Commands

#### Test AWS Credentials
```bash
# Verify AWS CLI is configured
aws sts get-caller-identity

# List available profiles
aws configure list-profiles

# Test specific profile
aws sts get-caller-identity --profile prod
```

## Authentication

The tool uses your existing AWS credentials. It works the same way as the AWS CLI.

### How Authentication Works

```
1. You run the tool
2. Tool looks for AWS credentials in this order:
   - AWS profiles in ~/.aws/credentials
   - Environment variables (AWS_ACCESS_KEY_ID, etc.)
   - IAM roles (if running on EC2/ECS)
3. Tool connects to AWS APIs
4. Tool fetches your cost data
```

### Setup AWS Credentials

**Option 1: Use AWS Profiles (Recommended)**

Create `~/.aws/credentials`:
```ini
[default]
aws_access_key_id = YOUR_KEY
aws_secret_access_key = YOUR_SECRET

[prod]
aws_access_key_id = PROD_KEY
aws_secret_access_key = PROD_SECRET
```

**Option 2: Use Environment Variables**

```bash
export AWS_ACCESS_KEY_ID=your_key
export AWS_SECRET_ACCESS_KEY=your_secret
export AWS_REGION=us-east-1
```

### Required Permissions

Your AWS user/role needs these permissions:
- `ce:GetCostAndUsage` - To read cost data
- `organizations:ListAccounts` - To list accounts (optional)
- `sts:GetCallerIdentity` - To identify current account

## How It Works

```
┌─────────────┐
│   You Run   │
│   The Tool  │
└──────┬──────┘
       │
       ▼
┌─────────────────┐
│  Tool Reads     │
│  AWS Profiles   │
└──────┬──────────┘
       │
       ▼
┌─────────────────┐
│  Tool Connects  │
│  to AWS APIs    │
└──────┬──────────┘
       │
       ▼
┌─────────────────┐
│  Fetches Cost   │
│  Data           │
└──────┬──────────┘
       │
       ▼
┌─────────────────┐
│  Shows Results  │
│  (Tables/CSV/   │
│   JSON/Charts)  │
└─────────────────┘
```

## Command Options

| Option | What It Does | Example |
|--------|--------------|---------|
| `--start-date` | Start date (YYYY-MM-DD) | `--start-date 2025-01-01` |
| `--end-date` | End date (YYYY-MM-DD) | `--end-date 2025-01-31` |
| `--profiles` | Which AWS profiles to use | `--profiles prod,dev` |
| `--account-id` | Filter specific accounts | `--account-id 123456789012` |
| `--granularity` | hourly, daily, or monthly | `--granularity daily` |
| `--csv` | Export to CSV files | `--csv report` |
| `--json` | Output as JSON | `--json` |
| `--chart` | Generate PNG charts | `--chart` |
| `--tag-key` | Filter by tag | `--tag-key Environment` |
| `--tag-value` | Tag value to filter | `--tag-value Production` |

## Examples

### Example 1: Basic Cost Check

```bash
# Using cargo run
cargo run --release

# Using the built binary
./target/release/aws-cost-cli
```

Shows costs for all your AWS profiles in a table.

### Example 2: Specific Date Range

```bash
# Using cargo run
cargo run --release -- \
  --start-date 2025-01-01 \
  --end-date 2025-01-31

# Using the built binary
./target/release/aws-cost-cli \
  --start-date 2025-01-01 \
  --end-date 2025-01-31
```

### Example 3: Export to CSV

```bash
# Using cargo run
cargo run --release -- \
  --start-date 2025-01-01 \
  --end-date 2025-03-31 \
  --csv q1-report

# Using the built binary
./target/release/aws-cost-cli \
  --start-date 2025-01-01 \
  --end-date 2025-03-31 \
  --csv q1-report
```

Creates CSV files you can open in Excel.

### Example 4: Multiple Profiles

```bash
# Using cargo run
cargo run --release -- \
  --profiles prod,dev,staging \
  --start-date 2025-01-01 \
  --end-date 2025-01-31

# Using the built binary
./target/release/aws-cost-cli \
  --profiles prod,dev,staging \
  --start-date 2025-01-01 \
  --end-date 2025-01-31
```

### Example 5: JSON Output

```bash
# Using cargo run
cargo run --release -- --json > costs.json

# Using the built binary
./target/release/aws-cost-cli --json > costs.json
```

Good for scripts and automation.

### Example 6: Generate Charts

```bash
# Using cargo run
cargo run --release -- --chart

# Using the built binary
./target/release/aws-cost-cli --chart
```

Creates PNG image files showing cost trends.

## Output Formats

### 1. Console Tables (Default)

Shows formatted tables in your terminal:
- Unified view of all accounts
- Cost trends per account
- Service breakdown per account
- Total summary

### 2. CSV Files

Use `--csv filename` to export:
- `filename_trend_profile_X_account_Y.csv` - Cost trends
- `filename_service_summary_profile_X_account_Y.csv` - Service costs
- `filename_global_summary.csv` - Totals
- `filename_unified_view.csv` - All accounts together

### 3. JSON

Use `--json` for machine-readable output:
```json
{
  "accounts": [...],
  "unified_view": [...],
  "global_summary": {
    "total_cost": 1234.56,
    "average_monthly_cost": 1234.56
  }
}
```

### 4. Charts

Use `--chart` to generate PNG images:
- `cost_trend_profile_X_account_Y.png`

## Account Discovery

The tool finds accounts in this order:

1. **Profile Mapping File** (if you provide `--profile-account-map`)
2. **AWS Organizations** (if your account is in an organization)
3. **Current Account** (uses STS to get the account you're logged into)

If Organizations doesn't work, it automatically falls back to using your current account.

## Troubleshooting

### "No AWS profiles found"

**Fix:** Set up AWS credentials:
```bash
aws configure
```

Or specify profiles manually:
```bash
./target/release/aws-cost-cli --profiles your-profile-name
```

### "Authentication error"

**Fix:** Check your credentials work:
```bash
aws sts get-caller-identity
```

If using SSO:
```bash
aws sso login
```

### "No cost data retrieved"

**Fix:**
- Make sure Cost Explorer is enabled in your AWS account
- Check your date range (Cost Explorer has limits)
- Verify the account actually has costs in that period

### "Hourly granularity limited to 7 days"

**Fix:** Hourly data only works for up to 7 days. Use daily or monthly for longer periods.

## Profile-Account Mapping

If you want to map specific profiles to account IDs, create a JSON file:

**accounts.json:**
```json
{
  "prod-profile": "123456789012",
  "dev-profile": "123456789013"
}
```

Then use it:
```bash
./target/release/aws-cost-cli --profile-account-map accounts.json
```

## Requirements

- Rust 1.70 or newer
- AWS account with Cost Explorer enabled
- AWS credentials configured

## License

[Add your license here]

---

**Note:** This tool uses AWS APIs which are free, but make sure you have the right permissions set up.
