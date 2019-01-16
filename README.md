# Music Manager

## Workflow

* API endpoint to collect the albums for an artist
    * -> Creates Albums Task in the database
* Task runner picks up the album task
    * -> Executes the scraper passing in the name from the task
    * -> Scraper output is parsed and new tasks are created for each album
* Task runner picks up each song list task and executes the scraper again
