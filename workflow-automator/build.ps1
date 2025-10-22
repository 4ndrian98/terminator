# Script di build per creare .exe distribuibile ai clienti
# 
# Questo script compila l'applicazione in un singolo .exe ottimizzato
# che può essere distribuito ai clienti SENZA dipendenze

Write-Host "🔨 BUILD WORKFLOW AUTOMATOR" -ForegroundColor Cyan
Write-Host "   Compilazione per distribuzione ai clienti" -ForegroundColor Gray
Write-Host ""

# Check Rust installato
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Rust non trovato! Installa da: https://rustup.rs/" -ForegroundColor Red
    exit 1
}

Write-Host "✅ Rust trovato" -ForegroundColor Green

# Build release (ottimizzato)
Write-Host ""
Write-Host "📦 Compilazione in corso (può richiedere alcuni minuti)..." -ForegroundColor Yellow
Write-Host ""

cargo build --release

if ($LASTEXITCODE -eq 0) {
    Write-Host ""
    Write-Host "✅ COMPILAZIONE COMPLETATA!" -ForegroundColor Green
    Write-Host ""
    Write-Host "📁 File creato in:" -ForegroundColor Cyan
    Write-Host "   target\release\workflow-automator.exe" -ForegroundColor White
    Write-Host ""
    Write-Host "📋 Dimensione file:" -ForegroundColor Cyan
    $size = (Get-Item "target\release\workflow-automator.exe").Length / 1MB
    Write-Host "   $($size.ToString('F2')) MB" -ForegroundColor White
    Write-Host ""
    Write-Host "🚀 DISTRIBUISCI AL CLIENTE:" -ForegroundColor Green
    Write-Host "   1. Copia 'workflow-automator.exe' al cliente" -ForegroundColor White
    Write-Host "   2. Il cliente può usarlo subito, NESSUNA dipendenza richiesta" -ForegroundColor White
    Write-Host "   3. Funziona su qualsiasi Windows (10/11)" -ForegroundColor White
    Write-Host ""
    Write-Host "📖 GUIDA PER IL CLIENTE:" -ForegroundColor Cyan
    Write-Host "   workflow-automator.exe guida" -ForegroundColor White
    Write-Host ""
} else {
    Write-Host ""
    Write-Host "❌ Errore durante la compilazione" -ForegroundColor Red
    Write-Host "   Controlla gli errori sopra" -ForegroundColor Yellow
    exit 1
}
