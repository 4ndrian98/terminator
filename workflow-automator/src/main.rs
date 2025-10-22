use anyhow::{Context, Result};
use calamine::{open_workbook, Reader, Xlsx};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use terminator_workflow_recorder::{WorkflowRecorder, WorkflowRecorderConfig};
use tokio::signal::ctrl_c;
use tracing::{info, error};

/// Automazione Workflow - Applicazione Standalone per Clienti
/// 
/// Permette di:
/// 1. Registrare workflow ripetitivi
/// 2. Eseguire workflow salvati
/// 3. Automatizzare da Excel: legge dati da Excel e ripete il workflow
#[derive(Parser)]
#[command(name = "Workflow Automator")]
#[command(version = "1.0")]
#[command(about = "Automatizza workflow ripetitivi - Facile da usare!", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Registra un nuovo workflow (premi Ctrl+C per fermare)
    Registra {
        /// Nome del workflow da salvare
        #[arg(short, long, default_value = "mio_workflow")]
        nome: String,
    },
    
    /// Esegui un workflow salvato (una volta)
    Esegui {
        /// File del workflow da eseguire (.json)
        #[arg(short, long)]
        workflow: PathBuf,
    },
    
    /// Automatizza da Excel: ripete il workflow per ogni riga
    Excel {
        /// File Excel con i dati (.xlsx)
        #[arg(short, long)]
        excel: PathBuf,
        
        /// File del workflow da ripetere (.json)
        #[arg(short, long)]
        workflow: PathBuf,
        
        /// Colonna da cui iniziare (default: A)
        #[arg(long, default_value = "A")]
        colonna_inizio: String,
        
        /// Riga da cui iniziare (default: 2, salta intestazione)
        #[arg(long, default_value = "2")]
        riga_inizio: u32,
    },
    
    /// Guida rapida per iniziare
    Guida,
}

#[derive(Debug, Serialize, Deserialize)]
struct ExcelRow {
    data: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Setup logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Registra { nome } => {
            registra_workflow(nome).await?;
        }
        Commands::Esegui { workflow } => {
            esegui_workflow(workflow).await?;
        }
        Commands::Excel {
            excel,
            workflow,
            colonna_inizio,
            riga_inizio,
        } => {
            automatizza_da_excel(excel, workflow, colonna_inizio, riga_inizio).await?;
        }
        Commands::Guida => {
            mostra_guida();
        }
    }

    Ok(())
}

/// Registra un nuovo workflow
async fn registra_workflow(nome: String) -> Result<()> {
    info!("🎬 INIZIA REGISTRAZIONE WORKFLOW: {}", nome);
    info!("");
    info!("📝 Esegui ora le azioni che vuoi automatizzare:");
    info!("   - Apri programmi");
    info!("   - Clicca pulsanti");
    info!("   - Compila campi");
    info!("   - Copia/Incolla dati");
    info!("");
    info!("🛑 Premi Ctrl+C quando hai finito");
    info!("");

    let config = WorkflowRecorderConfig::default();
    let mut recorder = WorkflowRecorder::new(nome.clone(), config);

    recorder.start().await?;
    
    // Aspetta Ctrl+C
    ctrl_c().await?;
    
    info!("");
    info!("🛑 Registrazione fermata, salvataggio in corso...");
    
    recorder.stop().await?;
    
    let output_file = format!("{}.json", nome);
    recorder.save(&output_file)?;
    
    info!("");
    info!("✅ Workflow salvato con successo in: {}", output_file);
    info!("");
    info!("▶️  Per eseguire il workflow:");
    info!("   workflow-automator.exe esegui --workflow {}", output_file);
    info!("");
    
    Ok(())
}

/// Esegui un workflow salvato
async fn esegui_workflow(workflow_path: PathBuf) -> Result<()> {
    info!("▶️  Esecuzione workflow: {:?}", workflow_path);
    
    // Carica il workflow
    let workflow_data = std::fs::read_to_string(&workflow_path)
        .context("Errore nella lettura del file workflow")?;
    
    let workflow: serde_json::Value = serde_json::from_str(&workflow_data)
        .context("Errore nel parsing del workflow")?;
    
    info!("✅ Workflow caricato");
    
    // Qui implementeresti la logica di esecuzione usando terminator
    // Per ora mostriamo cosa dovrebbe fare
    
    info!("");
    info!("🚀 Esecuzione degli step del workflow...");
    
    // TODO: Implementa esecuzione usando terminator APIs
    // Esempio concettuale:
    /*
    for event in workflow["events"].as_array() {
        match event["type"] {
            "click" => desktop.locator(selector).click(),
            "type" => desktop.locator(selector).type(text),
            ...
        }
    }
    */
    
    info!("⚠️  NOTA: Implementazione esecuzione in sviluppo");
    info!("   Il workflow è stato caricato correttamente");
    
    Ok(())
}

