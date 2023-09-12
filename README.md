# NBA Fantasy Draft CLI
A CLI tool that simulates an NBA Fantasy Draft.

### Updating Data
To scrape the latest salary information for this year, we've compiled a script that can be run to update the `player_salaries.json` file in the `data` folder:

```
cargo run --bin scrape
```

To scrape the latest team data, we've compiled a script that can be run to update the `teams.json` file in the `data` folder. Please note, you'll need to have an account at rapidapi.com and set the API key for [API-NBA](https://rapidapi.com/api-sports/api/api-nba/) in your `.env` file for this script to work:

```
cargo run --bin scrape_teams
```
