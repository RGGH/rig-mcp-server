[![Rust](https://github.com/RGGH/rig-mcp-server/actions/workflows/rust.yml/badge.svg)](https://github.com/RGGH/rig-mcp-server/actions/workflows/rust.yml)
# 🔍 Rust MCP Server + Inspector Example: SSE Transport with an Add Tool

credit : https://dev.to/joshmo_dev/using-model-context-protocol-with-rig-m7o

This project demonstrates how to set up an MCP (Model Context Protocol) server and client using Server-Sent Events (SSE) for communication. It includes a simple tool that adds two numbers and integrates with the [RIG](https://github.com/modelcontext/rig) agent for LLM prompting.

---

## 🚀 Getting Started

Clone this repo and run with:

```bash
cargo run
```

In a separate terminal start the MCP Inspector with:

```bash
npx @modelcontextprotocol/inspector sse http://127.0.0.1:3001/sse
```

You'll see output like:

```
Starting MCP inspector...
Proxy server listening on port 3000
New SSE connection
Query parameters: { transportType: 'sse', url: 'http://localhost:3001/sse' }
SSE transport: url=http://localhost:3001/sse, headers=
Connected to SSE transport
Connected MCP client to backing server transport
Created web app transport
Set up MCP proxy
🔍 MCP Inspector is up and running at http://localhost:5173 🚀
```

You can now view the web interface at [http://localhost:5173](http://localhost:5173)

---

## 🛠️ Features

- ✅ Sets up a custom MCP server using `ServerSseTransport`
- ✅ Connects a MCP client to the server
- ✅ Registers a custom tool: `AddTool`, which adds two numbers
- ✅ Lists registered tools via MCP
- ✅ Integrates with RIG and prompts an LLM agent using the tool

---

## 🧠 Code Overview

```rust
#[tool(
    name = "Add",
    description = "Adds two numbers together.",
    params(a = "The first number to add", b = "The second number to add")
)]
async fn add_tool(a: f64, b: f64) -> Result<ToolResponseContent> {
    Ok(tool_text_content!((a + b).to_string()))
}
```

This defines the `Add` tool that is registered in the MCP server.

The main function sets up:
- Tracing
- The MCP server and transport (SSE)
- A MCP client that initializes and lists available tools
- A RIG agent with OpenAI backend, which uses the MCP tool

The agent then runs a prompt:
```rust
let response = agent.prompt("Add 10 + 10").await;
```

---

## 🧪 Sample Output

When run successfully, you'll see logs like:

```
Initialized: Ok(...)
Tools: Ok([...])
Building RIG agent
Prompting RIG agent
Agent response: Some("20")
```

---

## 🧰 Tech Stack

- 🦀 Rust with [tokio](https://tokio.rs/)
- 📡 SSE transport from `mcp_core`
- 🔧 MCP server/client architecture
- 🤖 RIG agent with OpenAI model
- 🌐 MCP Inspector web interface

---

## 📦 Dependencies

Make sure you have these in your `Cargo.toml`:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
anyhow = "1"
serde_json = "1"
mcp_core = "..."
mcp_core_macros = "..."
rig = "..."
```

> Replace `...` with the appropriate versions based on your environment.

---

## 📍 Inspector GUI

Visit [http://localhost:5173](http://localhost:5173) to view and interact with the MCP Inspector UI.

---

## ✅ Test Log Sample

```
Query parameters: { transportType: 'sse', url: 'http://localhost:3001/sse' }
Connected to SSE transport
Connected MCP client to backing server transport
Set up MCP proxy
Received message for sessionId cdd4a8be-57e2-44e3-9b81-3df300e86f22
```
![Screenshot from 2025-04-07 23-22-21](https://github.com/user-attachments/assets/741f033f-7a9e-4d03-bff8-b1547b49fd50)

---

## 📬 Questions or Feedback?

Feel free to open an issue or start a discussion!
```

