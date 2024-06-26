# Purpose
The purpose of this crate is to provide a library that simplifies development of algorithmic trading strategies via composable components.

## Ideas

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

How do encode market conditions into our strategy components?

### Example

Let's say we have two strategies:

1. A strategy that buys/sells depending on the security's 50-day moving average
2. A strategy that buys/sells depending on some D/P ratio

Are these compatible? What would a combined strategy look like?

1. A strategy that buys/sells depending on the security's 50-day moving average iff the D/P ratio is below some threshold

## Structure
How do we encode this?

We could use a vector, where our conditions are stored in a vector, and we iterate over them to make a decision.

In this case, our 'strategy' struct would contain a vector of conditions.

### Conditions

Generally, a strategy checks to see if some value is above or below some threshold, which we'll refer to as a condition or a signal.
(e.g. 50-SMA > 200-SMA)

### Context

But how do we represent the market conditions required for the strategy to make a decision?
Some conditions require data that is not required by other conditions.

Is there some kind of hierarchy we can use to represent this?

I think market data can be separated into two categories: general market conditions and security-specific conditions.

What about sentiment data? How do we represent that? Probably optionally as part of security-specific data.
A weird edge case is that SPY represents the general market, so it's both general and security-specific.