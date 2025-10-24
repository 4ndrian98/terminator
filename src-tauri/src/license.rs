// Sistema di gestione licenze
use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use chrono::{NaiveDate, Utc};
use sha2::{Sha256, Digest};

// Secret key per la generazione/validazione delle chiavi
// IMPORTANTE: Cambia questo valore in produzione e tienilo segreto!
const SECRET_KEY: &str = "WORKFLOW_AUTOMATOR_SECRET_2025_CHANGE_IN_PRODUCTION";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LicenseType {
    Monthly,    // 30 giorni
    Quarterly,  // 90 giorni
    Annual,     // 365 giorni
}

impl LicenseType {
    pub fn from_code(code: &str) -> Option<Self> {
        match code {
            "M" => Some(LicenseType::Monthly),
            "T" => Some(LicenseType::Quarterly),
            "A" => Some(LicenseType::Annual),
            _ => None,
        }
    }

    pub fn to_code(&self) -> &str {
        match self {
            LicenseType::Monthly => "M",
            LicenseType::Quarterly => "T",
            LicenseType::Annual => "A",
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            LicenseType::Monthly => "Mensile".to_string(),
            LicenseType::Quarterly => "Trimestrale".to_string(),
            LicenseType::Annual => "Annuale".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct License {
    pub license_key: String,
    pub license_type: LicenseType,
    pub expiration_date: String, // YYYYMMDD
    pub activated_at: i64,       // timestamp
}

impl License {
    pub fn is_valid(&self) -> bool {
        // Verifica che la licenza non sia scaduta
        if let Ok(exp_date) = NaiveDate::parse_from_str(&self.expiration_date, "%Y%m%d") {
            let today = Utc::now().date_naive();
            return exp_date >= today;
        }
        false
    }

    pub fn days_remaining(&self) -> i32 {
        if let Ok(exp_date) = NaiveDate::parse_from_str(&self.expiration_date, "%Y%m%d") {
            let today = Utc::now().date_naive();
            return (exp_date - today).num_days() as i32;
        }
        0
    }
}

/// Valida una chiave di licenza
/// Formato: WA-<TIPO>-<YYYYMMDD>-<HASH>
/// Esempio: WA-M-20250830-ABC12345
pub fn validate_license_key(key: &str) -> Result<License> {
    let parts: Vec<&str> = key.split('-').collect();
    
    if parts.len() != 4 {
        return Err(anyhow!("Formato chiave non valido"));
    }

    if parts[0] != "WA" {
        return Err(anyhow!("Chiave non valida"));
    }

    let license_type = LicenseType::from_code(parts[1])
        .ok_or_else(|| anyhow!("Tipo licenza non valido"))?;

    let expiration_date = parts[2];
    if expiration_date.len() != 8 {
        return Err(anyhow!("Data scadenza non valida"));
    }

    // Verifica che la data sia valida
    NaiveDate::parse_from_str(expiration_date, "%Y%m%d")
        .context("Data scadenza non valida")?;

    let provided_hash = parts[3];

    // Calcola l'hash atteso
    let expected_hash = calculate_hash(parts[1], expiration_date);

    if provided_hash != expected_hash {
        return Err(anyhow!("Chiave non valida (verifica fallita)"));
    }

    Ok(License {
        license_key: key.to_string(),
        license_type,
        expiration_date: expiration_date.to_string(),
        activated_at: Utc::now().timestamp(),
    })
}

/// Calcola l'hash per una chiave di licenza
fn calculate_hash(license_type: &str, expiration_date: &str) -> String {
    let data = format!("{}{}{}", license_type, expiration_date, SECRET_KEY);
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let result = hasher.finalize();
    
    // Prendi i primi 8 caratteri in maiuscolo
    hex::encode(result)[..8].to_uppercase()
}

/// Genera una nuova chiave di licenza
pub fn generate_license_key(license_type: LicenseType, expiration_date: &str) -> Result<String> {
    // Verifica formato data
    NaiveDate::parse_from_str(expiration_date, "%Y%m%d")
        .context("Data scadenza non valida")?;

    let type_code = license_type.to_code();
    let hash = calculate_hash(type_code, expiration_date);

    Ok(format!("WA-{}-{}-{}", type_code, expiration_date, hash))
}

/// Path del file di licenza
fn get_license_file_path() -> Result<PathBuf> {
    let app_data = std::env::var("APPDATA")
        .or_else(|_| std::env::var("HOME"))
        .context("Impossibile trovare directory di configurazione")?;
    
    let config_dir = PathBuf::from(app_data).join("WorkflowAutomator");
    fs::create_dir_all(&config_dir)?;
    
    Ok(config_dir.join("license.json"))
}

/// Salva la licenza su disco
pub fn save_license(license: &License) -> Result<()> {
    let path = get_license_file_path()?;
    let json = serde_json::to_string_pretty(license)?;
    fs::write(path, json)?;
    Ok(())
}

/// Carica la licenza da disco
pub fn load_license() -> Result<License> {
    let path = get_license_file_path()?;
    
    if !path.exists() {
        return Err(anyhow!("Nessuna licenza trovata"));
    }

    let content = fs::read_to_string(path)?;
    let license: License = serde_json::from_str(&content)?;
    
    Ok(license)
}

/// Rimuove la licenza dal disco
pub fn remove_license() -> Result<()> {
    let path = get_license_file_path()?;
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_and_validate() {
        let key = generate_license_key(LicenseType::Monthly, "20251231").unwrap();
        println!("Chiave generata: {}", key);
        
        let license = validate_license_key(&key).unwrap();
        assert_eq!(license.expiration_date, "20251231");
    }

    #[test]
    fn test_invalid_key() {
        let result = validate_license_key("WA-M-20251231-INVALID");
        assert!(result.is_err());
    }

    #[test]
    fn test_hash_consistency() {
        let hash1 = calculate_hash("M", "20251231");
        let hash2 = calculate_hash("M", "20251231");
        assert_eq!(hash1, hash2);
    }
}
