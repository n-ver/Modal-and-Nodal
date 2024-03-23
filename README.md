# Modal and Nodal
## Basic Info
Group Name: Modal & Nodal


Group member names and NetIDs:

Nata Verthein (NetID: av45)

KB Cho (NetID: ronaldc5)


## Project Introduction:
Jazz is an under-digitized art form with a long history.  There exists a database of all jazz musicians and jazz recordings named “The Jazz Discography” (TJD, also known as Lordisco), but it’s not possible to sort by any criteria other than performer name or song title.  This is functional enough for its usual customers - jazz archivers - but it’s inadequate for any mass-scale research.  This project aims to create a database of preeminent jazz musicians by scraping TJD with a focus on recording sessions.


“Preeminent Musicians” will be defined as a musician with more than 25 recording sessions, a metric TJD helpfully tracks.  Our project will save basic information, such as artist name, years active, instruments, quantity of recording sessions, and TJD ID (to allow for easy reference for future scraping).


This information allows for many possible future use-cases.  It can later be used to create a node-edge map of musicians and their collaborations sorted by date-range / era, sorting by recording session quantity (a basic and currently impossible way to find important musicians), used to further mine individual musician’s pages to make a separate database of the most prominent jazz songs (also something that doesn’t currently exist), or any other uses that need a dataset.  It is also possible to use this data to gather information on each musician’s region (another metric TJD conveniently tracks, but that is not searchable), allowing for regional connections as well as temporal. This would be an invaluable resource that simply doesn’t exist yet due to the extremely small overlap between jazz musician and programmer.


## Technical Overview:
There are three core components of this project.  The current plan is to finish (1) by checkpoint 1, (2) (and a significant portion of 3) by checkpoint 2, and (3) by presentation date:

1. Access the database through a web scraper

2. Scrape the pages for our desired information (performer name, years active, instruments, number of recording sessions, and TJD ID)

3. Store the information in a database

(1) is the first and likely hardest part of this project.  TJD requires institutional authorization from UIUC, so our web scraper would have to navigate the UIUC log in portal.  This is necessary in order to access the discography.

(2) is the bulk of the web scraping.  The website we chose stores its information in plain HTML, and our desired information is stored in blocks like the following:
```
<tr id="M63562" class="index1">
	<td><a href="Musicianzetail?mid=63562">&quot;Four String Jack&quot;</a></td>
	<td>1973</td>
	<td>el-b</td>
	<td>1</td>
</tr>
```
For a full example web page taken from the site in question, please see Example.html.

The above stores information for a man named “Four String Jack”, who was active in 1973, played Electric Bass (el-b), and had 1 recording session.  We simply need to write a web scraper to iterate through each page, collect entries that pass our minimum session threshold of 25 recordings, and move to the next page.  This is not a modern site so it likely doesn’t have scraping protection, but this would still be a good stage to implement some safeguarding, such as a maximum # of requests per hour, to prevent our IP from being blocked or log in credentials from being revoked.

(3) will be implemented in tandem with (2).  Our goal with this project is to gather information in a format that allows for future computation and analysis, so we need to store it in a database.  This involves picking a database (SQL or NOSQL), finding appropriate libraries in Rust, and learning how to manage it and best store information so that it’s usable in the future.




## Possible Challenges: 
accessing source page behind account verification (how to log in with web scraper?)

There are fewer libraries and tools in Rust to make use of in web scraping

Since the web scraping involves a lot of scraping, working with the string in rust could be a challenging aspect of the project

What kind of database should we use (SQL or NOSQL)?

Is it possible to incorporate multithreading?

How much computational power will this take and how will we manage it?

- Rent computing power? 

- Cron job?

- Cloud computing?


References:
https://www.lordisco.com 


__________________________________

