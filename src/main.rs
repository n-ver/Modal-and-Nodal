use thirtyfour::prelude::*;
use tokio::time::Duration;
use std::env;

#[tokio::main]
async fn main() {
    login().await;
    
}

async fn login() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", &caps).await?;

    // Navigate to log in page

    driver.get("http://www.library.illinois.edu/proxy/go.php?url=http://www.lordisco.com/tjd/CoverFrame").await?;
    // let it load
    tokio::time::sleep(Duration::from_secs(1)).await;

    let email_form = driver.find_element(By::Id("i0116")).await?;
    
    // Find element from element.
    // let elem_text = elem_form.find_element(By::Id("searchInput")).await?;

    // Type in the search terms.
    // let UIUC_User : String = env::var("UIUC_User").unwrap_or_else(|_| "Unknown user".to_string());
    let UIUC_User = "ronaldc5@illinois.edu".to_string();
    email_form.send_keys(UIUC_User).await?;
    
    // Click the next button.
    let next_button = driver.find_element(By::Id("idSIButton9")).await?;
    next_button.click().await?;

    // Type in password
    // let UIUC_pass : String = env::var("UIUC_Pass").unwrap_or_else(|_| "Unknown user".to_string());
    let UIUC_pass = "Rc050109#123".to_string();
    driver.find_element(By::Id("i0118")).await?;
    tokio::time::sleep(Duration::from_secs(1)).await;
    let pass_form = driver.find_element(By::Id("i0118")).await?;
    pass_form.send_keys(UIUC_pass).await?;
    driver.find_element(By::Id("idSIButton9")).await?;
    tokio::time::sleep(Duration::from_secs(1)).await;
    let next_button2 = driver.find_element(By::Id("idSIButton9")).await?;
    next_button2.click().await?;
    // Look for header to implicitly wait for the page to load.
    // driver.find_element(By::ClassName("firstHeading")).await?;
    // assert_eq!(driver.title().await?, "Selenium - Wikipedia");

    tokio::time::sleep(Duration::from_secs(5)).await;
    // let search_button = WebDriverWait::new(&driver, std::time::Duration::from_secs(10))
    //     .until(|d| d.find_element(By::Css("div.musician #searchButtonId")));
    // search_button.click().await?;
    let search_button = driver.find_element(By::Css("a[href='MusicianSearch?dest=MusicianIndex'] > .nav-button.musician-element")).await?;
    search_button.click().await?;
    let search_button = driver.find_element(By::Css("input[name='action2'][value='Search']")).await?;
    search_button.click().await?;
    
    
    // let table = driver.find_element(By::Css("table.index")).await?;
    // let rows = table.find_elements(By::Css("tr")).await?;
    // let mut musicians_with_more_than_two_sessions = Vec::new();

    // for row in rows.iter().skip(1) {  // Skip header row
    //     let cells = row.find_elements(By::Css("td")).await?;
    //     if cells.len() > 2 { // Ensure there are enough columns in the row
    //         let musician_name = cells[0].text().await?; // Index might need adjustment
    //         let session_count: i32 = cells[3].text().await?.parse().unwrap_or(0); // Index might need adjustment

    //         if session_count > 2 {
    //             musicians_with_more_than_two_sessions.push(musician_name);
    //         }
    //     }
    // }
    match get_musicians_with_sessions(&driver).await {
        Ok(musicians) => {
            if !musicians.is_empty() {
                println!("Musicians with more than two sessions:");
                for musician in musicians {
                    println!("{}", musician);
                }
            } else {
                println!("No musicians found with more than two sessions.");
            }
        },
        Err(e) => {
            eprintln!("Failed to fetch musicians: {}", e);
        }
    }
    loop {
        let links = driver.find_elements(By::Css("a[href*='nav=down']")).await?;
        
        if links.is_empty() {
            break;
        }

        for link in links {
            
            link.click().await?;
            match get_musicians_with_sessions(&driver).await {
                Ok(musicians) => {
                    if !musicians.is_empty() {
                        println!("Musicians with more than two sessions:");
                        for musician in musicians {
                            println!("{}", musician);
                        }
                    } else {
                        println!("No musicians found with more than two sessions.");
                    }
                },
                Err(e) => {
                    eprintln!("Failed to fetch musicians: {}", e);
                }
            }
            
        }

    }
    
    // Output the musicians
    // for musician in musicians_with_more_than_two_sessions {
    //     println!("Musician: {}", musician);
    // }
    Ok(())
}

async fn get_musicians_with_sessions(driver: &WebDriver) -> WebDriverResult<Vec<String>> {
    let table = driver.find_element(By::Css("table.index")).await?;
    let rows = table.find_elements(By::Css("tr")).await?;

    let mut musicians_with_more_than_two_sessions: Vec<String> = Vec::new();

    for row in rows.iter().skip(1) {

        let cells = row.find_elements(By::Css("td")).await?;

        if cells.len() > 2 {
            // Extract the musician's name from the first column
            let musician_name = cells[0].text().await?;
            let years = cells[1].text().await?;
            let instruments = cells[2].text().await?
            // Try to parse the session count from the fourth column and handle parsing errors
            let session_count: i32 = cells[3].text().await?.parse().unwrap_or(0);

            // Check if the session count is greater than two
            if session_count > 2 {
                musicians_with_more_than_two_sessions.push("name:" + musician_name + " years:" + years + " instruments:" + instruments + " session_count:" + session_count);
            }
        }
    }

    // Return the list of musicians
    Ok(musicians_with_more_than_two_sessions)
}