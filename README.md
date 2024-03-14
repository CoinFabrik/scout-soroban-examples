# Scout Soroban Smart Contract Examples

## Project Overview

Welcome to the Scout Soroban Smart Contract Examples repository. This project is dedicated to advancing the development, security, and understanding of smart contracts on the Stellar network through the Soroban smart contract language. Our aim is to create a set of real-life, security-reviewed Soroban smart contracts that not only serve as robust development templates but also help identify and document good and bad practices in smart contract development. Furthermore, this initiative is designed to uncover new vulnerabilities, significantly contributing to the improvement of the Scout tool for Soroban.

By engaging developers with varying levels of experience in a time-constrained environment, we intentionally mimic real-world conditions under which smart contracts are developed. These contracts are subsequently reviewed by senior security auditors and analyzed using the Scout for Soroban tool to identify and rectify any security issues, ensuring a comprehensive security review process.

### Project Objectives

The project is structured around several key objectives:

1. **To enhance the Scout Soroban Tool** by using these contracts as real-world test cases, thereby refining the tool’s capabilities in vulnerability detection and prevention.
2. **To explore complex vulnerabilities** not covered in the proof of concept (PoC) and prototype phases, facilitating the development of new vulnerability classes, test cases, and detectors.
3. **To use the security findings** from the review process to further refine and improve the Scout tool, contributing to safer and more secure Soroban smart contract development.

## Getting Started

### Initial Environment Setup

To modify or interact with the smart contracts within, ensure your development environment is set up as follows:

1. **Install Soroban CLI and Rust SDK**: Follow the setup instructions provided in the [Soroban documentation](https://soroban.stellar.org/docs/getting-started/setup).
2. **Install Docker**: Refer to the [Docker installation guide](https://docs.docker.com/get-docker/).

### Building and Testing Contracts

General steps for building and testing contracts:

1. **Build the Contract**: Compile the smart contract to a WASM file.

	```console
	soroban contract build
	```
2. **Test de Contract**: [COMPLETE]

### Deploying on Local Node

For deploying Soroban smart contracts on a local node follow these steps:

1. **Configure Soroban Network**: Set up your connection to the Soroban standalone network.

	```console
	soroban config network add standalone \
   	--rpc-url "http://localhost:8000/soroban/rpc" \
   	--network-passphrase "Standalone Network ; February 2017"
	```

2. **Deploy the Contract**: Deploy the compiled contract to your chosen network.

	```console
	soroban contract deploy --wasm [path_to_wasm_file] --source [your_username] --network standalone
	```

### Security Review Process

All smart contracts featured in this repository will undergo an extensive security review conducted by senior auditors from [CoinFabrik](https://www.coinfabrik.com/) in April 2024. This  process ensures that each contract not only adheres to best practices in smart contract development but also is scrutinized for vulnerabilities, which are then corrected. A detailed security review report for each contract will be made publicly available, contributing to the transparency and educational value of this project.

## About Soroban

[Soroban](https://soroban-lang.github.io/) is a domain-specific language designed for smart contract development on blockchain platforms. It's built on top of Rust, leveraging its power while providing abstractions and tools tailored specifically for smart contract development. Soroban simplifies the process of writing, testing, and deploying smart contracts, abstracting away complex blockchain interactions and ensuring correctness and security.

Learn more about Soroban and its features at [Soroban Documentation](https://soroban.stellar.org/docs/).

## About CoinFabrik

We - [CoinFabrik](https://www.coinfabrik.com/) - are a research and development company specialized in Web3, with a strong background in cybersecurity. Founded in 2014, we have worked on over 180 blockchain-related projects, EVM based and also for Solana, Algorand, and Polkadot. Beyond development, we offer security audits through a dedicated in-house team of senior cybersecurity professionals, currently working on code in Substrate, Solidity, Clarity, Rust, and TEAL.

Our team has an academic background in computer science and mathematics, with work experience focused on cybersecurity and software development, including academic publications, patents turned into products, and conference presentations. Furthermore, we have an ongoing collaboration on knowledge transfer and open-source projects with the University of Buenos Aires.


## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.



