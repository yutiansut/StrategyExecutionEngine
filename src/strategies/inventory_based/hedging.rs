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


/******************************************************************************
Hedging involves using derivatives or other financial instruments to offset potential losses in
an inventory. This strategy is crucial for managing risk associated with holding positions in
various securities. Derivatives such as futures, options, and swaps are commonly used for hedging.

Derivatives: Financial contracts whose value is derived from the value of an underlying asset
(e.g., stocks, bonds, commodities).
Futures Contracts: Agreements to buy or sell an asset at a future date for a predetermined price.
Options Contracts: Contracts that give the holder the right, but not the obligation, to buy or
sell an asset at a predetermined price before a specific date.
Swaps: Agreements to exchange cash flows or other financial instruments between parties.
By entering into a derivative position that is opposite to their inventory position, traders
can protect against adverse price movements. For example, if a trader holds a large inventory of
a particular stock, they might buy put options (which increase in value as the stock price
decreases) to hedge against potential losses if the stock price falls.

Reference for Further Reading:

"Empirical Market Microstructure" by Joel Hasbrouck: This book covers various aspects of market
microstructure, including hedging strategies and their applications in financial markets. (Chapter
4 provides insights into hedging and risk management).
These references will provide you with detailed explanations and further insights into how these
strategies are implemented and the theoretical foundations behind them.
******************************************************************************/