/// Automatizza workflow usando dati da Excel
async fn automatizza_da_excel(
    excel_path: PathBuf,
    workflow_path: PathBuf,
    colonna_inizio: String,
    riga_inizio: u32,
) -> Result<()> {
    info!("📊 Automazione da Excel");
    info!("   Excel: {:?}", excel_path);
    info!("   Workflow: {:?}", workflow_path);
    info!("");

    // Apri Excel
    let mut workbook: Xlsx<_> = open_workbook(&excel_path)
        .context("Impossibile aprire il file Excel")?;
    
    // Prendi il primo foglio
    let sheet_name = workbook.sheet_names()[0].clone();
    let range = workbook.worksheet_range(&sheet_name)
        .context("Impossibile leggere il foglio Excel")?;
    
    info!("✅ Excel aperto: {} righe trovate", range.height());
    
    // Carica workflow
    let workflow_data = std::fs::read_to_string(&workflow_path)?;
    let _workflow: serde_json::Value = serde_json::from_str(&workflow_data)?;
    
    info!("✅ Workflow caricato");
    info!("");
    
    // Processa ogni riga
    let mut processed = 0;
    for row_idx in (riga_inizio as usize)..range.height() {
        let mut row_data = Vec::new();
        
        // Leggi tutte le colonne della riga
        for col_idx in 0..range.width() {
            if let Some(cell) = range.get((row_idx, col_idx)) {
                row_data.push(cell.to_string());
            }
        }
        
        if row_data.is_empty() {
            break; // Fine dei dati
        }
        
        info!("📝 Riga {}: {:?}", row_idx + 1, row_data);
        
        // Qui eseguiresti il workflow con i dati della riga
        // TODO: Implementa esecuzione workflow con sostituzione dati
        /*
        for event in workflow["events"] {
            // Sostituisci placeholder con dati Excel
            // Es: {{col_A}} -> row_data[0]
            execute_event_with_data(event, &row_data);
        }
        */
        
        processed += 1;
        
        // Pausa tra le esecuzioni
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }
    
    info!("");
    info!("✅ Completato! {} righe processate", processed);
    info!("⚠️  NOTA: Esecuzione automatica in sviluppo");
    
    Ok(())
}

/// Mostra guida rapida
fn mostra_guida() {
    println!(r#"
╔════════════════════════════════════════════════════════════════╗
║                    WORKFLOW AUTOMATOR                          ║
║              Guida Rapida per Automatizzare Workflow           ║
╚════════════════════════════════════════════════════════════════╝

📚 COME FUNZIONA:

1️⃣  REGISTRA il tuo workflow (una volta sola):
   
   workflow-automator.exe registra --nome "compila_form"
   
   Poi esegui manualmente le azioni che vuoi automatizzare.
   Premi Ctrl+C quando hai finito.

2️⃣  ESEGUI il workflow quando vuoi:
   
   workflow-automator.exe esegui --workflow compila_form.json

3️⃣  AUTOMATIZZA con Excel (ripete per ogni riga):
   
   workflow-automator.exe excel \
     --excel dati.xlsx \
     --workflow compila_form.json

───────────────────────────────────────────────────────────────

💡 ESEMPIO PRATICO:

Devi compilare un form 100 volte con dati da Excel?

1. Registra: compili il form UNA VOLTA mentre registri
2. Prepara Excel con i 100 dati (una riga per ogni compilazione)
3. Esegui: il programma ripete il workflow per tutte le 100 righe

───────────────────────────────────────────────────────────────

📋 FORMATO EXCEL:

Il file Excel deve avere:
- Prima riga: intestazioni (opzionale)
- Righe successive: i dati da usare

Esempio:
  A          B         C
  Nome       Email     Telefono
  Mario      m@.it     123456
  Luigi      l@.it     789012

Il programma userà questi dati per compilare i campi.

───────────────────────────────────────────────────────────────

❓ SERVE AIUTO?

Per ulteriori informazioni:
- Email: supporto@esempio.com
- Docs: https://docs.esempio.com

"#);
}
