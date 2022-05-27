[![Rust](https://github.com/int0x81/horus_rust_/actions/workflows/rust.yml/badge.svg)](https://github.com/int0x81/horus_edge/actions/workflows/rust.yml)

# Horus
This mono repo represents an algorithmic trading platform.
It contains
- financial libraries
- templates for building trading agents
- frontend services of the platform
- backend services of the platform
- scripts for setting up the entire platform on linux

## TODOS
- Improve testing/coverage
- Takeover technical indicators form C# Horus project
- Create eventlog for backtest results
- Create new "StrategyDecision" struct that contains a homan readable text
- Reporting for backtests (Terminal/Webapp/redirect to tradingview?)
- Mechanism for getting more data ranges for backtesting (eg. not just 1000 from binance) in a safe way
- Use Clippy
- Optimize existing codebase for perfomance (heap monitoring, hot paths analysis)
- Consider the case that no strategy has been found
- Write documentation
- Migrate the entire C# Horus codebase
- Migrate the entire C# Horus Terminal
