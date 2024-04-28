use thirtyfour::prelude::*;
use tokio;
use rpassword::read_password;
use input::input;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    // Prompt for email and password
    println!("Please enter your email:");
    let email: String = input().get();

    println!("Please enter your password (input will be hidden):");
    let password: String = read_password().expect("Password input failed");

    let caps = DesiredCapabilities::firefox();
    let driver = WebDriver::new("http://localhost:4444", &caps).await?;
    driver.get("https://login.microsoftonline.com").await?;

    // Use the provided email and password in the login process
    let email_field = driver.wait_for_element(By::Id("i0116")).await?;
    email_field.send_keys(email).await?;

    let next_button = driver.wait_for_element(By::Id("idSIButton9")).await?;
    next_button.click().await?;

    // Wait to ensure navigation to password entry if necessary
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    let password_field = driver.wait_for_element(By::Id("i0118")).await?;
    password_field.send_keys(password).await?;

    let login_button = driver.wait_for_element(By::Id("idSIButton9")).await?;
    login_button.click().await?

    Ok(())
}
