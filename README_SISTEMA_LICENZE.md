# 🤖 Workflow Automator - Sistema Completo con Licenze

Applicazione desktop per automatizzare workflow ripetitivi su Windows, con sistema di licenze integrato.

## 🎯 Caratteristiche

- ✅ **Registrazione Workflow**: Registra azioni desktop (click, tastiera, app switching)
- ✅ **Esecuzione Automatica**: Ripete le azioni registrate automaticamente
- ✅ **Sistema Licenze**: Gestione abbonamenti mensili/trimestrali/annuali
- ✅ **GUI User-Friendly**: Interfaccia grafica moderna e intuitiva
- ✅ **Zero Competenze Tecniche**: Progettata per utenti non tecnici
- ✅ **Multi-Ripetizione**: Esegui workflow 1-100 volte
- ✅ **Controllo Velocità**: Rallenta o velocizza l'esecuzione

## 📦 Struttura Progetto

```
/app/
├── src-tauri/              # Backend Rust (Tauri)
│   ├── src/
│   │   ├── main.rs         # Applicazione principale
│   │   └── license.rs      # Sistema gestione licenze
│   └── Cargo.toml
│
├── frontend/               # Frontend React
│   ├── src/
│   │   ├── pages/          # Dashboard, Record, Execute, License
│   │   ├── components/     # Componenti riusabili
│   │   ├── services/       # API Tauri
│   │   └── contexts/       # State management
│   └── package.json
│
├── license-generator/      # Tool generazione chiavi
│   ├── src/main.rs
│   └── README.md
│
├── GUIDA_UTENTE.md        # Documentazione utente finale
└── GUIDA_CLIENTE.md       # Guida originale (workflow-automator)
```

## 🚀 Setup Sviluppo

### Prerequisiti

- **Rust**: v1.70+  
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
  
- **Node.js**: v18+
  ```bash
  # Installa Node.js da nodejs.org
  ```

- **Yarn**: Package manager
  ```bash
  npm install -g yarn
  ```

- **Tauri CLI**:
  ```bash
  cargo install tauri-cli
  ```

### Installazione

```bash
# 1. Clona repository
git clone <repo-url>
cd app

# 2. Installa dipendenze frontend
cd frontend
yarn install

# 3. Build frontend
yarn build

# 4. Installa dipendenze Rust (automatico con cargo)
cd ../src-tauri
```

## 🔨 Build Applicazione

### Build Development

```bash
# Dalla root del progetto (/app)
cargo tauri dev
```

### Build Production

```bash
# Dalla root del progetto (/app)
cargo tauri build
```

L'eseguibile `.exe` sarà in: `/app/src-tauri/target/release/workflow-automator-gui.exe`

## 🔑 Sistema Licenze

### Per Sviluppatori

#### 1. Configurare Secret Key

**IMPORTANTE**: Prima della distribuzione, cambia il SECRET_KEY!

```rust
// In /app/src-tauri/src/license.rs
const SECRET_KEY: &str = "TUO_SECRET_KEY_SUPER_SICURO";

// In /app/license-generator/src/main.rs  
const SECRET_KEY: &str = "TUO_SECRET_KEY_SUPER_SICURO";  // STESSO VALORE!
```

#### 2. Compilare Generatore Chiavi

```bash
cd license-generator
cargo build --release
```

Eseguibile in: `license-generator/target/release/license-generator.exe`

#### 3. Generare Chiavi Licenza

```bash
# Windows
.\target\release\license-generator.exe

# Linux/Mac
./target/release/license-generator
```

Segui le istruzioni interattive:
1. Scegli tipo licenza (Mensile/Trimestrale/Annuale)
2. Scegli data scadenza (automatica o manuale)
3. Ottieni chiave formato: `WA-M-20250830-ABC12345`

#### 4. Distribuire Chiavi ai Clienti

- Invia la chiave via email
- Cliente la inserisce al primo avvio dell'app
- La chiave attiva l'applicazione fino alla data di scadenza

### Per Clienti

Vedi **GUIDA_UTENTE.md** per istruzioni complete.

## 📋 Formato Chiave Licenza

```
WA-<TIPO>-<YYYYMMDD>-<HASH>
```

