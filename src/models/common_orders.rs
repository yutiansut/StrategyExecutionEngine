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
    Option,
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


    // Futures specific fields
    pub contract_symbol: Option<String>,
    pub delivery_date: Option<u64>,

    // Options specific fields
    pub strike_price: Option<f64>,
    pub option_type: Option<OptionType>,

    // Swaps specific fields
    pub fixed_rate: Option<f64>,
    pub floating_rate_index: Option<String>,
    pub notional_amount: Option<f64>,

    // Bonds specific fields
    pub bond_coupon: Option<f64>,
    pub bond_maturity_date: Option<u64>,

    // Crypto specific fields
    pub crypto_wallet_address: Option<String>,

    // RealEstate specific fields
    pub real_estate_property_id: Option<String>,
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
