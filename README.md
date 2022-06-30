[![Rust](https://github.com/int0x81/horus/actions/workflows/rust.yml/badge.svg)](https://github.com/int0x81/horus/actions/workflows/rust.yml)

# Horus

### What is horus
Horus is a framework for algorithmic trading. It provides functionality to
- Create trading strategies based on multiple data streams
- Run backtests against these strategies
- Route orders to exchanges

### What can you do with horus
With Horus, you can build trading strategies in rust. These strategies can be backtested
against historical data and can trade on real-world exchanges. Strategies can watch multiple
data streams in parallel and can act based on signals from these data streams.
These data streams can provide market data as well as alternative data (social media, etc.)

Horus also provides a tool called AutoQuant that can backtest a strategy with various parameters
to find the best fitting parameters for a past market period.

### Where is this project different from other algorithmic trading platforms
This project has a huge focus on performance and reliability. The trading agents that contain a strategy are compiled to binary programs that are tuned for high performance by low-level techniques like 
- ✔ minimizing heap usage
- ✔ avoiding vtables
- ✔ reducing CPU instructions
- ❌avoiding POSIX sockets via kernel-bypass networking

## TODOS

### General
- SPIKE: Make use of incode documentation
- Apply performance tips from the rust perf book to the entire codebase
- Make the whole project representable. Create tutorials on how to create a strategy, how to backtest a strategy, and how to run a strategy

### Backtesting
- Implement backtesting based on fixed time intervals
- Create a eventlog mechanism in the backtestresult struct
- Create unit tests
- SPIKE: Find a mechanism for displaying a backtest run

### Signals

### Datastreams
- Implement a mechanism for getting more historical data in a safe way
- Create a service for local caching of historical market data with Redis time series
- Streams should only calculate the needed signals when receiving a new item. This is know at compile time

### Terminal
- Migrate the entire C# Horus Terminal

### Autoquant
- Consider the case that no strategy has been found