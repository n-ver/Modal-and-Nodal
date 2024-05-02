# Start Here
There are three necessary parts to set up: the code, the webdriver for the scraper, and the database to store data.  Note that this webscraper needs access to your UIUC account in order to bypass the Microsoft Authentication this website uses.  We have implemented this using your computer's environmental variables, so our code does not see your log in information.  Below is info on setting up the code:

1. Git clone to get the source code from the github
2. Using the environmental variable setting in your computer, set up ```UIUC_User = [your UIUC email]``` and ```UIUC_Pass = [your UIUC password]```.  Both the key and value are case sensitive!
3. Restart your computer.
4. Run Cargo build and Cargo run in the terminal in the respective root directory.
# The Webdriver
Below are steps to set up a chrome webdriver.  This is necessary for the scraper to work.
1. Go to [this website](https://chromedriver.chromium.org/downloads) and follow instructions to download the appropriate webdriver (make sure it is a webdriver!).  You likely have a newer version of chrome; click on the red text to see newer versions.  You may have to downgrade chrome.

2. Unzip the file and add it to PATH.

3. Open up a console (terminal outside of VS code), navigate to the unzipped chromedriver folder, and execute chromedriver.exe inside the terminal.  Leave this running while executing the code.

# The Database
We chose to use a MongoDB database for this project.  We expect you to set up your own database if you want to run the scraper.  You can read the data with the below URI, and we also shared recordings of the inside of our database in the video presentation.  We don't feel comfortable including write access to our database inside a public GitHub.  If you still want write access to our own database, message us on Discord.  Below are the steps to setting up your own database.
1. Make an account on [MongoDB](https://account.mongodb.com/account/login).
2.  Click **New Project**.  Name it whatever you'd like.  Click through "Create Project."
3. On the screen "Overview", click "Create Deployment."
4. **Click M0.**  This database is free!!  Click "Create Deployment".
5. Create a database user.  This is **not** the same thing as your MongoDB account.  You will provide the log-in information to our scraper.  Do not use a password you care about.
6. **Click "Create Database User"**.
7. Click "Choose a Connection Method".
8. Click "Drivers".
9. For the driver, select **Python** and **3.6 or later**.  This is the instruction given on the official Rust / MongoDB integration.  Do not select Rust.
10. You might have to download Python and the Pymongo driver.  I'm not sure, as I already had both of these installed from a previous project.  Ignore this step unless things start failing.

You are not done yet.  You have to either update our code to hold your connection string, or add it as an environmental variable
```MONGODB_URI = [your string here]```.
1. Copy the connection string provided.  If you lose it, just navigate back to Deployment > Database and click on "Connect" to view it again.
2. Environmental Variable: Add the environmental variable ```MONGODB_URI = [your string here]```.  Restart your computer.
3. Non-Environmental Variable:  Open the code, navigate to `main.rs`, and replace line 39 with the following:

`let client_uri = "[your link here without brackets]";`

Everything should now work.

# Database Problems
You might have to whitelist your own IP address.  Steps to do so:
1. Open MongoDB's website and log in to your own account.
2. Navigate to Data Services in the header bar second from the top.
3. On the right, scroll down to the bottom of the left pane.
4. Find the green "Security" title on the left header and click on "Network Access" somewhere underneath it.
5. Click "Add IP Address".
6. Click "Allow Access from Anywhere".  Click Confirm.
7. Make sure that the list of IP addresses at the bottom of the page now contains the line:

`0.0.0.0/0  (includes your current IP address)`
