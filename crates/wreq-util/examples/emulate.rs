use wreq_util::{Emulation, Platform, Profile};

#[tokio::main]
async fn main() -> Result<(), wreq::Error> {
    let client = wreq::Client::builder_tokio().build()?;
    // Example 1: Basic emulation - Safari26
    let text = client
        .get("https://pingly.us.kg/api/all")
        .emulation(Emulation::Safari26)
        .send()
        .await?
        .text()
        .await?;
    println!("{}\n", text);

    // Example 2: Advanced emulation with options - OkHttp5
    let text = client
        .get("https://pingly.us.kg/api/all")
        .emulation(
            Emulation::builder()
                .profile(Profile::OkHttp5)
                .platform(Platform::Windows)
                .http2(false)
                .build(),
        )
        .send()
        .await?
        .text()
        .await?;
    println!("{}\n", text);

    // Example 3: Random device emulation
    let text = client
        .get("https://pingly.us.kg/api/all")
        .emulation(Emulation::random())
        .send()
        .await?
        .text()
        .await?;
    println!("{}", text);

    Ok(())
}
