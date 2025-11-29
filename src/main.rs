
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_costexplorer::{Client as CostExplorerClient, types::{DateInterval, Granularity, GroupDefinition, GroupDefinitionType, Dimension}};
use aws_sdk_organizations::Client as OrganizationsClient;
use aws_sdk_sts::Client as StsClient;
use clap::{Parser, ValueEnum};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use tokio;
use prettytable::{Table, Row, Cell, format};
use chrono::{NaiveDate, Duration};
use csv::Writer;
use serde::{Serialize, Deserialize};
use plotters::prelude::*;
use std::cmp::min;

#[derive(Parser, Debug)]
#[command(author, version, about = "CLI tool to fetch AWS cost trend analysis and service consumption for multiple profiles", long_about = None)]
struct Cli {
    #[arg(long, default_value = "2025-01-01")]
    start_date: String,
    #[arg(long, default_value = "2025-07-04")]
    end_date: String,
    #[arg(long, value_enum, default_value_t = GranularityOption::Monthly)]
    granularity: GranularityOption,
    #[arg(long)]
    csv: Option<String>,
    #[arg(long, value_delimiter = ',', help = "Comma-separated list of account IDs to filter (e.g., 1234567890,546796989090)")]
    account_id: Option<Vec<String>>,
    #[arg(long, value_delimiter = ',', help = "Comma-separated list of AWS profile names (e.g., prod-profile,dev-profile)")]
    profiles: Option<Vec<String>>,
    #[arg(long, help = "Path to JSON file mapping profiles to account IDs (e.g., {\"prod-profile\": \"123456789012\"})")]
    profile_account_map: Option<String>,
    #[arg(long)]
    tag_key: Option<String>,
    #[arg(long)]
    tag_value: Option<String>,
    #[arg(long, default_value_t = false)]
    json: bool,
    #[arg(long, default_value_t = false)]
    chart: bool,
}

#[derive(ValueEnum, Clone, Debug, PartialEq)]
enum GranularityOption {
    Daily,
    Monthly,
    Hourly,
}

