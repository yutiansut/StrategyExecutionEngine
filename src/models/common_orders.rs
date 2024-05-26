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
    GTC,
    IOC,
    GTD,
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
    pub contract_symbol: String,
    pub delivery_date: u64,
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


/// Common structure for orders.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderCommon {
    pub id: String,
    pub quantity: u32,
    pub product_type: ProductType,
    pub order_type: OrderType,
    pub price: Option<f64>,
    pub timestamp: u64,
    pub expiry_date: Option<u64>,
    pub symbol: String,
    pub side: Side,
    pub strategy_id: Option<String>,
    pub timeinforce: Option<TimeInForce>,

    // Futures specific fields
    pub futures_opt: Option<Futures>,

    // Options specific fields
    pub options_opt: Option<Options>,

    // Swaps specific fields
    pub swap_opt: Option<Swap>,

    // CFDs specific fields
    pub cfd_opt: Option<String>,

}


/// Trait defining common behaviors for orders.
pub trait OrderTrait {
    fn new(id: String, quantity: u32) -> Self;
    fn get_id(&self) -> &String;
    fn get_quantity(&self) -> u32;
}

impl OrderCommon {
    pub fn new(id: String, quantity: u32) -> Self {
        OrderCommon { id, quantity }
    }
}

impl OrderTrait for OrderCommon {
    fn new(id: String, quantity: u32) -> Self {
        OrderCommon::new(id, quantity)
    }

    fn get_id(&self) -> &String {
        &self.id
    }

    fn get_quantity(&self) -> u32 {
        self.quantity
    }
}
