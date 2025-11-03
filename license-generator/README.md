# 🔑 Generatore Chiavi Licenza - Workflow Automator

Questo tool genera chiavi di licenza per l'applicazione Workflow Automator.

## 🚀 Come Usare

### 1. Compilare il Generatore

```bash
cd license-generator
cargo build --release
```

Il file eseguibile sarà disponibile in: `target/release/license-generator.exe` (Windows) o `target/release/license-generator` (Linux/Mac)

### 2. Eseguire il Generatore

```bash
# Windows
.\target\release\license-generator.exe

# Linux/Mac
./target/release/license-generator
```

### 3. Generare Chiavi

Il programma è interattivo e ti guiderà attraverso i seguenti passaggi:

1. **Seleziona tipo di licenza:**
   - Mensile (30 giorni)
   - Trimestrale (90 giorni)
   - Annuale (365 giorni)

2. **Scegli data di scadenza:**
   - Automatica (da oggi + durata licenza)
   - Manuale (inserisci data specifica in formato YYYYMMDD)

3. **Ottieni la chiave generata**

### 4. Formato Chiave

Le chiavi generate hanno il seguente formato:

```
WA-<TIPO>-<YYYYMMDD>-<HASH>
```

Esempio:
- `WA-M-20250830-ABC12345` - Licenza mensile che scade il 30/08/2025
- `WA-T-20250930-DEF67890` - Licenza trimestrale che scade il 30/09/2025
- `WA-A-20260731-GHI11111` - Licenza annuale che scade il 31/07/2026

Dove:
- `WA` = Workflow Automator (prefisso fisso)
- `M/T/A` = Tipo (Mensile/Trimestrale/Annuale)
- `YYYYMMDD` = Data di scadenza
- `HASH` = Codice di verifica (8 caratteri)

## 🔒 Sicurezza

**IMPORTANTE:**

1. Il file `license-generator` **NON deve essere distribuito ai clienti**
2. Distribuisci solo il file `.exe` dell'applicazione principale
3. Il `SECRET_KEY` nei file `license.rs` e `main.rs` del generatore **DEVE essere lo stesso**
4. Cambia il `SECRET_KEY` in produzione e tienilo segreto!

## 📝 Gestione Clienti

Consiglio di tenere un registro dei clienti e delle chiavi generate:

```
Cliente              | Tipo        | Data Acquisto | Data Scadenza | Chiave
----------------------------------------------------------------------------------
Mario Rossi         | Mensile     | 01/01/2025    | 31/01/2025    | WA-M-20250131-...
Luigi Bianchi       | Annuale     | 15/01/2025    | 15/01/2026    | WA-A-20260115-...
```

Puoi usare un foglio Excel o un database per tracciare le licenze.

## 🔄 Rinnovi

Per rinnovare una licenza scaduta:

1. Genera una nuova chiave con data di scadenza futura
2. Invia la nuova chiave al cliente
3. Il cliente la inserisce nella sezione "Gestione Licenza" dell'app

## 💡 Esempi d'Uso

### Licenza Mensile (30 giorni da oggi)

```
Scegli il tipo di licenza: 1
Opzioni data scadenza: 1
```

### Licenza Annuale con data specifica

```
Scegli il tipo di licenza: 3
Opzioni data scadenza: 2
Inserisci data scadenza: 20251231
```

Risultato: `WA-A-20251231-XXXXXXXX`

## 🛠️ Modifica Secret Key

**Prima della distribuzione in produzione**, modifica il `SECRET_KEY`:

1. Apri `/app/src-tauri/src/license.rs`
2. Cambia la costante `SECRET_KEY`
3. Apri `/app/license-generator/src/main.rs`
4. Cambia la stessa costante `SECRET_KEY` con lo stesso valore
5. Ricompila entrambi:
   ```bash
   # App principale
   cd /app/src-tauri
   cargo build --release
   
   # Generatore
   cd /app/license-generator
   cargo build --release
   ```

⚠️ **Attenzione:** Se cambi il secret key dopo aver generato delle chiavi, quelle chiavi non funzioneranno più!

## 📞 Supporto

Per domande o problemi, contatta lo sviluppatore.

---

**Nota:** Questo tool è solo per uso interno. Non distribuirlo ai clienti!
