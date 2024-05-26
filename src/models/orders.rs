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
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProductType {
    Spot,
    Futures,
    Options,
    Swap,
    CFD,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderType {
    Market,
    Limit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptionType {
    Call,
    Put,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeInForce {
    GTC, // Good-Til-Canceled
    IOC, // Immediate-Or-Cancel
    GTD, // Good-Til-Date
    FOK, // Fill-Or-Kill
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssetClass {
    Stock,
    Bond,
    Commodity,
    Currency,
    Crypto,
    ETF,
    MutualFund,
    Index,
    Equity,
    Derivative,
    Insurance,
    Loan,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spot {
    pub asset_class: Option<AssetClass>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Futures {
    pub delivery_date: Option<u64>,
    pub contract_size: Option<f64>,
    pub margin: Option<f64>,
    pub commission: Option<f64>,
    pub overnight_fee: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Options {
    pub strike_price: f64,
    pub option_type: OptionType,
    pub expiry_date: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Swap {
    pub fixed_rate: f64,
    pub floating_rate_index: String,
    pub notional_amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CFD {
    pub leverage: Option<u32>,
    pub margin: Option<f64>,
    pub commission: Option<f64>,
    pub overnight_fee: Option<f64>,
    pub dividend_adjustment: Option<f64>,
    pub contract_size: Option<f64>,
}

/// Common structure for orders.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub quantity: u32,
    pub product_type: ProductType,
    pub order_type: OrderType,
    pub price: Option<f64>,
    pub timestamp: u64,
    pub expiry_date: Option<u64>,
    pub symbol: String,
    pub side: Side,
    pub currency: String,
    pub exchange: Option<String>,
    pub timeinforce: Option<TimeInForce>,

    // Futures specific fields
    pub futures_opt: Option<Futures>,

    // Options specific fields
    pub options_opt: Option<Options>,

    // Swaps specific fields
    pub swap_opt: Option<Swap>,

    // CFDs specific fields
    pub cfd_opt: Option<CFD>,
}
impl Order {
    pub fn new(
        id: String,
        quantity: u32,
        product_type: ProductType,
        order_type: OrderType,
        price: Option<f64>,
        timestamp: u64,
        expiry_date: Option<u64>,
        symbol: String,
        side: Side,
        currency: String,
        exchange: Option<String>,
        timeinforce: Option<TimeInForce>,
        futures_opt: Option<Futures>,
        options_opt: Option<Options>,
        swap_opt: Option<Swap>,
        cfd_opt: Option<CFD>,
    ) -> Self {
        Order {
            id,
            quantity,
            product_type,
            order_type,
            price,
            timestamp,
            expiry_date,
            symbol,
            side,
            currency,
            exchange,
            timeinforce,
            futures_opt,
            options_opt,
            swap_opt,
            cfd_opt,
        }
    }
}
