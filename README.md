[![Rust](https://github.com/int0x81/horus_rust_/actions/workflows/rust.yml/badge.svg)](https://github.com/int0x81/horus_edge/actions/workflows/rust.yml)

# Horus

### What is horus
Horus is a framework for algorithmic trading. It provides functionality to
- Create trading strategies based on multiple data streams
- Run backtests against strategies
- Route orders to exchanges

### What can you do with horus
With Horus you can build trading strategies in rust. These strategies can be backtested
against historical data and can trade on real world exchanges. Strategies can watch multiple
datastrams in parallel and can act based on signals from these datastreams.
These datastreams can be provide market data as well as alternative data (social media, etc.)

Horus also provides a tool called AutoQuant that can backtest a strategy with various parameters
to find the best fitting parameters for a past market period.

### Where is this project different from other algorithmic trading platforms
This project has a huge focus on performance and reliability. The trading agents that contain
your strategy are compiled to binary programs that are tuned for high performance by low level techniques (minimizing heap usage, avoiding vtables, reducing CPU instructions, etc...)


## TODOS

### General
- SPIKE: Make use of clippy (in pipeline)
- SPIKE: Make use of incode documentation
- Apply performance tips from the rust perf book to the entire codebase
- Make the whole project representable

### Autoquant
- Consider the case that no strategy has been found

### Backtesting
- Implement backtesting based on fixed time intervals
- Create a eventlog mechanism in the backtestresult struct
- Create unit tests
- SPIKE: Find a mechanism for displaying a backtest run

### Signals
- Rename the strategy struct and implementations to signals and align implementations
- Create new Strategy struct that returns a new "StrategyDecision" object that contains human readable text


### Datastreams
- Migrate the technical indicators from the C# project
- Implement a mechanism for getting more historical data in a safe way
- Caching historical market data

### Terminal
- Migrate the entire C# Horus Terminal
