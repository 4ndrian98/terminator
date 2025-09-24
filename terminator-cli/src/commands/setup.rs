use anyhow::Result;
use clap::Args;
use colored::*;
use std::path::PathBuf;
use std::process::Command as ProcessCommand;
use tracing::debug;

#[derive(Debug, Args)]
pub struct SetupCommand {
    /// Skip Chrome extension installation
    #[arg(long)]
    skip_chrome: bool,

    /// Skip Visual C++ Redistributables check (Windows only)
    #[arg(long)]
    skip_vcredist: bool,

    /// Skip SDK setup (Node.js, Bun, terminator.js)
    #[arg(long)]
    skip_sdk: bool,

    /// Run Chrome extension installation via MCP automation
    #[arg(long)]
    auto_chrome: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

impl SetupCommand {
    pub async fn execute(&self) -> Result<()> {
        println!("{}", "🚀 Terminator Setup Wizard".bold().cyan());
        println!("{}", "==========================".cyan());
        println!();

        let mut results = Vec::new();

        // Step 1: Check prerequisites
        println!("{}", "📋 Checking prerequisites...".bold());
        results.push(self.check_prerequisites().await);

        // Step 2: VC++ Redistributables (Windows only)
        #[cfg(windows)]
        if !self.skip_vcredist {
            results.push(self.setup_vcredist().await);
        }

        // Step 3: SDK Setup
        if !self.skip_sdk {
            results.push(self.setup_sdks().await);
        }

        // Step 4: Chrome Extension
        if !self.skip_chrome {
            if self.auto_chrome {
                results.push(self.auto_install_chrome_extension().await);
            } else {
                results.push(self.manual_install_chrome_extension().await);
            }
        }

        // Step 5: Verify installation
        results.push(self.verify_installation().await);

        // Print summary
        self.print_summary(&results);

        Ok(())
    }

    async fn check_prerequisites(&self) -> (&'static str, Result<String>) {
        debug!("Checking system prerequisites");

        let mut checks = Vec::new();

        // Check OS
        #[cfg(windows)]
        checks.push(("Windows", true));
        #[cfg(target_os = "macos")]
        checks.push(("macOS", true));
        #[cfg(target_os = "linux")]
        checks.push(("Linux", true));

        // Check Chrome/Chromium
        let chrome_installed = self.check_chrome_installed();
        checks.push(("Chrome/Chromium", chrome_installed));

        // Check Node.js
        let node_installed = ProcessCommand::new("node")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);
        checks.push(("Node.js", node_installed));

        let all_ok = checks.iter().all(|(_, ok)| *ok);
        let summary = checks
            .iter()
            .map(|(name, ok)| format!("{}: {}", name, if *ok { "✓" } else { "✗" }))
            .collect::<Vec<_>>()
            .join(", ");

        if all_ok {
            ("Prerequisites", Ok(summary))
        } else {
            ("Prerequisites", Err(anyhow::anyhow!("Missing: {}", summary)))
        }
    }

    #[cfg(windows)]
    async fn setup_vcredist(&self) -> (&'static str, Result<String>) {
        println!("{}", "📦 Setting up Visual C++ Redistributables...".bold());

        // Check if already installed
        let check = ProcessCommand::new("reg")
            .args(&[
                "query",
                "HKLM\\SOFTWARE\\Microsoft\\VisualStudio\\14.0\\VC\\Runtimes\\x64",
                "/v",
                "Version",
            ])
            .output();

        if check.map(|o| o.status.success()).unwrap_or(false) {
            println!("  {} Already installed", "✓".green());
            return ("VC++ Redistributables", Ok("Already installed".to_string()));
        }

        // Check with winget
        println!("  Checking winget availability...");
        let winget_check = ProcessCommand::new("winget")
            .arg("--version")
            .output();

        if winget_check.map(|o| o.status.success()).unwrap_or(false) {
            println!();
            println!("  📦 Installing via winget...");
            println!("  Please run this command in an elevated terminal:");
            println!();
            println!("    {}", "winget install Microsoft.VCRedist.2015+.x64".bold().yellow());
            println!();
            println!("  Press Enter after installation completes...");
            std::io::stdin().read_line(&mut String::new()).ok();
            ("VC++ Redistributables", Ok("Installed via winget".to_string()))
        } else {
            let url = "https://aka.ms/vs/17/release/vc_redist.x64.exe";
            println!();
            println!("  📥 Please download and install:");
            println!("  {}", url.underline().blue());
            println!();
            println!("  Press Enter to open the download page...");
            std::io::stdin().read_line(&mut String::new()).ok();

            ProcessCommand::new("cmd")
                .args(&["/C", "start", url])
                .spawn()
                .ok();

            println!("  Press Enter after installation completes...");
            std::io::stdin().read_line(&mut String::new()).ok();

            ("VC++ Redistributables", Ok("Manual installation".to_string()))
        }
    }

