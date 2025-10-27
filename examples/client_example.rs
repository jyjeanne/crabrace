use anyhow::Result;
use crabrace::CrabraceClient;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🦀 Crabrace Client Example\n");

    // Create a client pointing to the Crabrace server
    let client = CrabraceClient::new("http://localhost:8080");

    // Check if the server is healthy
    println!("Checking server health...");
    match client.health_check().await {
        Ok(true) => println!("✅ Server is healthy\n"),
        Ok(false) => println!("❌ Server is not healthy\n"),
        Err(e) => {
            eprintln!("❌ Failed to connect to server: {}", e);
            eprintln!("Make sure the Crabrace server is running on http://localhost:8080");
            return Ok(());
        }
    }

    // Get all providers
    println!("Fetching providers...");
    let providers = client.get_providers().await?;

    println!("Found {} providers:\n", providers.len());

    // Display provider information
    for provider in &providers {
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("📦 {} ({})", provider.name, provider.id);
        println!("   Models: {}", provider.models.len());

        if let Some(api_endpoint) = &provider.api_endpoint {
            println!("   API Endpoint: {}", api_endpoint);
        }

        println!("\n   Available Models:");
        for model in &provider.models {
            println!("   • {} ({})", model.name, model.id);
            println!("     - Context Window: {} tokens", model.context_window);
            println!(
                "     - Cost: ${:.2}/1M in, ${:.2}/1M out",
                model.cost_per_1m_in, model.cost_per_1m_out
            );

            // Display capabilities
            let mut capabilities = Vec::new();
            if model.can_reason {
                capabilities.push("reasoning");
            }
            if model.supports_attachments {
                capabilities.push("vision/attachments");
            }

            if !capabilities.is_empty() {
                println!("     - Capabilities: {}", capabilities.join(", "));
            }

            // Calculate example cost (without caching)
            let example_cost = model.calculate_cost(100_000, 50_000, false);
            println!(
                "     - Example cost (100k in, 50k out): ${:.4}",
                example_cost
            );

            println!();
        }
    }

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // Calculate total models
    let total_models: usize = providers.iter().map(|p| p.models.len()).sum();
    println!("\n📊 Summary:");
    println!("   Total Providers: {}", providers.len());
    println!("   Total Models: {}", total_models);

    Ok(())
}
