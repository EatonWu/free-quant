# Purpose

The purpose of this crate is to provide a library that simplifies development of algorithmic trading strategies via composable components.

# Ideas

My primary idea is the abstraction of 'strategies' into composable components.
This is inspired by the 'strategy pattern' in object-oriented programming.

The idea is to have a 'strategy' trait that defines a method that takes a 'context' and returns a 'decision'.
The 'context' would contain all the information that the strategy needs to make a decision.
The 'decision' would contain the action to take (buy, sell, hold) and the quantity to take that action with.

In this case, our context would be our financial data, our decision the trading action, and our strategy the algorithm that decides
what to do depending on the context.

## 'composable' components?

The idea is that we can have multiple strategies that can be combined to form a new strategy.

Of course, strategies that depend on the same data cannot be combined, but strategies that depend on different data can be combined.

How do we represent this with Rust's type system?

### Example

Let's say we have two strategies:

1. A strategy that buys/sells depending on the security's 50 day moving average
2. 

