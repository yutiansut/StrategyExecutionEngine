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
    The Bid-Ask Quotation Strategy involves adjusting the bid (buy) and ask (sell) prices to balance
    incoming orders and minimize inventory risk. This strategy is often used by market makers and
    involves setting the bid price slightly lower and the ask price slightly higher than the current
    market price. The goal is to profit from the spread between the bid and ask prices while managing
    the inventory levels effectively.

    Bid Price: The highest price that a buyer is willing to pay for a security.
    Ask Price: The lowest price that a seller is willing to accept for a security.
    Spread: The difference between the bid and ask prices.
    By continuously adjusting these prices based on market conditions, order flow, and inventory levels,
     market makers can manage their risk. If they hold too much inventory, they might lower the ask
     price to sell off some of their holdings. Conversely, if their inventory is too low, they might
     raise the bid price to attract more sellers.

    Reference for Further Reading:

    "Market Microstructure Theory" by Maureen O'Hara: This book provides a comprehensive overview of
    market making and the role of bid-ask spreads in managing inventory risk. (Chapter 2 discusses
    inventory models and bid-ask strategies in detail).
 ******************************************************************************/
