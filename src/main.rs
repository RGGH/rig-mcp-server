use anyhow::Result;
use mcp_core::tool_text_content;
use mcp_core::types::ClientCapabilities;
use mcp_core::types::Implementation;
use mcp_core::types::ToolResponseContent;
use mcp_core::{
    client::ClientBuilder, server::Server, transport::ServerSseTransport, types::ServerCapabilities,
};
use mcp_core_macros::tool;
use serde_json::json;

use rig::{
    completion::Prompt,
    providers::{self},
};

#[tool(
    name = "Add",
    description = "Adds two numbers together.",
    params(a = "The first number to add", b = "The second number to add")
)]
async fn add_tool(a: f64, b: f64) -> Result<ToolResponseContent> {
    Ok(tool_text_content!((a + b).to_string()))
}

#[tool(
    name = "Sub",
    description = "Subtract 2nd number from 1st",
    params(a = "The first number", b = "The second number")
)]
async fn sub_tool(a: f64, b: f64) -> Result<ToolResponseContent> {
    Ok(tool_text_content!((a -  b).to_string()))
}

#[tokio::main]
async fn main()->Result<(), anyhow::Error>  {
    tracing_subscriber::fmt::init();
    let mcp_server_protocol = Server::builder("add".to_string(), "1.0".to_string())
        .capabilities(ServerCapabilities {
            tools: Some(json!({
                "listChanged": false,
            })),
            ..Default::default()
        })
        .register_tool(AddTool::tool(), AddTool::call())
        .register_tool(SubTool::tool(), SubTool::call())
        .build();

    let mcp_server_transport =
        ServerSseTransport::new("127.0.0.1".to_string(), 3001, mcp_server_protocol);

    let _ = Server::start(mcp_server_transport.clone()).await;

    // Create the MCP client
    let mcp_client = ClientBuilder::new(mcp_server_transport).build();

    // Start the MCP client
    let _ = mcp_client.open().await;
    let init_res = mcp_client
        .initialize(
            Implementation {
                name: "mcp-client".to_string(),
                version: "0.1.0".to_string(),
            },
            ClientCapabilities::default(),
        )
        .await;
    println!("Initialized: {:?}", init_res);

    let tools_list_res = mcp_client.list_tools(None, None).await;
    println!("Tools: {:?}", tools_list_res);

    tracing::info!("Building RIG agent");
    let completion_model = providers::openai::Client::from_env();
    let mut agent_builder = completion_model.agent("gpt-3.5-turbo-0125");

    // Add MCP tools to the agent
    agent_builder = tools_list_res
        .unwrap()
        .tools
        .into_iter()
        .fold(agent_builder, |builder, tool| {
            builder.mcp_tool(tool, mcp_client.clone().into())
        });
    let agent = agent_builder.build();

    tracing::info!("Prompting RIG agent");
    let response = agent.prompt("Add 10 + 10").await;
    tracing::info!("Agent response: {:?}", response);
    Ok(())
}
