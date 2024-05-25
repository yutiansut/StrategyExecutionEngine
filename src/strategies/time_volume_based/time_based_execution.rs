/*******************************************************************************
Copyright (c) 2024.

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
******************************************************************************/

/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 25/5/24
******************************************************************************/

/*
Time-Based Execution Strategy
Time-Based Execution Strategy involves scheduling trades at specific times of the day when the
market is known to be more liquid. Market liquidity varies throughout the trading day, with certain
 periods exhibiting higher trading activity and tighter spreads.

Liquidity Windows: Specific times during the trading day when the market is most active and liquid,
 such as the market open (first hour) and close (last hour).
Scheduled Orders: Planning trades to coincide with these high-liquidity periods to achieve better
execution prices.
By timing trades to align with these liquidity windows, traders can take advantage of the increased
market activity and potentially reduce the cost of their trades. This strategy is particularly
effective for reducing slippage, which is the difference between the expected price of a trade and
the actual executed price.

Example: A trader might schedule their trades to occur during the opening hour of the market when
trading volume and liquidity are typically highest, thereby achieving a more favorable execution
price.

Reference for Further Reading:

"The Market Maker's Edge" by Joshua Lukeman: This book explains the importance of timing in trade
execution and how traders can optimize their strategies by understanding market liquidity patterns.
(Chapter 4 covers timing and execution strategies in detail).
 */
