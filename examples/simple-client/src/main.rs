use rig::{agent::AgentBuilder, completion::Prompt, providers::doubao::{self, DOUBAO_PRO_32K}};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Create Doubao clients
    let openai_client = doubao::Client::from_env();
    let model = openai_client.completion_model(DOUBAO_PRO_32K);

    // Create an agent with multiple context documents
    let agent = AgentBuilder::new(model)
        .context("Definition of a *flurbo*: A flurbo is a green alien that lives on cold planets")
        .context("Definition of a *glarb-glarb*: A glarb-glarb is a ancient tool used by the ancestors of the inhabitants of planet Jiro to farm the land.")
        .context("Definition of a *linglingdong*: A term used by inhabitants of the far side of the moon to describe humans.")
        .build();

    // Prompt the agent and print the response
    let response = agent.prompt("What does \"glarb-glarb\" mean?").await?;

    println!("{}", response);

    Ok(())
}