    #[cfg(not(windows))]
    async fn setup_vcredist(&self) -> (&'static str, Result<String>) {
        ("VC++ Redistributables", Ok("Not needed".to_string()))
    }

    async fn setup_sdks(&self) -> (&'static str, Result<String>) {
        println!("{}", "🛠️  Setting up SDKs...".bold());

        let mut components = Vec::new();

        // Check Node.js
        print!("  Node.js: ");
        match ProcessCommand::new("node").arg("--version").output() {
            Ok(output) if output.status.success() => {
                let version = String::from_utf8_lossy(&output.stdout);
                println!("{} {}", "✓".green(), version.trim());
                components.push("Node.js");
            }
            _ => {
                println!("{} Not installed", "✗".red());
                println!("    Install from: {}", "https://nodejs.org".underline());
            }
        }

        // Check Bun (optional)
        print!("  Bun (optional): ");
        match ProcessCommand::new("bun").arg("--version").output() {
            Ok(output) if output.status.success() => {
                let version = String::from_utf8_lossy(&output.stdout);
                println!("{} {}", "✓".green(), version.trim());
                components.push("Bun");
            }
            _ => {
                println!("{} Not installed", "○".yellow());
                if self.verbose {
                    println!("    Install for better TypeScript support: {}", "https://bun.sh".underline());
                }
            }
        }

        // Pre-install terminator.js
        println!();
        println!("  Pre-caching terminator.js...");
        let cache_dir = dirs::cache_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("terminator")
            .join("mcp-deps");

        if let Err(e) = tokio::fs::create_dir_all(&cache_dir).await {
            println!("    {} Could not create cache directory: {}", "⚠️".yellow(), e);
            return ("SDK Setup", Err(anyhow::anyhow!("Failed to create cache directory: {}", e)));
        }

        // Run npm install in cache directory
        let npm_result = ProcessCommand::new("npm")
            .current_dir(&cache_dir)
            .args(&["install", "terminator.js", "--save"])
            .output();

        match npm_result {
            Ok(output) if output.status.success() => {
                println!("    {} terminator.js cached", "✓".green());
                components.push("terminator.js");
            }
            _ => {
                println!("    {} Could not pre-cache (will install on demand)", "○".yellow());
            }
        }

        let summary = components.join(", ");
        ("SDK Setup", Ok(summary))
    }

    async fn auto_install_chrome_extension(&self) -> (&'static str, Result<String>) {
        println!("{}", "🌐 Installing Chrome Extension (automated)...".bold());

        // Run the existing workflow via MCP
        println!("  Running automated installation workflow...");

        let workflow_path = "terminator/browser-extension/install_chrome_extension_ui.yml";

        let spawn_result = ProcessCommand::new("terminator")
            .args(&["mcp", "run", workflow_path, "--command", "npx -y terminator-mcp-agent"])
            .spawn();

        match spawn_result {
            Ok(mut child) => {
                match child.wait() {
                    Ok(status) if status.success() => {
                        ("Chrome Extension", Ok("Installed via automation".to_string()))
                    }
                    Ok(_) => {
                        ("Chrome Extension", Err(anyhow::anyhow!("Automation failed, try manual install")))
                    }
                    Err(e) => {
                        ("Chrome Extension", Err(anyhow::anyhow!("Installation workflow failed: {}", e)))
                    }
                }
            }
            Err(e) => {
                ("Chrome Extension", Err(anyhow::anyhow!("Failed to run installation workflow: {}", e)))
            }
        }
    }

    async fn manual_install_chrome_extension(&self) -> (&'static str, Result<String>) {
        println!("{}", "🌐 Installing Chrome Extension (manual)...".bold());

        let ext_path = PathBuf::from("terminator/browser-extension");

        if !ext_path.exists() {
            // Try downloading from release
            println!("  Local extension not found, downloading from release...");

            let temp_dir = std::env::temp_dir().join("terminator-bridge");
            if let Err(e) = tokio::fs::create_dir_all(&temp_dir).await {
                return ("Chrome Extension", Err(anyhow::anyhow!("Failed to create temp directory: {}", e)));
            }

            println!("  Downloading extension...");
            let download_url = "https://github.com/mediar-ai/terminator/releases/latest/download/terminator-browser-extension.zip";

            // Use curl or PowerShell to download
            #[cfg(windows)]
            {
                if let Err(e) = ProcessCommand::new("powershell")
                    .args(&[
                        "-Command",
                        &format!("Invoke-WebRequest -Uri '{}' -OutFile '{}'",
                            download_url,
                            temp_dir.join("extension.zip").display())
                    ])
                    .output()
                {
                    return ("Chrome Extension", Err(anyhow::anyhow!("Failed to download extension: {}", e)));
                }
            }

            #[cfg(not(windows))]
            {
                if let Err(e) = ProcessCommand::new("curl")
                    .args(&[
                        "-L",
                        download_url,
                        "-o",
                        temp_dir.join("extension.zip").to_str().unwrap(),
                    ])
                    .output()
                {
                    return ("Chrome Extension", Err(anyhow::anyhow!("Failed to download extension: {}", e)));
                }
            }

            // Extract
            println!("  Extracting extension...");
            #[cfg(windows)]
            {
                if let Err(e) = ProcessCommand::new("powershell")
                    .args(&[
                        "-Command",
                        &format!("Expand-Archive -Path '{}' -DestinationPath '{}' -Force",
                            temp_dir.join("extension.zip").display(),
                            temp_dir.display())
                    ])
                    .output()
                {
                    return ("Chrome Extension", Err(anyhow::anyhow!("Failed to extract extension: {}", e)));
                }
            }

            println!();
            self.show_chrome_install_instructions(&temp_dir);
        } else {
            match std::fs::canonicalize(&ext_path) {
                Ok(abs_path) => self.show_chrome_install_instructions(&abs_path),
                Err(e) => {
                    return ("Chrome Extension", Err(anyhow::anyhow!("Failed to get extension path: {}", e)));
                }
            }
        }

        ("Chrome Extension", Ok("Manual instructions provided".to_string()))
    }

