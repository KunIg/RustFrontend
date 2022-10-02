# No code - low code blockchain data analytics


The project, splitted in 3 components, has the goal to make on-chain data easily and visualy accessible for average users.

## 1) The data aggregator.

It crawls the history of the ethereum chain and aggregates all data to reflect the current state of affairs. Once the whole history has been processed, it watches the tip of the chain to keep the collections up-to-date, as a deamon.

It is configurable so that a set of filters can be set via a .toml file so it can fits the needs of any apps.

Disclaimer: The data is parsed, not yet aggregated to the database. Configuration .toml file not yet implemented.


## 2) The middleware.

It runs a web server to make accessible the aggregated data, exposing a set of GET endpoints.

Disclaimer: Quick and dirty. Switch from ORM and SQLITE asap!

## 3) A single page app for the front.

=> TODO IGOR


![Blank diagram (1)](https://user-images.githubusercontent.com/33451439/193436465-65e161ed-e1d4-4a19-8779-a6db88007197.jpeg)
