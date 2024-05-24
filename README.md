
# StrategyExecutionEngine

StrategyExecutionEngine is a Rust library for creating and executing market order strategies with integration for various messaging systems including Kafka, NATS, RabbitMQ, ZeroMQ, and Redis.

## Features

- **High Performance**: Optimized for handling high volumes of orders in real-time with low latency.
- **Real-Time Data Processing**: Ensures timely and efficient processing of parent and child orders.
- **Easy Integration**: Easily integrates with multiple messaging systems such as Kafka, NATS, RabbitMQ, ZeroMQ, and Redis for seamless order handling.
- **Flexible Strategies**: Supports various order execution strategies.

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

What things you need to install the software and how to install them:

```bash
# Example: Installation of Rust and Cargo (Rust's package manager)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Installation of Kafka (using Homebrew as an example)
brew install kafka
```

### Installing

A step-by-step series of examples that tell you how to get a development environment running:

```bash
# Clone the repository
git clone https://github.com/joaquinbejar/StrategyExecutionEngine.git

# Go into the repository
cd StrategyExecutionEngine

# Build the project
cargo build
```

### Running the tests

```bash
cargo test
```

### Deployment

Add additional notes about how to deploy this on a live system.

### Contributing

Please read CONTRIBUTING.md for details on our code of conduct, and the process for submitting pull requests to us.

### Versioning

We use [SemVer](http://semver.org/) for versioning. This approach allows us to maintain a clear, predictable system for version management. Under this scheme, version numbers are given in the format of \`MAJOR.MINOR.PATCH\`, where:

- `MAJOR` versions indicate incompatible API changes,
- `MINOR` versions add functionality in a backwards-compatible manner, and
- `PATCH` versions include backwards-compatible bug fixes.

This standard helps users and developers to understand the impact of new updates at a glance. For the versions available, see the [tags on this repository](https://github.com/joaquinbejar/StrategyExecutionEngine/tags).

### Authors

Joaquín Béjar García - Initial work - [joaquinbejar](https://github.com/joaquinbejar)

See also the list of contributors who participated in this project.

### License

This project is licensed under the MIT License - see the LICENSE.md file for details.

### Acknowledgments

Will be added in the future.
