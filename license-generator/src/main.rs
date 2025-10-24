use chrono::{Duration, NaiveDate, Utc};
use sha2::{Digest, Sha256};
use std::io::{self, Write};

// IMPORTANTE: Questo deve essere lo STESSO secret del file license.rs
const SECRET_KEY: &str = "WORKFLOW_AUTOMATOR_SECRET_2025_CHANGE_IN_PRODUCTION";

#[derive(Debug, Clone)]
enum LicenseType {
    Monthly,    // 30 giorni
    Quarterly,  // 90 giorni
    Annual,     // 365 giorni
}

impl LicenseType {
    fn to_code(&self) -> &str {
        match self {
            LicenseType::Monthly => "M",
            LicenseType::Quarterly => "T",
            LicenseType::Annual => "A",
        }
    }

    fn to_string(&self) -> &str {
        match self {
            LicenseType::Monthly => "Mensile (30 giorni)",
            LicenseType::Quarterly => "Trimestrale (90 giorni)",
            LicenseType::Annual => "Annuale (365 giorni)",
        }
    }

    fn days(&self) -> i64 {
        match self {
            LicenseType::Monthly => 30,
            LicenseType::Quarterly => 90,
            LicenseType::Annual => 365,
        }
    }
}

fn calculate_hash(license_type: &str, expiration_date: &str) -> String {
    let data = format!("{}{}{}", license_type, expiration_date, SECRET_KEY);
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)[..8].to_uppercase()
}

fn generate_license_key(license_type: &LicenseType, expiration_date: &str) -> String {
    let type_code = license_type.to_code();
    let hash = calculate_hash(type_code, expiration_date);
    format!("WA-{}-{}-{}", type_code, expiration_date, hash)
}

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║      WORKFLOW AUTOMATOR - GENERATORE CHIAVI LICENZA     ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    println!();

    loop {
        println!("Scegli il tipo di licenza:");
        println!("  1. Mensile (30 giorni)");
        println!("  2. Trimestrale (90 giorni)");
        println!("  3. Annuale (365 giorni)");
        println!("  4. Esci");
        print!("\nScelta (1-4): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        if choice == "4" {
            println!("\n👋 Arrivederci!");
            break;
        }

        let license_type = match choice {
            "1" => LicenseType::Monthly,
            "2" => LicenseType::Quarterly,
            "3" => LicenseType::Annual,
            _ => {
                println!("❌ Scelta non valida!\n");
                continue;
            }
        };

        println!("\nOpzioni data scadenza:");
        println!("  1. Da oggi + {} giorni", license_type.days());
        println!("  2. Inserisci data manualmente (YYYYMMDD)");
        print!("\nScelta (1-2): ");
        io::stdout().flush().unwrap();

        let mut date_choice = String::new();
        io::stdin().read_line(&mut date_choice).unwrap();
        let date_choice = date_choice.trim();

        let expiration_date = match date_choice {
            "1" => {
                let today = Utc::now().date_naive();
                let exp_date = today + Duration::days(license_type.days());
                exp_date.format("%Y%m%d").to_string()
            }
            "2" => {
                print!("Inserisci data scadenza (YYYYMMDD): ");
                io::stdout().flush().unwrap();
                let mut date = String::new();
                io::stdin().read_line(&mut date).unwrap();
                let date = date.trim().to_string();

                // Valida formato
                if date.len() != 8 {
                    println!("❌ Formato data non valido! Deve essere YYYYMMDD\n");
                    continue;
                }

                if let Err(_) = NaiveDate::parse_from_str(&date, "%Y%m%d") {
                    println!("❌ Data non valida!\n");
                    continue;
                }

                date
            }
            _ => {
                println!("❌ Scelta non valida!\n");
                continue;
            }
        };

        // Genera la chiave
        let key = generate_license_key(&license_type, &expiration_date);

        // Mostra risultato
        println!("\n╔══════════════════════════════════════════════════════════╗");
        println!("║                  CHIAVE GENERATA CON SUCCESSO            ║");
        println!("╚══════════════════════════════════════════════════════════╝");
        println!();
        println!("📋 Tipo Licenza:  {}", license_type.to_string());
        println!("📅 Data Scadenza: {}", expiration_date);
        println!();
        println!("🔑 CHIAVE DI LICENZA:");
        println!("   ┌────────────────────────────────────────┐");
        println!("   │  {}  │", key);
        println!("   └────────────────────────────────────────┘");
        println!();
        println!("💡 Copia questa chiave e forniscila al cliente.");
        println!();

        // Chiedi se generare altra chiave
        print!("Generare un'altra chiave? (s/n): ");
        io::stdout().flush().unwrap();
        let mut another = String::new();
        io::stdin().read_line(&mut another).unwrap();

        if another.trim().to_lowercase() != "s" {
            println!("\n👋 Arrivederci!");
            break;
        }

        println!("\n{}\n", "=".repeat(60));
    }
}
