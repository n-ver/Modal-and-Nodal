use thirtyfour::prelude::*;
use tokio::time::Duration;
use std::env;
use std::fmt;
use mongodb::{Client, options::{ClientOptions, ResolverConfig}};
use std::error::Error;
use tokio;
use mongodb::bson::doc;
use mongodb::Database;
use mongodb::Collection;
use bson::Document;
use tokio::sync::mpsc;
use thirtyfour::ChromeCapabilities;


struct MusData {
    id: String,
    name: String,
    instruments: String,
    years: String,
    sessions: i32,
}

impl fmt::Display for MusData {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let str = format!(
            "{} {} {} {} {}",
            self.id,
            self.name,
            self.instruments,
            self.years,
            self.sessions
        );
        write!(fmt, "{}", str) 
        // Ok(())
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("RUST_BACKTRACE", "1");
    let client_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!"); not working

    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;
    let client = Client::with_options(options)?;
    let database = client.database("Jazz_Musicians");
    let (tx, rx) = mpsc::channel(32);
    
    // scraper().await;
    let scraper_handle = tokio::spawn(async move {
        scraper(tx).await;
    });
    
    let upload_handle = tokio::spawn(async move {
        receive_from_scraper(rx, &database).await;
    });
    
    let result = tokio::try_join!(scraper_handle, upload_handle);
    result?;

    Ok(())
}
// Gathers data
async fn scraper(tx: mpsc::Sender<MusData>) -> Result<(), Box<dyn Error>> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    // Navigate to log in page

    driver.get("http://www.library.illinois.edu/proxy/go.php?url=http://www.lordisco.com/tjd/CoverFrame").await?;
    // let it load
    tokio::time::sleep(Duration::from_secs(1)).await;

    let email_form = driver.find_element(By::Id("i0116")).await?;
    
    // Type in the search terms.
    let UIUC_User : String = env::var("UIUC_User").unwrap_or_else(|_| "Unknown user".to_string());
    email_form.send_keys(UIUC_User).await?;
    
    // Click the next button.
    let next_button = driver.find_element(By::Id("idSIButton9")).await?;
    next_button.click().await?;

    // Type in password
    let UIUC_pass : String = env::var("UIUC_Pass").unwrap_or_else(|_| "Unknown user".to_string());
    driver.find_element(By::Id("i0118")).await?;
    tokio::time::sleep(Duration::from_secs(1)).await;
    let pass_form = driver.find_element(By::Id("i0118")).await?;
    pass_form.send_keys(UIUC_pass).await?;
    driver.find_element(By::Id("idSIButton9")).await?;
    tokio::time::sleep(Duration::from_secs(1)).await;
    let next_button2 = driver.find_element(By::Id("idSIButton9")).await?;
    next_button2.click().await?;
    tokio::time::sleep(Duration::from_secs(5)).await;
    let search_button = driver.find_element(By::Css("a[href='MusicianSearch?dest=MusicianIndex'] > .nav-button.musician-element")).await?;
    search_button.click().await?;
    let search_button = driver.find_element(By::Css("input[name='action2'][value='Search']")).await?;
    search_button.click().await?;
    let mut musicians_with_more_than_two_sessions : Vec<MusData> = Vec::new();
    let mut page_num : i64 = 1;
    loop {
        println!("on page {}", page_num);
        let bottom_buttons = driver.find_element(By::Css(".bottom_buttons")).await?;
        if let Ok(next_page_button) = bottom_buttons.find_element(By::Css("a[href*='nav=down']")).await {
            let table = driver.find_element(By::Css("table.index")).await?;
            let rows = table.find_elements(By::Css("tr")).await?;
            for row in rows.iter().skip(1) { // Skip header row
                let cells = row.find_elements(By::Css("td")).await?;
                if cells.len() > 2 { // Ensure there are enough columns in the row
                    let session_count: i32 = cells[3].text().await?.parse().unwrap_or(0);
                    if session_count >= 25 { // ensure musician meets minimum requirement
                        let data: MusData = MusData {
                            id: row.get_attribute("id").await?.unwrap_or_else(|| "id failed".to_string()),
                            name: cells[0].text().await?,
                            instruments: cells[2].text().await?,
                            years: cells[1].text().await?,
                            sessions: session_count,
                        };
                        
                        tx.send(data).await.expect("Failed to send data");
                    }
                }
            }
            page_num = page_num + 1;
            next_page_button.click().await?;
        } else {
            break;
        }
    }

        return Ok(())
    }
// Uploads to database
async fn upload(database: &Database, data: MusData) -> Result<(), Box<dyn Error>> {
    // filters into correct collection
    if data.sessions < 50 {
        let collection = database.collection("0025");
        insert_or_not(collection, data).await?;
    } else if data.sessions < 100 {
        let collection = database.collection("0050");
        insert_or_not(collection, data).await?;
    } else if data.sessions < 200 {
        let collection = database.collection("0100");
        insert_or_not(collection, data).await?;
    } else if data.sessions < 500 {
        let collection = database.collection("0200");
        insert_or_not(collection, data).await?;
    } else if data.sessions < 1000 {
        let collection = database.collection("0500");
        insert_or_not(collection, data).await?;
    } else {
        let collection = database.collection("1000");
        insert_or_not(collection, data).await?;
    }
    Ok(())
}
async fn receive_from_scraper(mut receiver: mpsc::Receiver<MusData>, database: &Database) -> Result<(), Box<dyn Error>> {
    while let Some(data) = receiver.recv().await {
        println!("MusData {{ \nid : \"{}\",\nname : \"{}\",\ninstruments : \"{}\",\nyears : \"{}\",\nsessions : {},\n}},", data.id, data.name, data.instruments, data.years, data.sessions);
        upload(&database, data).await?;
    }
    Ok(())
}

// inserts if not already in database
async fn insert_or_not(collection: Collection<Document>, data: MusData)  -> Result<(), Box<dyn Error>> { 
    println!("insert or not running");
    let already_exists: Option <Document> = collection.find_one(
        doc! {
              "id": data.id.clone(),
        },
        None,
     ).await?;
     if already_exists.is_none() { // entry does not exist
        println!("making new entry");
        let new_doc = doc! {
            "id" : data.id,
            "name" : data.name,
            "instruments" : data.instruments,
            "years active" : data.years,
            "session count" : data.sessions,
        };
    let insert_result = collection.insert_one(new_doc.clone(), None).await?;
     } 
     Ok(())
}