impl From<GranularityOption> for Granularity {
    fn from(opt: GranularityOption) -> Self {
        match opt {
            GranularityOption::Daily => Granularity::Daily,
            GranularityOption::Monthly => Granularity::Monthly,
            GranularityOption::Hourly => Granularity::Hourly,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct CostTrendData {
    month: String,
    total_cost: f64,
    mom_change_percent: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct ServiceConsumptionData {
    service: String,
    monthly_costs: HashMap<String, f64>,
    total_cost: f64,
    percent_of_total: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct AccountCostData {
    profile: String,
    account_id: String,
    account_name: String,
    cost_trend: Vec<CostTrendData>,
    service_consumption: Vec<ServiceConsumptionData>,
    total_cost: f64,
    average_monthly_cost: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct UnifiedViewData {
    profile: String,
    account_id: String,
    account_name: String,
    monthly_costs: HashMap<String, f64>,
}

fn get_aws_profile_names() -> Vec<String> {
    use std::collections::HashSet;
    use std::fs;
    use std::path::PathBuf;

    let mut profiles = HashSet::new();
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    for file in &[".aws/credentials", ".aws/config"] {
        let path = PathBuf::from(&home).join(file);
        if let Ok(content) = fs::read_to_string(path) {
            for line in content.lines() {
                if let Some(profile) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
                    let profile = profile.trim().trim_start_matches("profile ").to_string();
                    profiles.insert(profile);
                }
            }
        }
    }
    profiles.into_iter().collect()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let start_date = NaiveDate::parse_from_str(&cli.start_date, "%Y-%m-%d")
        .map_err(|e| format!("Invalid start date: {}", e))?;
    let end_date = NaiveDate::parse_from_str(&cli.end_date, "%Y-%m-%d")
        .map_err(|e| format!("Invalid end date: {}", e))?;

    let six_months_ago = end_date - Duration::days(180);
    if start_date < six_months_ago {
        eprintln!("Warning: Start date is before {}. Trend analysis will include data from {} onwards.", 
            six_months_ago.format("%Y-%m-%d"), six_months_ago.format("%Y-%m-%d"));
    }

    if cli.granularity != GranularityOption::Monthly {
        eprintln!("Warning: Cost trend analysis is best with --granularity monthly. Using {} instead.", 
            cli.granularity.to_possible_value().unwrap().get_name());
    }

    // Validate hourly granularity date range
    if cli.granularity == GranularityOption::Hourly {
        let days = (end_date - start_date).num_days();
        if days > 7 {
            eprintln!("Warning: Hourly granularity is limited to 7 days. Please adjust the date range.");
            return Ok(());
        }
    }

    // Load AWS profiles
    let profiles = cli.profiles.clone().unwrap_or_else(get_aws_profile_names);

    if profiles.is_empty() {
        eprintln!("No AWS profiles found in ~/.aws/credentials or ~/.aws/config.");
        return Ok(());
    }

    // Load profile-to-account mapping if provided
    let profile_account_map: HashMap<String, String> = if let Some(map_path) = cli.profile_account_map {
        let map_str = std::fs::read_to_string(&map_path)?;
        serde_json::from_str(&map_str)?
    } else {
        HashMap::new()
    };

    let mut account_cost_data: Vec<AccountCostData> = Vec::new();
    let mut unified_view_data: Vec<UnifiedViewData> = Vec::new();
    let mut global_monthly_totals: HashMap<String, f64> = HashMap::new();
    let mut all_months: Vec<String> = Vec::new();
    let account_id_set: Option<HashSet<String>> = cli.account_id.map(|ids| ids.into_iter().collect());

    // Iterate through each profile
    for profile in &profiles {
        eprintln!("Processing profile: {}", profile);

        // Load AWS configuration for the profile
        let region_provider = RegionProviderChain::default_provider()
            .or_else("us-east-1");
        let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
            .profile_name(profile)
            .region(region_provider)
            .load()
            .await;

        let ce_client = CostExplorerClient::new(&config);
        let org_client = OrganizationsClient::new(&config);
        let sts_client = StsClient::new(&config);

        // Fetch accounts for the profile
        let mut accounts = Vec::new();
        if let Some(account_id) = profile_account_map.get(profile) {
            // Use mapping if provided
            accounts.push(
                aws_sdk_organizations::types::Account::builder()
                    .id(account_id.clone())
                    .name(format!("Account-{}", account_id))
                    .build()
            );
        } else {
            // Try AWS Organizations first
            match org_client.list_accounts().send().await {
                Ok(response) => {
                    accounts.extend(response.accounts.unwrap_or_default());
                }
                Err(e) => {
                    eprintln!("Error fetching accounts for profile {} via Organizations: {}. Attempting STS fallback.", profile, e);
                    // Fallback to STS for standalone account
                    match sts_client.get_caller_identity().send().await {
                        Ok(response) => {
                            if let Some(account_id) = response.account {
                                accounts.push(
                                    aws_sdk_organizations::types::Account::builder()
                                        .id(account_id.clone())
                                        .name(format!("Account-{}", account_id))
                                        .build()
                                );
                            } else {
                                eprintln!("No account ID returned by STS for profile {}. Skipping profile.", profile);
                                continue;
                            }
                        }
                        Err(e) => {
                            eprintln!("Error fetching account ID via STS for profile {}: {}. Skipping profile.", profile, e);
                            continue;
                        }
                    }
                }
            }
        }

        let filtered_accounts = if let Some(ref account_ids) = account_id_set {
            accounts.into_iter()
                .filter(|acc| acc.id.as_ref().map_or(false, |id| account_ids.contains(id)))
                .collect::<Vec<_>>()
        } else {
            accounts
        };

        if filtered_accounts.is_empty() {
            eprintln!("No accounts found for profile {}{}", profile, 
                account_id_set.clone().map_or("".to_string(), |ids| format!(" for account IDs {:?}", ids)));
            continue;
        }

        for account in filtered_accounts {
            let account_id = account.id.unwrap_or_default();
            let account_name = account.name.unwrap_or("N/A".to_string());
            let mut monthly_totals: HashMap<String, f64> = HashMap::new();
            let mut service_monthly_totals: HashMap<String, HashMap<String, f64>> = HashMap::new();

            let mut request_builder = ce_client
                .get_cost_and_usage()
                .time_period(
                    DateInterval::builder()
                        .start(cli.start_date.clone())
                        .end(cli.end_date.clone())
                        .build()?,
                )
                .granularity(cli.granularity.clone().into())
                .metrics("UnblendedCost")
                .group_by(
                    GroupDefinition::builder()
                        .r#type(GroupDefinitionType::Dimension)
                        .key("SERVICE")
                        .build(),
                )
                .filter(
                    aws_sdk_costexplorer::types::Expression::builder()
                        .dimensions(
                            aws_sdk_costexplorer::types::DimensionValues::builder()
                                .key(Dimension::LinkedAccount)
                                .values(account_id.clone())
                                .build(),
                        )
                        .build(),
                );

            if let (Some(tag_key), Some(tag_value)) = (&cli.tag_key, &cli.tag_value) {
                request_builder = request_builder.filter(
                    aws_sdk_costexplorer::types::Expression::builder()
                        .tags(
                            aws_sdk_costexplorer::types::TagValues::builder()
                                .key(tag_key)
                                .values(tag_value)
                                .build(),
                        )
                        .build(),
                );
            } else if let Some(tag_key) = &cli.tag_key {
                request_builder = request_builder.group_by(
                    GroupDefinition::builder()
                        .r#type(GroupDefinitionType::Tag)
                        .key(tag_key)
                        .build(),
                );
            }

            let response = match request_builder.send().await {
                Ok(response) => response,
                Err(e) => {
                    eprintln!("Error fetching cost data for account {} (profile {}): {}. Skipping account.", 
                        account_id, profile, e);
                    continue;
                }
            };

            if let Some(results) = response.results_by_time {
                for result in results {
                    let month = result.time_period.as_ref().map(|tp| tp.start.clone()).unwrap_or_default();
                    let mut total_cost = 0.0;

                    if let Some(groups) = result.groups {
                        for group in groups {
                            let service = group.keys.unwrap_or_default().join(", ");
                            let cost = group
                                .metrics
                                .as_ref()
                                .and_then(|m| m.get("UnblendedCost"))
                                .map(|m| m.amount.as_ref().map(|a| a.parse::<f64>().unwrap_or(0.0)).unwrap_or(0.0))
                                .unwrap_or(0.0);
                            total_cost += cost;

                            let service_monthly = service_monthly_totals
                                .entry(service.clone())
                                .or_insert_with(HashMap::new);
                            *service_monthly.entry(month.clone()).or_insert(0.0) += cost;
                        }
                    }

                    *monthly_totals.entry(month.clone()).or_insert(0.0) += total_cost;
                    *global_monthly_totals.entry(month.clone()).or_insert(0.0) += total_cost;
                    if !all_months.contains(&month) {
                        all_months.push(month);
                    }
                }
            }

            let mut cost_trend = Vec::new();
            let mut previous_cost: Option<f64> = None;
            for month in &all_months {
                let cost = monthly_totals.get(month).unwrap_or(&0.0);
                let mom_change = previous_cost.map(|prev| {
                    if prev == 0.0 { 0.0 } else { ((cost - prev) / prev * 100.0).round() }
                }).unwrap_or(0.0);
                cost_trend.push(CostTrendData {
                    month: month.clone(),
                    total_cost: *cost,
                    mom_change_percent: mom_change,
                });
                previous_cost = Some(*cost);
            }

            let total_cost: f64 = monthly_totals.values().sum();
            let average_monthly_cost = if !all_months.is_empty() {
                total_cost / all_months.len() as f64
            } else {
                0.0
            };

            let mut service_consumption = Vec::new();
            let total_service_cost: f64 = service_monthly_totals
                .iter()
                .flat_map(|(_, months)| months.values())
                .sum();
            for (service, monthly_costs) in service_monthly_totals {
                let service_total_cost: f64 = monthly_costs.values().sum();
                if service_total_cost > 0.0 {
                    service_consumption.push(ServiceConsumptionData {
                        service,
                        monthly_costs,
                        total_cost: service_total_cost,
                        percent_of_total: if total_service_cost > 0.0 {
                            (service_total_cost / total_service_cost * 100.0).round()
                        } else {
                            0.0
                        },
                    });
                }
            }
            service_consumption.sort_by(|a, b| b.total_cost.partial_cmp(&a.total_cost).unwrap_or(std::cmp::Ordering::Equal));

            account_cost_data.push(AccountCostData {
                profile: profile.clone(),
                account_id: account_id.clone(),
                account_name: account_name.clone(),
                cost_trend,
                service_consumption,
                total_cost,
                average_monthly_cost,
            });

            unified_view_data.push(UnifiedViewData {
                profile: profile.clone(),
                account_id,
                account_name,
                monthly_costs: monthly_totals,
            });
        }
    }

    if account_cost_data.is_empty() {
        eprintln!("No cost data retrieved for any accounts across specified profiles.");
        return Ok(());
    }

    all_months.sort();
    let filtered_months: Vec<String> = all_months
        .into_iter()
        .filter(|m| {
            NaiveDate::parse_from_str(m, "%Y-%m-%d").map_or(false, |d| d >= six_months_ago)
        })
        .collect();

    let total_global_cost: f64 = global_monthly_totals.values().sum();
    let average_global_monthly_cost = if !filtered_months.is_empty() {
        total_global_cost / filtered_months.len() as f64
    } else {
        0.0
    };

    // JSON Output
    if cli.json {
        let output = serde_json::json!({
            "accounts": account_cost_data,
            "unified_view": unified_view_data,
            "global_summary": {
                "total_cost": total_global_cost,
                "average_monthly_cost": average_global_monthly_cost
            }
        });
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        // Unified View Table with pagination for large datasets
        let max_columns = 10; // Adjust this based on terminal width
        for chunk in filtered_months.chunks(max_columns - 3) { // -3 for Profile, Account ID, Account Name
            let mut unified_table = Table::new();
            unified_table.set_format(*format::consts::FORMAT_DEFAULT); // Restore grid lines
            let mut unified_titles = vec![
                Cell::new("Profile").style_spec("bFc"),
                Cell::new("Account ID").style_spec("bFc"),
                Cell::new("Account Name").style_spec("bFc"),
            ];
            for month in chunk {
                unified_titles.push(Cell::new(month).style_spec("bFr"));
            }
            unified_table.set_titles(Row::new(unified_titles));

            for account in &unified_view_data {
                let mut row = vec![
                    Cell::new(&account.profile),
                    Cell::new(&account.account_id),
                    Cell::new(&account.account_name),
                ];
                for month in chunk {
                    let cost = account.monthly_costs.get(month).unwrap_or(&0.0);
                    row.push(Cell::new(&format!("{:.2}", cost)).style_spec("Fr"));
                }
                unified_table.add_row(Row::new(row));
            }

            println!("\nUnified Cost View (Past 6 Months) - Page {}:", (filtered_months.iter().position(|m| m == chunk[0].as_str()).unwrap() / (max_columns - 3)) + 1);
            unified_table.printstd();
        }

        // Per-Account Tables
        for account_data in &account_cost_data {
            let mut trend_table = Table::new();
            trend_table.set_format(*format::consts::FORMAT_DEFAULT);
            trend_table.set_titles(Row::new(vec![
                Cell::new("Month").style_spec("bFc"),
                Cell::new("Total Cost (USD)").style_spec("bFr"),
                Cell::new("MoM Change (%)").style_spec("bFc"),
            ]));

            for data in &account_data.cost_trend {
                trend_table.add_row(Row::new(vec![
                    Cell::new(&data.month),
                    Cell::new(&format!("{:.2}", data.total_cost)).style_spec("Fr"),
                    Cell::new(&format!("{:.1}", data.mom_change_percent)).style_spec("Fc"),
                ]));
            }

            println!("\nCost Trend Analysis for Profile {} Account {} ({}):", 
                account_data.profile, account_data.account_id, account_data.account_name);
            trend_table.printstd();
            println!("Total Cost ({} to {}): ${:.2}", cli.start_date, cli.end_date, account_data.total_cost);
            println!("Average Monthly Cost: ${:.2}", account_data.average_monthly_cost);

            // Service Consumption Table with pagination
            for chunk in filtered_months.chunks(max_columns - 2) { // -2 for Service, Total Cost, Percent of Total
                let mut service_table = Table::new();
                service_table.set_format(*format::consts::FORMAT_DEFAULT);
                let mut service_titles = vec![
                    Cell::new("Service").style_spec("bFc"),
                ];
                for month in chunk {
                    service_titles.push(Cell::new(month).style_spec("bFr"));
                }
                service_titles.push(Cell::new("Total Cost (USD)").style_spec("bFr"));
                service_titles.push(Cell::new("Percent of Total (%)").style_spec("bFc"));

                service_table.set_titles(Row::new(service_titles));

                for data in &account_data.service_consumption {
                    let mut row = vec![Cell::new(&data.service)];
                    for month in chunk {
                        let cost = data.monthly_costs.get(month).unwrap_or(&0.0);
                        row.push(Cell::new(&format!("{:.2}", cost)).style_spec("Fr"));
                    }
                    row.push(Cell::new(&format!("{:.2}", data.total_cost)).style_spec("Fr"));
                    row.push(Cell::new(&format!("{:.1}", data.percent_of_total)).style_spec("Fc"));
                    service_table.add_row(Row::new(row));
                }

                println!(
                    "\nService Consumption Summary for Profile {} Account {} ({} to {}) - Page {}:",
                    account_data.profile, account_data.account_id, cli.start_date, cli.end_date,
                    (filtered_months.iter().position(|m| m == chunk[0].as_str()).unwrap() / (max_columns - 2)) + 1
                );
                service_table.printstd();
            }
        }

        // Global Summary
        println!("\nGlobal Summary (All Accounts):");
        println!("Total Cost ({} to {}): ${:.2}", cli.start_date, cli.end_date, total_global_cost);
        println!("Average Monthly Cost: ${:.2}", average_global_monthly_cost);
    }

    // Chart Output
    if cli.chart {
        for account_data in &account_cost_data {
            if account_data.cost_trend.is_empty() {
                eprintln!("Warning: No cost trend data available for profile {} account {}. Skipping chart generation.", 
                    account_data.profile, account_data.account_id);
                continue;
            }
            let chart_path = format!("cost_trend_profile_{}_account_{}.png", 
                account_data.profile, account_data.account_id);
            match generate_cost_trend_chart(&account_data.cost_trend, &chart_path) {
                Ok(()) => println!("Cost trend chart saved to {}", chart_path),
                Err(e) => eprintln!("Failed to generate chart for profile {} account {}: {}", 
                    account_data.profile, account_data.account_id, e),
            }
        }
    }

    // CSV Output
    if let Some(csv_path) = cli.csv {
        for account_data in &account_cost_data {
            let trend_csv_path = format!(
                "{}_trend_profile_{}_account_{}.csv", 
                csv_path.trim_end_matches(".csv"), 
                account_data.profile, 
                account_data.account_id
            );
            let mut trend_writer = Writer::from_path(&trend_csv_path)?;
            trend_writer.write_record(&["Month", "Total Cost (USD)", "MoM Change (%)"])?;
            for data in &account_data.cost_trend {
                trend_writer.write_record(&[
                    data.month.clone(),
                    format!("{:.2}", data.total_cost),
                    format!("{:.1}", data.mom_change_percent),
                ])?;
            }
            trend_writer.flush()?;
            println!("Exported trend report for profile {} account {} to {}", 
                account_data.profile, account_data.account_id, trend_csv_path);

            let service_csv_path = format!(
                "{}_service_summary_profile_{}_account_{}.csv",
                csv_path.trim_end_matches(".csv"),
                account_data.profile,
                account_data.account_id
            );
            let mut service_writer = Writer::from_path(&service_csv_path)?;
            let mut headers = vec!["Service".to_string()];
            headers.extend(filtered_months.iter().map(|m| m.clone()));
            headers.push("Total Cost (USD)".to_string());
            headers.push("Percent of Total (%)".to_string());
            service_writer.write_record(&headers)?;
            for data in &account_data.service_consumption {
                let mut row = vec![data.service.clone()];
                for month in &filtered_months {
                    let cost = data.monthly_costs.get(month).unwrap_or(&0.0);
                    row.push(format!("{:.2}", cost));
                }
                row.push(format!("{:.2}", data.total_cost));
                row.push(format!("{:.1}", data.percent_of_total));
                service_writer.write_record(&row)?;
            }
            service_writer.flush()?;
            println!(
                "Exported service summary for profile {} account {} to {}",
                account_data.profile, account_data.account_id, service_csv_path
            );
        }

        let global_csv_path = format!("{}_global_summary.csv", csv_path.trim_end_matches(".csv"));
        let mut global_writer = Writer::from_path(&global_csv_path)?;
        global_writer.write_record(&["Metric", "Value"])?;
        global_writer.write_record(&["Total Cost (USD)", format!("{:.2}", total_global_cost).as_ref()])?;
        global_writer.write_record(&["Average Monthly Cost (USD)", format!("{:.2}", average_global_monthly_cost).as_ref()])?;
        global_writer.flush()?;
        println!("Exported global summary to {}", global_csv_path);

        let unified_csv_path = format!("{}_unified_view.csv", csv_path.trim_end_matches(".csv"));
        let mut unified_writer = Writer::from_path(&unified_csv_path)?;
        let mut headers = vec!["Profile".to_string(), "Account ID".to_string(), "Account Name".to_string()];
        headers.extend(filtered_months.iter().map(|m| m.clone()));
        unified_writer.write_record(&headers)?;
        for account in &unified_view_data {
            let mut row = vec![account.profile.clone(), account.account_id.clone(), account.account_name.clone()];
            for month in &filtered_months {
                let cost = account.monthly_costs.get(month).unwrap_or(&0.0);
                row.push(format!("{:.2}", cost));
            }
            unified_writer.write_record(&row)?;
        }
        unified_writer.flush()?;
        println!("Exported unified view to {}", unified_csv_path);
    }

    Ok(())
}

fn generate_cost_trend_chart(cost_trend: &[CostTrendData], output_path: &str) -> Result<(), Box<dyn Error>> {
    if cost_trend.is_empty() {
        return Err("No data available to generate chart".into());
    }

    let root = BitMapBackend::new(output_path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    let months: Vec<String> = cost_trend.iter().map(|data| data.month.clone()).collect();
    let costs: Vec<f64> = cost_trend.iter().map(|data| data.total_cost).collect();
    let max_cost = costs.iter().cloned().fold(0.0, f64::max).max(1.0);
    let num_months = months.len();

    let mut chart = ChartBuilder::on(&root)
        .caption("Cost Trend Analysis", ("sans-serif", 40))
        .x_label_area_size(40)
        .y_label_area_size(40)
        .margin(10)
        .build_cartesian_2d(0..num_months, 0.0..max_cost + 100.0)?;

    chart.configure_mesh()
        .x_labels(num_months)
        .x_label_formatter(&|i| {
            if *i < months.len() {
                months[*i].clone()
            } else {
                String::new()
            }
        })
        .y_desc("Cost (USD)")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(BLUE.filled())
            .data(costs.iter().enumerate().map(|(i, cost)| (i, *cost))),
    )?;

    root.present()?;
    Ok(())
}

