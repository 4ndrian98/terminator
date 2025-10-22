# 📖 Workflow Automator - Guida per il Cliente

## 🎯 Cosa fa questo programma?

**Workflow Automator** ti permette di automatizzare attività ripetitive sul computer:
- Compilazione di form
- Inserimento dati da Excel in programmi
- Operazioni ripetitive che fai manualmente

**NESSUNA competenza tecnica richiesta!**

---

## 🚀 Come Iniziare

### ✅ Requisiti
- Windows 10 o 11
- Nient'altro! Il programma funziona subito

### 📥 Installazione
1. Copia il file `workflow-automator.exe` sul tuo computer
2. Mettilo in una cartella comoda (es: `C:\Automazioni\`)
3. Fatto! Non serve installare nulla

---

## 📝 Guida Passo-Passo

### STEP 1: Registra il Workflow (da fare UNA VOLTA)

Apri il **Prompt dei Comandi** (CMD) o **PowerShell**:

```cmd
cd C:\Automazioni
workflow-automator.exe registra --nome "mio_lavoro"
```

**Cosa succede:**
1. Il programma inizia a registrare
2. **TU esegui manualmente** le azioni che vuoi automatizzare
3. Esempio: apri un programma, compila i campi, premi OK
4. Quando hai finito, premi **Ctrl+C**
5. Il workflow viene salvato come `mio_lavoro.json`

💡 **Consiglio:** Fai azioni semplici e chiare. Il programma copia esattamente quello che fai!

---

### STEP 2: Esegui il Workflow

Ora puoi ripetere il workflow salvato:

```cmd
workflow-automator.exe esegui --workflow mio_lavoro.json
```

Il programma ripete AUTOMATICAMENTE tutte le azioni che hai registrato!

---

### STEP 3: Automatizza con Excel (POTENTE!)

Hai 100 righe di dati in Excel da inserire? Facile!

**Prepara Excel:**
```
     A          B         C
1    Nome       Email     Telefono
2    Mario      m@x.it    123456
3    Luigi      l@x.it    789012
...  (altre 98 righe)
```

**Esegui:**
```cmd
workflow-automator.exe excel --excel dati.xlsx --workflow mio_lavoro.json
```

Il programma:
1. Legge la prima riga di Excel
2. Esegue il workflow usando quei dati
3. Passa alla riga successiva
4. Ripete per TUTTE le righe automaticamente!

---

## 💼 Esempi Pratici

### Esempio 1: Compilare Form Ripetitivo

**Scenario:** Devi compilare un form aziendale 50 volte con dati diversi.

**Soluzione:**
1. Registra: compili il form UNA volta mentre registri
2. Prepara Excel con i 50 dati
3. Esegui con Excel: il programma compila tutto automaticamente

**Tempo risparmiato:** Da 2 ore → 5 minuti!

---

### Esempio 2: Inserimento Dati in Gestionale

**Scenario:** Devi inserire 200 clienti da Excel nel gestionale aziendale.

**Soluzione:**
1. Registra: inserisci 1 cliente mentre registri
2. Prepara Excel con i 200 clienti
3. Esegui: il programma inserisce tutti i 200 clienti

---

## ❓ Domande Frequenti

### **Q: Serve installare Python o altri programmi?**
A: NO! Il file .exe funziona subito, senza dipendenze.

### **Q: Funziona con qualsiasi programma Windows?**
A: Sì! Funziona con qualsiasi applicazione (gestionali, form web, Excel, ecc.)

### **Q: Posso modificare un workflow registrato?**
A: Non direttamente. Se vuoi cambiare, registra un nuovo workflow.

### **Q: Il programma può danneggiare il mio PC?**
A: No! Fa solo quello che registri. Non ha accesso a file di sistema.

### **Q: Posso distribuirlo ad altri colleghi?**
A: Sì, copia semplicemente il file .exe

### **Q: Cosa succede se sbaglio durante la registrazione?**
A: Ricomincia! Registra un nuovo workflow con nome diverso.

---

## 🆘 Supporto

**Problemi?** Contatta:
- Email: supporto@tuaazienda.com
- Tel: 123-456-789

---

## 📋 Comandi Rapidi

```cmd
# Mostra guida
workflow-automator.exe guida

# Registra workflow
workflow-automator.exe registra --nome "nome_workflow"

# Esegui workflow
workflow-automator.exe esegui --workflow nome_workflow.json

# Automatizza da Excel
workflow-automator.exe excel --excel dati.xlsx --workflow nome_workflow.json
```

---

## ⚡ Consigli Pro

1. **Nomi Chiari:** Usa nomi descrittivi per i workflow (es: "inserimento_clienti")
2. **Test Prima:** Prova il workflow su 2-3 righe Excel prima di fare tutto
3. **Backup:** Salva i file .json dei workflow, sono preziosi!
4. **Velocità:** Il programma va veloce, ma puoi rallentarlo se serve
5. **Pause:** Se il programma va troppo veloce, chiudilo e contatta supporto

---

## 🎓 Tutorial Video

[Link al tutorial video] - Coming soon!

---

**Buon Lavoro! 🚀**