    fn show_chrome_install_instructions(&self, path: &std::path::Path) {
        println!("  📂 Extension location: {}", path.display().to_string().green());
        println!();
        println!("  To install:");
        println!("  1. Open Chrome and go to: {}", "chrome://extensions".bold());
        println!("  2. Enable {} (top right)", "Developer mode".bold());
        println!("  3. Click {} (top left)", "Load unpacked".bold());
        println!("  4. Select the folder above");
        println!();

        // Copy to clipboard on Windows
        #[cfg(windows)]
        {
            let _ = ProcessCommand::new("cmd")
                .args(&["/C", &format!("echo {}| clip", path.display())])
                .output();
            println!("  📋 Path copied to clipboard!");
        }

        println!("  Press Enter to open Chrome extensions page...");
        std::io::stdin().read_line(&mut String::new()).ok();

        // Open extensions page
        #[cfg(windows)]
        ProcessCommand::new("cmd")
            .args(&["/C", "start", "chrome://extensions"])
            .spawn()
            .ok();

        #[cfg(target_os = "macos")]
        ProcessCommand::new("open")
            .arg("chrome://extensions")
            .spawn()
            .ok();

        println!();
        println!("  Press Enter when done...");
        std::io::stdin().read_line(&mut String::new()).ok();
    }

    async fn verify_installation(&self) -> (&'static str, Result<String>) {
        println!("{}", "✅ Verifying installation...".bold());

        // Test MCP agent
        print!("  MCP Agent: ");
        let mcp_test = ProcessCommand::new("npx")
            .args(&["-y", "terminator-mcp-agent", "--version"])
            .output();

        match mcp_test {
            Ok(output) if output.status.success() => {
                println!("{} Ready", "✓".green());
                ("Verification", Ok("All systems ready".to_string()))
            }
            _ => {
                println!("{} Will install on first use", "○".yellow());
                ("Verification", Ok("Ready (MCP will install on demand)".to_string()))
            }
        }
    }

    fn check_chrome_installed(&self) -> bool {
        #[cfg(windows)]
        let chrome_paths = vec![
            "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe",
            "C:\\Program Files (x86)\\Google\\Chrome\\Application\\chrome.exe",
        ];

        #[cfg(target_os = "macos")]
        let chrome_paths = vec![
            "/Applications/Google Chrome.app",
            "/Applications/Chromium.app",
        ];

        #[cfg(target_os = "linux")]
        let chrome_paths = vec![
            "/usr/bin/google-chrome",
            "/usr/bin/chromium",
        ];

        chrome_paths.iter().any(|path| std::path::Path::new(path).exists())
    }

    fn print_summary(&self, results: &[(&'static str, Result<String>)]) {
        println!();
        println!("{}", "📊 Setup Summary".bold().green());
        println!("{}", "================".green());

        let mut has_errors = false;

        for (step, result) in results {
            match result {
                Ok(msg) => println!("  {} {}: {}", "✅".green(), step.bold(), msg),
                Err(err) => {
                    println!("  {} {}: {}", "❌".red(), step.bold(), err);
                    has_errors = true;
                }
            }
        }

        println!();
        if has_errors {
            println!("{}", "⚠️  Some steps need attention. See above for details.".yellow());
        } else {
            println!("{}", "🎉 Setup complete!".bold().green());
            println!();
            println!("Next steps:");
            println!("  1. Test with: {}", "terminator mcp chat --command \"npx -y terminator-mcp-agent\"".cyan());
            println!("  2. Run examples: {}", "terminator mcp run examples/notepad.py".cyan());
        }
    }
}