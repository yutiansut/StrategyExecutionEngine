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
Volume-Based Execution Strategy involves executing orders during periods of high trading volume to
minimize market impact. High trading volumes typically indicate greater liquidity, which means
there are more buyers and sellers in the market. This can help in reducing the price impact of
large orders, ensuring that the execution prices are more stable and closer to the desired levels.

High Volume Periods: Times when trading activity is significantly higher, often coinciding with
market openings, closings, or major economic announcements.
Liquidity: The ability to buy or sell an asset quickly without causing a significant price change.
By executing orders during these high-volume periods, traders can benefit from tighter bid-ask
spreads and lower transaction costs. This strategy is particularly useful for large institutional
orders that could otherwise move the market if executed during low volume periods.

Example: If a trader wants to buy a large quantity of a stock, executing this order during the
first hour of trading (when volume is typically higher) can result in a more favorable execution
price compared to executing the same order during a low volume period in the middle of the day.

Reference for Further Reading:

"The Market Maker's Edge" by Joshua Lukeman: This book offers insights into various trading tactics,
 including volume-based execution strategies. (Chapter 3 discusses volume and liquidity
 considerations in trading).
 */