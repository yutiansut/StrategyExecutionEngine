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
Liquidity Seeking
Based on "Empirical Market Microstructure" by Joel Hasbrouck, liquidity seeking
strategies involve executing orders in segments of the market with the highest
liquidity. The aim is to minimize market impact by taking advantage of areas
where there are more buyers and sellers, ensuring better execution prices and
reduced slippage.

Example: A trader identifies the most liquid times and venues for a particular
security and schedules their large order to be executed during these periods,
thereby reducing the likelihood of moving the market price.

Reference for Further Reading:
"Empirical Market Microstructure" by Joel Hasbrouck: This book provides a
detailed analysis of market structures and the dynamics of liquidity. (Chapter 3
discusses liquidity and execution strategies in detail).
*/

