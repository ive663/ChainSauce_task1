## ChainSauce_task1
### Rust Greenfield-CMD

Rust implemintation of the Greenfield client cmd tool, supporting CLI commands to make requests to Greenfield Blockchain.

### Disclaimer
**The software and related documentation are under active development, all subject to potential future change without notification and not ready for production use. The code and security audit have not been fully completed and not ready for any bug bounty. We advise you to be careful and experiment on the network at your own risk. Stay safe out there.**

### Quick Start Examples
Config Examples
basic config
The command should run with "-c filePath" to load the config file and the config should be TOML format. The default config file is "config.toml".

Below is an example of the config file. The rpcAddr and chainId should be consistent with the Greenfield network. For Greenfield Testnet, you can refer to Greenfield [Testnet RPC Endpoints](https://greenfield.bnbchain.org/docs/guide/resources.html#bridge).
```
gnfd-testnet-fullnode-cosmos-us.bnbchain.org:443
gnfd-testnet-fullnode-cosmos-us.nodereal.io:9090
```
The rpcAddr indicates the Tendermint RPC address with the port info. The configuration for passwordFile is the path to the file containing the password required to generate or parse the keystore. Users need to set the password on passwordFile before running commands and the password can be any random string.



```
rpcAddr = "gnfd-testnet-fullnode-cosmos-us.bnbchain.org:443"
chainId = "greenfield_5600-1"
passwordFile = "password.txt"
```


Run Examples
Note: Requires Rust 1.68+


Usage
Reference

### Related Projects
[greenfield-cmd](https://github.com/bnb-chain/greenfield-common): Greenfield client cmd tool, supporting commands to make requests to greenfield written in Golang.

[Greenfield](https://github.com/bnb-chain/greenfield): The Golang implementation of the Greenfield Blockchain.

[Cosmos SDK](https://github.com/cosmos/cosmos-sdk): A Framework for Building High Value Public Blockchains.

[CosmosRust](https://github.com/cosmos/cosmos-rust): A Rust implemintation the Cosmos SDK.

[Greenfield-Go-SDK](https://github.com/bnb-chain/greenfield-go-sdk): The Greenfield SDK, interact with SP, Greenfield and Tendermint.

[Greenfield-Common](https://github.com/bnb-chain/greenfield-common): The Greenfield common package. Support common libs for different repos of greenfield.

### License
#### MIT License

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
Copyright (c) 2023
