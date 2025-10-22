# Workflow Automator - Soluzione Standalone per Clienti

## 🎯 Obiettivo

Creare un **singolo .exe** distribuibile a clienti NON tecnici per automatizzare workflow ripetitivi, inclusa l'integrazione con Excel.

## ✨ Caratteristiche

- ✅ **Zero Dipendenze**: Un singolo .exe, niente Python/Node.js richiesti
- ✅ **Facile da Usare**: Interfaccia CLI semplice in italiano
- ✅ **Registrazione Workflow**: Registra azioni desktop in modo visivo
- ✅ **Esecuzione Workflow**: Ripete azioni automaticamente
- ✅ **Integrazione Excel**: Legge dati da .xlsx e ripete workflow
- ✅ **Cross-Window**: Funziona con qualsiasi applicazione Windows

## 🏗️ Architettura

```
workflow-automator/
├── src/
│   └── main.rs              # Applicazione principale
├── Cargo.toml               # Dipendenze Rust
├── build.ps1                # Script di build
├── GUIDA_CLIENTE.md         # Documentazione per l'utente finale
└── README.md                # Questa guida (per sviluppatori)
```

## 🔧 Build per Distribuzione

### Prerequisiti
- Rust toolchain installato
- Windows 10/11
- Git (per clonare dipendenze)

### Compilazione

```powershell
# Opzione 1: Usa lo script automatico
.\build.ps1

# Opzione 2: Build manuale
cargo build --release
```

Il file `target/release/workflow-automator.exe` è pronto per la distribuzione!

### Ottimizzazioni Build

Il `Cargo.toml` include ottimizzazioni per ridurre dimensione:
- `opt-level = "z"` - Ottimizza per dimensione minima
- `lto = true` - Link Time Optimization
- `strip = true` - Rimuove simboli debug
- Risultato: ~10-15 MB (dipende dalle dipendenze)

## 📦 Distribuzione ai Clienti

### Pacchetto da Fornire

```
workflow-automator-v1.0/
├── workflow-automator.exe     # L'eseguibile
├── GUIDA_CLIENTE.md           # Guida in italiano
└── esempi/                    # (opzionale) Esempi
    ├── esempio.xlsx
    └── esempio_workflow.json
```

### Cosa Dire al Cliente

> **"Questo programma ti permette di automatizzare compiti ripetitivi.**
> 
> 1. Copia `workflow-automator.exe` sul tuo PC
> 2. Apri il Prompt dei Comandi
> 3. Esegui: `workflow-automator.exe guida`
> 
> Non serve installare nulla. Funziona subito!"

## 🎨 User Experience per il Cliente

### Workflow Tipico del Cliente

```powershell
# 1. Registra una volta
C:\> workflow-automator.exe registra --nome "inserimento_ordini"
# Cliente fa le azioni manualmente mentre il programma registra
# Premi Ctrl+C quando finito

# 2. Esegui quando serve
C:\> workflow-automator.exe esegui --workflow inserimento_ordini.json

# 3. Automatizza con Excel (100+ righe)
C:\> workflow-automator.exe excel --excel ordini.xlsx --workflow inserimento_ordini.json
# Il programma processa tutte le righe automaticamente
```

## 🛠️ Sviluppo e Personalizzazione

### Aggiungere Nuove Funzionalità

Modifica `src/main.rs` per aggiungere comandi:

```rust
#[derive(Subcommand)]
enum Commands {
    // ... comandi esistenti
    
    /// Nuovo comando personalizzato
    MioComando {
        #[arg(short, long)]
        parametro: String,
    },
}
```

### Testing Locale

```powershell
# Build e test veloce
cargo run -- guida
cargo run -- registra --nome "test"
```

## 📊 Integrazione Excel

### Formato Excel Supportato

```
     A          B         C          D
1    Nome       Email     Telefono   Note
2    Mario      m@.it     123456     Cliente VIP
3    Luigi      l@.it     789012     Nuovo
```

- **Riga 1**: Intestazioni (opzionale)
- **Righe 2+**: Dati da processare
- **Colonne**: Illimitate

### Placeholder nel Workflow

I dati Excel possono sostituire placeholder nel workflow:
- `{{col_A}}` → Valore colonna A
- `{{col_B}}` → Valore colonna B
- etc.

## 🔐 Sicurezza

- ✅ Nessun accesso a Internet richiesto
- ✅ Non raccoglie dati utente
- ✅ Opera solo con permessi utente locale
- ✅ Workflow salvati localmente

## 🚀 Deployment su Scala

### Per Distribuire a Molti Clienti

1. **Branding**: Personalizza nome e icona in `Cargo.toml`
2. **Sign**: Firma digitalmente il .exe (opzionale ma raccomandato)
3. **Installer**: Crea installer con Inno Setup o NSIS (opzionale)
4. **Updates**: Implementa sistema di auto-update (futuro)

### Firma Digitale (Windows)

```powershell
# Usa signtool.exe (Windows SDK)
signtool sign /f "certificato.pfx" /p "password" workflow-automator.exe
```

## 📈 Versioning

Gestione versioni in `Cargo.toml`:

```toml
[package]
version = "1.0.0"  # Incrementa per release
```

Build con versione:
```powershell
cargo build --release
# Il .exe avrà metadata versione incorporato
```

## 🐛 Troubleshooting

### Errore: "missing VCRUNTIME140.dll"

**Soluzione**: Include Visual C++ Redistributables o compila static:

```toml
[target.x86_64-pc-windows-msvc]
rustflags = ["-C", "target-feature=+crt-static"]
```

### Errore: File Excel non si apre

**Causa**: Formato non supportato (solo .xlsx)

**Soluzione**: Chiedi al cliente di salvare come .xlsx

### Il workflow non si ripete correttamente

**Causa**: Timing o elementi UI cambiati

**Soluzione**: Aggiungi delay nel workflow o registra di nuovo

## 📝 TODO / Future Features

- [ ] GUI con interfaccia grafica (usando `eframe`)
- [ ] Editor workflow visuale
- [ ] Supporto Word/PDF oltre a Excel
- [ ] Scheduler integrato (esegui a orari prestabiliti)
- [ ] Cloud sync workflow (opzionale)
- [ ] Statistiche esecuzioni
- [ ] Report errori via email

## 🤝 Support

Per supporto tecnico:
- Email: dev@esempio.com
- Issues: GitHub repo
- Docs: https://docs.esempio.com

## 📄 License

[Scegli la tua licenza]

---

## 🎓 Best Practices per Clienti

### Cose da Insegnare ai Clienti:

1. **Registrazioni Pulite**: Fare azioni lente e chiare
2. **Test Prima**: Provare con 2-3 righe Excel prima di tutto
3. **Backup Workflow**: Salvare i .json in sicurezza
4. **Naming**: Usare nomi descrittivi per workflow
5. **Documentare**: Annotare cosa fa ogni workflow

### Template Email per il Cliente:

```
Caro Cliente,

Ti invio il programma di automazione workflow.

COSA FA:
- Automatizza compiti ripetitivi
- Legge dati da Excel e li inserisce dove serve
- Ti fa risparmiare ore di lavoro manuale

COME USARE:
1. Scarica il file allegato "workflow-automator.exe"
2. Mettilo in una cartella (es: C:\Automazioni\)
3. Leggi la guida: workflow-automator.exe guida

SUPPORTO:
Se hai problemi, contattami!

Saluti
```

---

**Buon Sviluppo! 🚀**
