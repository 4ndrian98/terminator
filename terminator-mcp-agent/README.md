# Terminator MCP Agent

# Build the MCP agent:

```
cargo build -p terminator-mcp-agent --release
```

###  **Configure Cursor:**
    You need to tell Cursor how to run this agent. Create a file named `mcp.json` in your Cursor configuration directory (`~/.cursor` on macOS/Linux, `%USERPROFILE%\.cursor` on Windows).

    **macOS / Linux:**

    ```bash
    # Run this command inside the terminator/mcp directory
    MCP_PATH="$(pwd)/target/release/terminator-mcp-agent.exe"
    JSON_CONTENT=$(cat <<EOF
    {
      "mcpServers": {
        "terminator-mcp-agent": {
          "command": "$MCP_PATH",
          "args": []
        }
      }
    }
    EOF
    )
    echo "--- Copy the JSON below and save it as mcp.json in your ~/.cursor directory ---"
    echo "$JSON_CONTENT"
    echo "------------------------------------------------------------------------------------------"
    mkdir -p "$HOME/.cursor"
    ```

    **Windows (PowerShell):**

    You can use this PowerShell command **while inside the `mcp` directory** to generate the correct JSON content:

    ```powershell
    # Run this command inside the terminator/mcp directory
    $mcpPath = ($pwd).Path.Replace('\', '\\') + '\\target\\release\\terminator-mcp-agent.exe'
    $jsonContent = @"
    {
      "mcpServers": {
        "terminator-mcp-agent": {
          "command": "$mcpPath",
          "args": []
        }
      }
    }
    "@
    Write-Host "--- Copy the JSON below and save it as mcp.json in your %USERPROFILE%\.cursor directory ---"
    Write-Host $jsonContent
    Write-Host "------------------------------------------------------------------------------------------"
    # Optional: Try to automatically open the directory
    Start-Process "$env:USERPROFILE\.cursor" -ErrorAction SilentlyContinue
    ```

    *   Run the appropriate command for your OS (PowerShell for Windows, Bash for macOS/Linux).
    *   Copy the JSON output (starting with `{` and ending with `}`).
    *   Create the `%USERPROFILE%\.cursor` (Windows) or `~/.cursor` (macOS/Linux) directory if it doesn't exist.
    *   Create a new file named `mcp.json` inside that directory.
    *   Paste the copied JSON content into `mcp.json` and save it.


###  **Configure Claude Desktop app:**

open the claude app and search for developer options then MCP. when you click on configure MCP button it'll open a json file where you have to edit a `claude_desktop_config.json` file

`
{
  "mcpServers": {
    "terminator-mcp-agent": {
      "command": "path_to_terminator-mcp-agent.exe",
      "args": []
    }
  }
}
`
remember to replace `path_to_terminator` exe with actual path of terminator-mcp-agent binary

