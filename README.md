## ChainSauce_task1
### Rust Greenfield-CMD

Rust implemintation of the [Greenfield client cmd tool](https://github.com/bnb-chain/greenfield-cmd), supporting CLI commands to make requests to Greenfield Blockchain.

### Disclaimer
**The software and related documentation are under active development, all subject to potential future change without notification and not ready for production use. The code and security audit have not been fully completed and not ready for any bug bounty. We advise you to be careful and experiment on the network at your own risk. Stay safe out there.**

### In active development
### Modules

We took greenfield proto files and Cosmos SDK proto file and generate Greenfield SDK in Rust.  

All modules was uploaded as separate repository [Greenfield Rust SDK](https://github.com/KRakenoZavr/greenfield-sdk-rust-v2) for other developers to use.


| Greenfield Module | Dev Status | Note |
| ------------- | ------------- | -----|
| Auth | 🔨|
| Authz | 🔨 |
| Bank | ✅ 🔨| can check balance |
| Tendermint | 🔨 |
| bucket | ✅ 🔨| can get bucket list |
| object |🔨|          
| group  |🔨|         
| crosschain |🔨|     
| bank |🔨|  
| policy |🔨|          
| payment |🔨|
| sp |🔨|
| create-keystore |🔨| in active development 65% done | 
| help, h | ✅ |   
---

### Supported CMD's (what is working right now)
```
#help:
./ChainSauce_task1 -h

#check balance:
./ChainSauce_task1 bank balance  

#list bucket info:
./ChainSauce_task1 bucket ls
```
---
### installation

Note: Requires Rust 1.68+
```
git clone https://github.com/ive663/ChainSauce_task1.git
cd ChainSauce
make build
cd src
./ChainSauce_task1 -h
```
### Basic Config
The command should run with "-c filePath" to load the config file and the config should be TOML format. The default config file is "config.toml".

Below is an example of the config file. The rpcAddr and chainId should be consistent with the Greenfield network. For Greenfield Testnet, you can refer to Greenfield [Testnet RPC Endpoints](https://greenfield.bnbchain.org/docs/guide/resources.html#bridge).
```
gnfd-testnet-fullnode-cosmos-us.bnbchain.org:443
```
The rpcAddr indicates the Tendermint RPC address with the port info. The configuration for passwordFile is the path to the file containing the password required to generate or parse the keystore. Users need to set the password on passwordFile before running commands and the password can be any random string.


```
config.toml
rpcAddr = "gnfd-testnet-fullnode-cosmos-us.bnbchain.org:443"
chainId = "greenfield_5600-1"
passwordFile = "password.txt"
```

### Generate Keystore (not yet working)

Before generate keystore, you should export your private key from MetaMask and write it into a local file as plaintext . You need also write your password on the password file which set by the "passwordFile" field in the config file.

Assuming that the current private key hex string is written as plaintext in the file key.txt, the following command can be used to generate a keystore file called key.json:

// generate keystore key.json
```
./ChainSauce_task1 create-keystore --privKeyFile key.txt key.json
```
After the keystore file has been generated, you can delete the private key file which contains the plaintext of private key.





### Documentation, Resources and Sources that was used in this project.

[Greenfield - gRPC Gateway docs](https://greenfield.bnbchain.org/openapi)

[gRPC-Gateway](https://grpc-ecosystem.github.io/grpc-gateway/)

[gRPC, REST, and CometBFT Endpoints](https://docs.cosmos.network/main/core/grpc_rest#grpc-server)

[Blockchain GRPC REST](https://greenfield.bnbchain.org/docs/api-sdk/grpc-rest.html)

[Cosmos-client](https://github.com/SylvestreG/cosmos-client/tree/master)

[GRPC Greenfield Cosmos Client](https://github.com/KRakenoZavr/Count/tree/grpc)

[Greenfield Rust SDK](https://github.com/KRakenoZavr/greenfield-sdk-rust-v2)

---
### Related Projects
[greenfield-cmd](https://github.com/bnb-chain/greenfield-common): Greenfield client cmd tool, supporting commands to make requests to greenfield written in Golang.

[Greenfield](https://github.com/bnb-chain/greenfield): The Golang implementation of the Greenfield Blockchain.

[Cosmos SDK](https://github.com/cosmos/cosmos-sdk): A Framework for Building High Value Public Blockchains.

[CosmosRust](https://github.com/cosmos/cosmos-rust): A Rust implemintation the Cosmos SDK.

[Greenfield-Go-SDK](https://github.com/bnb-chain/greenfield-go-sdk): The Greenfield SDK, interact with SP, Greenfield and Tendermint.

[Greenfield-Common](https://github.com/bnb-chain/greenfield-common): The Greenfield common package. Support common libs for different repos of greenfield.


---
#### MIT License
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE. Copyright (c) 2023
