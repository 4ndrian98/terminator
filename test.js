import { Desktop } from "terminator.js";

const main = async () => {
  const desktop = new Desktop();
  await desktop.openApplication("notepad");
  await desktop.locator("name:Edit").typeText("Ciao dal Terminator!");
};

main();