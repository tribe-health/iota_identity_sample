use identity::crypto::KeyPair;
use identity::iota::Client;
use identity::iota::Document;
use identity::iota::Network;
use identity::iota::Result;
use identity::iota::TangleRef;

#[tokio::main]
async fn main() -> Result<()> {
    // Create a client to interact with the IOTA Tangle.
    let client: Client = Client::new()?;

    // Create a DID Document (an identity).
    let keypair: KeyPair = KeyPair::new_ed25519()?;
    let mut document: Document = Document::from_keypair(&keypair)?;

    // Sign the DID Document with the default authentication key.
    document.sign(keypair.secret())?;

    // Use the client to publish the DID Document to the IOTA Tangle.
    document.publish(&client).await?;

    // Print the DID Document transaction link.
    let network: Network = document.id().into();
    let explore: String = format!("{}/transaction/{}", network.explorer_url(), document.message_id());

    println!("DID Document Transaction > {}", explore);

    Ok(())
}