use clap::{Parser, Subcommand};
use chrono::prelude::*;
use serde::Deserialize;

/// Simple program to greet a person
#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]


struct Cli {
    #[command(subcommand)]
    ///Name of the service to get info from
    service: Command,
}

#[derive(Subcommand)]
#[derive(Debug)]
enum Command {
    /// Get the status of the Electicity
    Electricity ,
    /// Get the status of the Water
    Water,
}

#[derive(Deserialize, Debug)]
struct ElectricityData {
    couleurJourJ: String,
    couleurJourJ1: String,
}




#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let local_time: DateTime<Local> = Local::now();
    let today = local_time.format("%Y-%m-%d").to_string();
    match &cli.service {
        Command::Electricity => {
            let electricity = get_electricity(&today).await.unwrap();
            let today_color = match electricity.couleurJourJ.as_str() {
                "TEMPO_BLEU" => "🔵",
                "TEMPO_BLANC" => "⚪️",
                "TEMPO_ROUGE" => "🔴",
                _ => "unknown"
            };
            println!("Today is {:?}", today_color);
            let tomorrow_color = match electricity.couleurJourJ1.as_str() {
                "TEMPO_BLEU" => "🔵",
                "TEMPO_BLANC" => "⚪️",
                "TEMPO_ROUGE" => "🔴",
                _ => "unknown"
            };
            println!("Tomorrow will be {:?}", tomorrow_color)
        },
        Command::Water => {
            println!("Getting the version of the service {:?}", cli.service)
        }
    }
    
    
}

async fn get_electricity(date_str: &str) -> Result<ElectricityData, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = format!("https://particulier.edf.fr/services/rest/referentiel/searchTempoStore?dateRelevant={}", date_str);

    let resp = client.get(&url)
        .header("User-Agent", "curl/8.1.2") // Mimic the user agent from curl
        .header("Accept", "*/*") // Explicitly accept any content type
        .send()
        .await?
        .json::<ElectricityData>()
        .await?;
    Ok(resp)
}