Dove:
- `WA` = Workflow Automator (prefisso)
- `<TIPO>` = `M` (Mensile), `T` (Trimestrale), `A` (Annuale)
- `<YYYYMMDD>` = Data di scadenza (es: 20250830)
- `<HASH>` = Codice verifica SHA256 (8 caratteri)

**Esempi**:
- `WA-M-20250830-A1B2C3D4` - Mensile, scade 30 Agosto 2025
- `WA-T-20251130-E5F6G7H8` - Trimestrale, scade 30 Novembre 2025
- `WA-A-20260731-I9J0K1L2` - Annuale, scade 31 Luglio 2026

## 🛡️ Sicurezza

1. **Secret Key**: Cambia il `SECRET_KEY` prima del deploy e non condividerlo mai
2. **Generatore**: NON distribuire il `license-generator` ai clienti
3. **Validazione**: Offline, nessun server necessario
4. **Hash**: SHA256 per verifica integrità chiave

## 📖 Documentazione

- **GUIDA_UTENTE.md**: Manuale completo per utenti finali (italiano)
- **GUIDA_CLIENTE.md**: Guida originale CLI (italiano)
- **license-generator/README.md**: Guida generatore chiavi

## 🧪 Testing

### Test Backend (Rust)

```bash
cd src-tauri
cargo test
```

### Test Frontend (React)

```bash
cd frontend
yarn test  # Se configurato
```

### Test Manuale Licenza

```bash
# Genera una chiave di test
cd license-generator
cargo run

# Usa la chiave generata nell'app per testare
```

## 📦 Distribuzione

### Per Clienti Finali

Distribuisci solo:
1. `workflow-automator-gui.exe` (dall'app build)
2. `GUIDA_UTENTE.md`

**NON distribuire**:
- Codice sorgente
- `license-generator`
- Secret keys

### Packaging

Opzioni:
1. **Installer** (raccomandato): Usa Inno Setup o NSIS
2. **Zip**: Comprimi .exe + GUIDA in un archivio
3. **MSI**: Usa WiX Toolset per installer Windows

## 🔄 Workflow Aggiornamenti

1. Incrementa versione in `/app/src-tauri/Cargo.toml`
2. Build nuova versione: `cargo tauri build`
3. Distribuisci nuovo .exe
4. Clienti installano sopra versione vecchia
5. Licenze esistenti continuano a funzionare

## 💰 Modello Business

### Prezzi Suggeriti (Esempio)

- **Mensile**: €29/mese
- **Trimestrale**: €69/trimestre (risparmio 20%)
- **Annuale**: €199/anno (risparmio 43%)

### Gestione Rinnovi

Tieni un database/foglio Excel:

```
Cliente      | Email            | Tipo      | Scadenza   | Chiave
----------------------------------------------------------------
Mario Rossi  | mario@email.com  | Annuale   | 2026-01-15 | WA-A-...
```

Invia reminder 7 giorni prima della scadenza.

## 🐛 Troubleshooting

### Build Errors

**Error**: `cargo: not found`
- **Soluzione**: Installa Rust da rustup.rs

**Error**: `yarn: not found`
- **Soluzione**: Installa Node.js e poi `npm install -g yarn`

**Error**: Frontend build fails
- **Soluzione**: `cd frontend && yarn install && yarn build`

### Runtime Errors

**Error**: "Nessuna licenza trovata"
- **Soluzione**: Inserisci chiave di licenza valida

**Error**: "Chiave non valida"
- **Soluzione**: Verifica che SECRET_KEY sia uguale in app e generatore

## 🤝 Supporto

Per supporto tecnico o domande:
- Email: [tuo-email@esempio.com]
- Issues: GitHub Issues (se pubblico)

## 📄 Licenza

[Scegli la tua licenza - es: MIT, Proprietaria, ecc.]

## 🙏 Credits

- **Tauri**: Framework per app desktop
- **React**: Libreria UI
- **Terminator**: Engine automazione workflow
- **TailwindCSS**: Styling

---

**Versione**: 1.0.0  
**Ultimo Aggiornamento**: Gennaio 2025  
**Autore**: [Tuo Nome/Azienda]

🚀 **Happy Automating!**
