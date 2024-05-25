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
// Declaring submodules within the clients module
pub mod common;
pub mod kafka_client;
pub mod nats_client;
pub mod rabbitmq_client;
pub mod redis_client;
pub mod zeromq_client;

// Re-exporting submodules to make them accessible from the clients module
pub use common::*;
pub use kafka_client::*;
pub use nats_client::*;
pub use rabbitmq_client::*;
pub use redis_client::*;
pub use zeromq_client::*;
