# Scout Soroban Smart Contract Examples

![https://img.shields.io/badge/license-MIT-green](https://img.shields.io/badge/license-MIT-green)


## Project Overview

Welcome to the Scout Soroban Smart Contract Examples repository. This project is dedicated to advancing the development, security, and understanding of smart contracts on the Stellar network through the [Soroban smart contract language](https://soroban.stellar.org/docs). 

Our aim is to create a set of real-life, security-reviewed Soroban smart contracts that not only serve as robust development templates but also help identify and document good and bad practices in smart contract development. This initiative is designed to uncover new vulnerabilities, significantly contributing to the improvement of the [Scout](https://github.com/CoinFabrik/scout-soroban) tool for Soroban.

By engaging developers with varying levels of experience in a time-constrained environment, we intentionally mimic real-world conditions under which smart contracts are developed. Our contracts are analyzed using [Scout](https://github.com/CoinFabrik/scout-soroban) and reviewed by senior security auditors to identify and rectify any undetected security issues, ensuring a comprehensive security review process.

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
2. **Test the Contract**: Compile and run all the tests.
   
	```console
	cargo test
	```
 
### Deploying on Local Node

For deploying Soroban smart contracts on a local node follow these steps:

1. **Execute Standalone Network**: Run a local standalone network with the Stellar Quickstart Docker image.

	```console
	docker run --rm -it \
	-p 8000:8000 \
	--name stellar \
	stellar/quickstart:testing \
	--standalone \
	--enable-soroban-rpc
	```

2. **Configure Soroban Network**: Set up your connection to the Soroban standalone network.

	```console
	soroban config network add standalone \
   	--rpc-url "http://localhost:8000/soroban/rpc" \
   	--network-passphrase "Standalone Network ; February 2017"
	```
3. **Create Identities**: Generate the necessary identities.

	```console
	soroban config identity generate --global [name]
	```
4. **Fund Identities**: Fund identities so they can be used as accounts for contract calls.

 	```console
  	soroban config identity fund [name] --network standalone
  	```

5. **Deploy the Contract**: Deploy the compiled contract to your chosen network.

	```console
	soroban contract deploy --wasm [path_to_wasm_file] --source [name] --network standalone
	```

 	_Deploying the contract will output the contract's address. For example: 						`CBB7KJK37V26SL3BGPMFPU3LT2QH53VQ4KVQCR6LJSSA3FALMA2OHMR2`_

	_For convenience, save it to an environment variable_

	```console
 	CONTRACT=[address]
 	```

#### In case you need to use a token, follow these instructions:

1. **Wrap the Native Token**: To be able to use tokens in contract calls, we'll need to obtain an address.

   	```console
	soroban lab token wrap --asset native --network standalone --source [name]
	```
    
   	_For convenience, save it to an environment variable_

	```console
 	TOKEN=[returned address]
 	```

 2. **Check Balances**: To check the balance of an identity:

 	```console
	soroban contract invoke --id $TOKEN --source [name] --network standalone -- balance --id [name]
	```

3. **Token Usage**: Now you can pass a token as a parameter to contract calls.
  
   	```console
	soroban contract invoke --id $CONTRACT --source [name] --network standalone -- [function_name] --token $TOKEN
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



