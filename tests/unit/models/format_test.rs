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
    use strategy_execution_engine::CFD;

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
        // println!("{:?}", cfd);
        // println!("{}", cfd);
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

        assert_eq!(display_output, expected_output);
        // println!("{:?}", cfd);
        // println!("{}", cfd);
    }
}
