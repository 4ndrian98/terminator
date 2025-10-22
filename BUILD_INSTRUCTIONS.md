# 🏗️ Istruzioni per Build e Sviluppo

## 📋 Prerequisiti

### Sistema Operativo
- Windows 10/11 (per build .exe)
- macOS 10.15+ (per build .app/.dmg)
- Linux (per build AppImage/deb)

### Software Richiesto

#### 1. Rust
```bash
# Installa Rust (se non già installato)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verifica installazione
cargo --version
rustc --version
```

#### 2. Node.js
- Node.js 18+ e npm
```bash
# Verifica installazione
node --version
npm --version
```

#### 3. Dipendenze Sistema (Windows)
- **Microsoft Visual C++ Build Tools**
  - Scarica da: https://visualstudio.microsoft.com/visual-cpp-build-tools/
  - Oppure installa Visual Studio con "Desktop development with C++"

- **WebView2** (già incluso in Windows 11, per Windows 10):
  - Scarica da: https://developer.microsoft.com/microsoft-edge/webview2/

#### 4. Dipendenze Sistema (macOS)
```bash
xcode-select --install
```

#### 5. Dipendenze Sistema (Linux - Ubuntu/Debian)
```bash
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

---

## 🚀 Installazione Progetto

### 1. Clone Repository
```bash
git clone <repo-url>
cd workflow-automator-gui
```

### 2. Installa Dipendenze Frontend
```bash
cd frontend
npm install
cd ..
```

### 3. Installa Dipendenze Tauri (automatico al primo build)
Le dipendenze Rust vengono installate automaticamente durante il primo build.

---

## 💻 Sviluppo

### Modalità Development

```bash
# Dalla root del progetto
npm run tauri:dev
```

Questo comando:
1. Avvia il dev server Vite (frontend React) su http://localhost:5173
2. Compila e avvia l'applicazione Tauri
3. Hot reload abilitato per frontend (modifiche React si aggiornano automaticamente)
4. Backend Rust richiede rebuild per modifiche

### Solo Frontend (senza Tauri)
```bash
cd frontend
npm run dev
```

Utile per sviluppo rapido UI, ma i comandi Tauri non funzioneranno.

---

## 📦 Build per Produzione

### Windows (.exe)

```bash
# Dalla root del progetto
npm run tauri:build
```

**Output:**
- `/app/src-tauri/target/release/workflow-automator-gui.exe` - Eseguibile standalone
- `/app/src-tauri/target/release/bundle/msi/` - Installer MSI (opzionale)

**Dimensione file:** ~15-30 MB (ottimizzato)

### macOS (.app / .dmg)

```bash
npm run tauri:build
```

**Output:**
- `/app/src-tauri/target/release/bundle/macos/` - App bundle
- `/app/src-tauri/target/release/bundle/dmg/` - DMG installer

### Linux (AppImage / deb)

```bash
npm run tauri:build
```

**Output:**
- `/app/src-tauri/target/release/bundle/appimage/` - AppImage
- `/app/src-tauri/target/release/bundle/deb/` - Package deb

---

## 🎯 Test Build Locale

Dopo il build, testa l'eseguibile:

### Windows
```bash
./src-tauri/target/release/workflow-automator-gui.exe
```

### macOS
```bash
open ./src-tauri/target/release/bundle/macos/Workflow\ Automator.app
```

### Linux
```bash
./src-tauri/target/release/workflow-automator-gui
```

---

## 🔧 Configurazione Build

### Personalizzazione Icona

Sostituisci le icone in `/app/src-tauri/icons/` con le tue:
- `32x32.png`
- `128x128.png`
- `128x128@2x.png`
- `icon.icns` (macOS)
- `icon.ico` (Windows)

### Personalizzazione Nome App

Modifica `/app/src-tauri/tauri.conf.json`:
```json
{
  "package": {
    "productName": "Il Mio Nome App",
    "version": "1.0.0"
  }
}
```

### Ottimizzazione Dimensione

Il file `/app/src-tauri/Cargo.toml` è già ottimizzato:
```toml
[profile.release]
panic = "abort"
codegen-units = 1
lto = true
opt-level = "s"  # Ottimizza per dimensione
strip = true     # Rimuove simboli debug
```

---

## 🐛 Troubleshooting

### Errore: "cargo not found"
```bash
# Ricarica environment Rust
source $HOME/.cargo/env  # Linux/macOS
# oppure riavvia il terminale
```

### Errore: "WebView2 not found" (Windows)
Installa WebView2 Runtime: https://developer.microsoft.com/microsoft-edge/webview2/

### Errore: Build fallisce con errori Rust
```bash
# Pulisci cache e rebuilda
cd src-tauri
cargo clean
cargo build
```

### Errore: Frontend non si connette a Tauri
Verifica che il frontend usi `@tauri-apps/api`:
```javascript
import { invoke } from '@tauri-apps/api/tauri';
```

---

## 📝 Note Importanti

### Build Cross-Platform
- Puoi buildare SOLO per il sistema operativo su cui stai compilando
- Per Windows .exe → buildi su Windows
- Per macOS .app → buildi su macOS
- Per Linux → buildi su Linux

### Firma Digitale (Opzionale ma Raccomandato)

#### Windows
```bash
# Usa signtool.exe (Windows SDK)
signtool sign /f certificato.pfx /p password workflow-automator-gui.exe
```

#### macOS
```bash
# Usa codesign
codesign --force --deep --sign "Developer ID" Workflow\ Automator.app
```

### CI/CD Build
Per build automatizzati su GitHub Actions, vedi esempio in `.github/workflows/build.yml` (da creare).

---

## 🎉 Distribuzione

### Metodo 1: File Standalone
Distribuisci direttamente il file `.exe` (Windows) o `.app` (macOS).
- Vantaggi: Immediato, nessuna installazione
- Svantaggi: Nessun auto-update

### Metodo 2: Installer
Usa gli installer generati (MSI per Windows, DMG per macOS, DEB per Linux).
- Vantaggi: Aspetto più professionale
- Svantaggi: Richiede permessi admin

### Metodo 3: Microsoft Store / Mac App Store
Pubblica negli store ufficiali per massima distribuzione.

---

## 📊 Checklist Pre-Release

- [ ] Testa tutte le funzionalità principali
- [ ] Verifica performance (CPU, memoria)
- [ ] Test su sistema pulito (senza dev tools)
- [ ] Verifica icone e branding
- [ ] Aggiorna version number in `tauri.conf.json` e `Cargo.toml`
- [ ] Crea documentazione utente (README, guida)
- [ ] (Opzionale) Firma digitale eseguibile
- [ ] (Opzionale) Crea installer

---

**Per supporto: [email/discord/github]**
