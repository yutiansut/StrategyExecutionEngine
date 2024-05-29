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
   Date: 29/5/24
******************************************************************************/

#[cfg(test)]
mod orders_format_tests {
    use strategy_execution_engine::{
        ChildOrder, Futures, OptionType, Options, Order, OrderType, ParentOrder, ProductType, Side,
        Swap, TimeInForce, CFD,
    };

    #[test]
    fn test_debug_pretty_print() {
        let cfd = CFD {
            leverage: Some(10),
            margin: Some(1000.0),
            commission: Some(0.1),
            overnight_fee: Some(0.01),
            dividend_adjustment: None,
            contract_size: Some(100.0),
        };

        let debug_output = format!("{:?}", cfd);
        let expected_output = r#"{
  "leverage": 10,
  "margin": 1000.0,
  "commission": 0.1,
  "overnight_fee": 0.01,
  "dividend_adjustment": null,
  "contract_size": 100.0
}"#;

        assert_eq!(debug_output, expected_output);
    }

    #[test]
    fn test_display_single_line() {
        let cfd = CFD {
            leverage: Some(10),
            margin: Some(1000.0),
            commission: Some(0.1),
            overnight_fee: Some(0.01),
            dividend_adjustment: Some(0.02),
            contract_size: None,
        };

        let display_output = format!("{}", cfd);
        let expected_output = r#"{"leverage":10,"margin":1000.0,"commission":0.1,"overnight_fee":0.01,"dividend_adjustment":0.02,"contract_size":null}"#;

        // println!("{}", cfd);
        assert_eq!(display_output, expected_output);
    }

    #[test]
    fn test_order_debug_display() {
        let order = Order::new(
            String::from("order1"),
            100,
            ProductType::Spot,
            OrderType::Market,
            Some(3000.0),
            1622512800,
            Some(1625114800),
            String::from("AAPL"),
            Side::Buy,
            String::from("USD"),
            Some(String::from("NASDAQ")),
            Some(TimeInForce::GTC),
            None,
            None,
            None,
            None,
            Some(300000.0),
            Some(123456),
        );

        // println!("{:?}", order);

        let display_output = format!("{:?}", order);
        let expected_output = r#"{
  "id": "order1",
  "quantity": 100,
  "product_type": "Spot",
  "order_type": "Market",
  "price": 3000.0,
  "timestamp": 1622512800,
  "expiry_date": 1625114800,
  "symbol": "AAPL",
  "side": "Buy",
  "currency": "USD",
  "exchange": "NASDAQ",
  "timeinforce": "GTC",
  "futures_opt": null,
  "options_opt": null,
  "swap_opt": null,
  "cfd_opt": null,
  "notional": 300000.0,
  "nonce": 123456
}"#;

        // Test Display
        assert_eq!(display_output, expected_output);
    }

    #[test]
    fn test_order_single_line() {
        let order = Order::new(
            String::from("order1"),
            100,
            ProductType::Spot,
            OrderType::Market,
            Some(3000.0),
            1622512800,
            Some(1625114800),
            String::from("AAPL"),
            Side::Buy,
            String::from("USD"),
            Some(String::from("NASDAQ")),
            Some(TimeInForce::GTC),
            None,
            None,
            None,
            None,
            Some(300000.0),
            Some(123456),
        );

        // println!("{}", order);

        let display_output = format!("{}", order);
        let expected_output = r#"{"id":"order1","quantity":100,"product_type":"Spot","order_type":"Market","price":3000.0,"timestamp":1622512800,"expiry_date":1625114800,"symbol":"AAPL","side":"Buy","currency":"USD","exchange":"NASDAQ","timeinforce":"GTC","futures_opt":null,"options_opt":null,"swap_opt":null,"cfd_opt":null,"notional":300000.0,"nonce":123456}"#;

        // Test Display
        assert_eq!(display_output, expected_output);
    }

    #[test]
    fn test_parent_order_debug_display() {
        let parent_order = ParentOrder::new(
            String::from("parent_order1"),
            200,
            ProductType::Futures,
            OrderType::Limit,
            Some(2500.0),
            1622512800,
            Some(1625114800),
            String::from("ES"),
            Side::Sell,
            String::from("USD"),
            Some(String::from("CME")),
            Some(TimeInForce::FOK),
            None,
            None,
            None,
            None,
            Some(500000.0),
            Some(654321),
            String::from("strategy1"),
        );

        // println!("{:?}", parent_order);

        let display_output = format!("{:?}", parent_order);
        let expected_output = r#"{
  "id": "parent_order1",
  "quantity": 200,
  "product_type": "Futures",
  "order_type": "Limit",
  "price": 2500.0,
  "timestamp": 1622512800,
  "expiry_date": 1625114800,
  "symbol": "ES",
  "side": "Sell",
  "currency": "USD",
  "exchange": "CME",
  "timeinforce": "FOK",
  "futures_opt": null,
  "options_opt": null,
  "swap_opt": null,
  "cfd_opt": null,
  "notional": 500000.0,
  "nonce": 654321,
  "strategy_id": "strategy1"
}"#;

        // Test Display
        assert_eq!(display_output, expected_output);
    }

    #[test]
    fn test_parent_order_single_line() {
        let parent_order = ParentOrder::new(
            String::from("parent_order1"),
            200,
            ProductType::Futures,
            OrderType::Limit,
            Some(2500.0),
            1622512800,
            Some(1625114800),
            String::from("ES"),
            Side::Sell,
            String::from("USD"),
            Some(String::from("CME")),
            Some(TimeInForce::FOK),
            None,
            None,
            None,
            None,
            Some(500000.0),
            Some(654321),
            String::from("strategy1"),
        );

        // println!("{}", parent_order);

        let display_output = format!("{}", parent_order);
        let expected_output = r#"{"id":"parent_order1","quantity":200,"product_type":"Futures","order_type":"Limit","price":2500.0,"timestamp":1622512800,"expiry_date":1625114800,"symbol":"ES","side":"Sell","currency":"USD","exchange":"CME","timeinforce":"FOK","futures_opt":null,"options_opt":null,"swap_opt":null,"cfd_opt":null,"notional":500000.0,"nonce":654321,"strategy_id":"strategy1"}"#;

        // Test Display
        assert_eq!(display_output, expected_output);
    }

    #[test]
    fn test_child_order_debug_display() {
        let child_order = ChildOrder::new(
            String::from("child_order1"),
            50,
            ProductType::Options,
            OrderType::Market,
            Some(1500.0),
            1622512800,
            Some(1625114800),
            String::from("GOOGL"),
            Side::Buy,
            String::from("USD"),
            Some(String::from("NYSE")),
            Some(TimeInForce::IOC),
            None,
            None,
            None,
            None,
            Some(75000.0),
            Some(789012),
            String::from("parent_order2"),
            "parent_order2".to_string(),
            None,
        );

        // println!("{:?}", child_order);

        let display_output = format!("{:?}", child_order);
        let expected_output = r#"{
  "id": "child_order1",
  "quantity": 50,
  "product_type": "Options",
  "order_type": "Market",
  "price": 1500.0,
  "timestamp": 1622512800,
  "expiry_date": 1625114800,
  "symbol": "GOOGL",
  "side": "Buy",
  "currency": "USD",
  "exchange": "NYSE",
  "timeinforce": "IOC",
  "futures_opt": null,
  "options_opt": null,
  "swap_opt": null,
  "cfd_opt": null,
  "notional": 75000.0,
  "nonce": 789012,
  "strategy_id": "parent_order2",
  "parent_id": "parent_order2",
  "insert_at": null
}"#;

        // Test Display
        assert_eq!(display_output, expected_output);
    }

    #[test]
    fn test_child_order_single_line() {
        let child_order = ChildOrder::new(
            String::from("child_order1"),
            50,
            ProductType::Options,
            OrderType::Market,
            Some(1500.0),
            1622512800,
            Some(1625114800),
            String::from("GOOGL"),
            Side::Buy,
            String::from("USD"),
            Some(String::from("NYSE")),
            Some(TimeInForce::IOC),
            None,
            None,
            None,
            None,
            Some(75000.0),
            Some(789012),
            String::from("parent_order2"),
            "parent_order2".to_string(),
            None,
        );

        // println!("{}", child_order);

        let display_output = format!("{}", child_order);
        let expected_output = r#"{"id":"child_order1","quantity":50,"product_type":"Options","order_type":"Market","price":1500.0,"timestamp":1622512800,"expiry_date":1625114800,"symbol":"GOOGL","side":"Buy","currency":"USD","exchange":"NYSE","timeinforce":"IOC","futures_opt":null,"options_opt":null,"swap_opt":null,"cfd_opt":null,"notional":75000.0,"nonce":789012,"strategy_id":"parent_order2","parent_id":"parent_order2","insert_at":null}"#;

        // Test Display
        assert_eq!(display_output, expected_output);
    }

    #[test]
    fn test_futures_debug_display() {
        let futures = Futures {
            delivery_date: Some(20240101),
            contract_size: Some(100.0),
            margin: Some(0.05),
            commission: Some(0.02),
            overnight_fee: Some(0.01),
        };

        // println!("{:?}", futures);

        let display_output = format!("{:?}", futures);
        let expected_output = r#"{
  "delivery_date": 20240101,
  "contract_size": 100.0,
  "margin": 0.05,
  "commission": 0.02,
  "overnight_fee": 0.01
}"#;

        // Test Display
        assert_eq!(display_output, expected_output);
    }

    #[test]
    fn test_futures_single_line() {
        let futures = Futures {
            delivery_date: Some(20240101),
            contract_size: Some(100.0),
            margin: Some(0.05),
            commission: Some(0.02),
            overnight_fee: Some(0.01),
        };

        // println!("{}", futures);

        let display_output = format!("{}", futures);
        let expected_output = r#"{"delivery_date":20240101,"contract_size":100.0,"margin":0.05,"commission":0.02,"overnight_fee":0.01}"#;

        // Test Display
        assert_eq!(display_output, expected_output);
    }

    #[test]
    fn test_options_debug_display() {
        let options = Options {
            strike_price: 2000.0,
            option_type: OptionType::Call,
            expiry_date: 20241231,
        };

        // println!("{:?}", options);

        let display_output = format!("{:?}", options);
        let expected_output = r#"{
  "strike_price": 2000.0,
  "option_type": "Call",
  "expiry_date": 20241231
}"#;

        // Test Display
        assert_eq!(display_output, expected_output);
    }

    #[test]
    fn test_options_single_line() {
        let options = Options {
            strike_price: 2000.0,
            option_type: OptionType::Call,
            expiry_date: 20241231,
        };

        // println!("{}", options);

        let display_output = format!("{}", options);
        let expected_output =
            r#"{"strike_price":2000.0,"option_type":"Call","expiry_date":20241231}"#;

        // Test Display
        assert_eq!(display_output, expected_output);
    }

    #[test]
    fn test_swap_debug_display() {
        let swap = Swap {
            fixed_rate: 1.5,
            floating_rate_index: String::from("LIBOR"),
            notional_amount: 1000000.0,
        };

        // println!("{:?}", swap);

        let display_output = format!("{:?}", swap);
        let expected_output = r#"{
  "fixed_rate": 1.5,
  "floating_rate_index": "LIBOR",
  "notional_amount": 1000000.0
}"#;

        // Test Display
        assert_eq!(display_output, expected_output);
    }

    #[test]
    fn test_swap_single_line() {
        let swap = Swap {
            fixed_rate: 1.5,
            floating_rate_index: String::from("LIBOR"),
            notional_amount: 1000000.0,
        };

        // println!("{}", swap);

        let display_output = format!("{}", swap);
        let expected_output =
            r#"{"fixed_rate":1.5,"floating_rate_index":"LIBOR","notional_amount":1000000.0}"#;

        // Test Display
        assert_eq!(display_output, expected_output);
    }
}
