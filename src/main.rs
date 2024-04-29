use thirtyfour::prelude::*;
use tokio::time::Duration;
use std::env;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", &caps).await?;

    // Navigate to https://wikipedia.org.
    driver.get("https://us.prairielearn.com/pl/auth/institution/3/saml/login").await?;
    tokio::time::sleep(Duration::from_secs(1)).await;
    let elem_form = driver.find_element(By::Id("i0116")).await?;
    
    // Find element from element.
    // let elem_text = elem_form.find_element(By::Id("searchInput")).await?;

    // Type in the search terms.
    let UIUC_User : String = env::var("UIUC_User").unwrap_or_else(|_| "Unknown user".to_string());
    elem_form.send_keys(UIUC_User).await?;

    // Click the search button.
    // let elem_button = elem_form.find_element(By::Css("button[type='submit']")).await?;
    // elem_button.click().await?;

    // Look for header to implicitly wait for the page to load.
    // driver.find_element(By::ClassName("firstHeading")).await?;
    // assert_eq!(driver.title().await?, "Selenium - Wikipedia");
    tokio::time::sleep(Duration::from_secs(5)).await;
    // Always explicitly close the browser.
    driver.quit().await?;

    Ok(())
}
