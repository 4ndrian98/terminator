const { Desktop } = require("terminator.js");

(async () => {
  const desktop = new Desktop();

  // 1️⃣ Apri Chrome e vai su Google
  await desktop.openApplication("chrome", ["https://www.google.com"]);

  // 2️⃣ Attendi che la finestra di Google sia visibile
  await desktop.waitForWindow("Google");

  // 3️⃣ Trova il campo di ricerca e digita "OpenAI"
  await desktop.locator("role:Text").typeText("OpenAI{Enter}");

  // 4️⃣ Attendi qualche secondo per il caricamento dei risultati
  await desktop.sleep(3000);

  // 5️⃣ Fai uno screenshot
  const image = await desktop.screenshot();
  await image.save("screenshot.png");

  console.log("✅ Screenshot salvato come screenshot.png!");
})();
