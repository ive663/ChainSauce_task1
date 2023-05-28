use ::cosmos_sdk_proto;
use ::cosmrs;
use ::tokio;
use ::tonic;
use cosmos_sdk_proto::cosmos::bank::v1beta1::MsgSend;
use reqwest::{Client, Version};
use tonic::{
    codegen::{http::uri::PathAndQuery, CompressionEncoding},
    IntoRequest, Request,
};

// use hyper::client::connector::HttpConnector;
// use hyper_openssl::HttpsConnector;
// use openssl::ssl::{SslMethod, SslConnector};

#[tokio::main]
async fn main() {
    // println!("Hello, world!");

    // You'd have to switch to a TLS implementation that supports ALPN such as hyper-openssl, and configure it to be turned on:

    // let mut ssl = SslConnector::builder(SslMethod::tls()).unwrap();
    // ssl.set_alpn_protos(b"\x02h2").unwrap();
    // let http = HttpConnector::new(4);
    // let connector = HttpsConnector::with_connector(http, ssl);

    // cosmrs::tx::SignerInfo

    // let asd = "str";

    // let grpc_server = tonic::client::Grpc::new(asd);

    // tonic::client::Grpc {
    //     inner:
    // }

    // let grpc_server = tonic::client::Grpc

    // let endpoint = tonic::transport::Endpoint::from_static("https://example.com");

    // let protos: &[&[u8]] = &[b"h21", b"http/1.1"];
    // let wire = protos.into_iter().flat_map(|proto| {
    //     let mut proto = proto.to_vec();
    //     let len: u8 = proto.len().try_into().expect("proto is too long");
    //     proto.insert(0, len);
    //     proto
    // }).collect::<Vec<_>>();

    // println!("{:?}", wire);

    // let rt = tokio::runtime::Builder::new_multi_thread()
    //     .enable_all()
    //     .build()
    //     .unwrap();

    // async {
    let client1 = Client::new();

    let response = client1
        .get("https://google.com")
        // Causes an runtime error without native-tls-alpn feature
        .version(Version::HTTP_2)
        .send()
        .await
        .unwrap();

    // cosmos_sdk_proto::cosmos::auth::v1beta1::query_client::QueryClient::connect(dst)

    let mut client = get_client().await;

    // client.<

    // client.

    let msg = cosmos_sdk_proto::cosmos::bank::v1beta1::QueryBalanceRequest {
        address: "0x1060D988E6b1235d1Bd0A01E6378A934b6aC763e".to_string(),
        denom: "BNB".to_string(),
    };

    // cosmos_sdk_proto::ibc::core::connection::v1::comp

    // let req = Request::new(msg);

    // MsgSend;

    // tonic::IntoRequest

    // client.send(req);

    // client.

    let res = client.balance(msg).await.unwrap();

    println!("{:?}", res);
    // };

    // rt.block_on(async {

    // Request::new(message)

    // client.create_client(req);

    // let encoding = "asd".to_string();

    // let asd = CompressionEncoding;

    // client.send_compressed(encoding);
    // })

    // client.<</

    // cosmos_sdk_proto::cosmos::tx::v1beta1::AuthInfo::

    // cosmos_sdk_proto::cosmos::base::v1beta1::Coin

    // cosmos_sdk_proto::cosmos::bank::v1beta1::Balance

    // let req = tonic::IntoRequest::into_request(self)

    // client.create_client(request)/

    // client

    // cosmos_sdk_proto::
}

async fn get_client() -> cosmos_sdk_proto::cosmos::bank::v1beta1::query_client::QueryClient<
    tonic::transport::channel::Channel,
> {
    cosmos_sdk_proto::cosmos::bank::v1beta1::query_client::QueryClient::connect(
        "https://gnfd-testnet-fullnode-tendermint-us.bnbchain.org:443",
    )
    .await
    .unwrap()
}

async fn kekwpek() {
    // let dst = "https://gnfd-testnet-fullnode-tendermint-us.bnbchain.org:443";

    // let conn = tonic::transport::Endpoint::new(dst)
    //     .unwrap()
    //     .connect()
    //     .await;

    // let server = tonic::client::Grpc::new(conn);

    // let request = cosmos_sdk_proto::cosmos::bank::v1beta1::QueryBalanceRequest {
    //     address: "0x1060D988E6b1235d1Bd0A01E6378A934b6aC763e".to_string(),
    //     denom: "BNB".to_string(),
    // };

    // server.unary(request, path, codec)

    // let codec = tonic::codec::ProstCodec::default();
    // let path = http::uri::PathAndQuery::from_static("/cosmos.bank.v1beta1.Query/Balance");

    // let path = PathAndQuery::from_static("/cosmos.bank.v1beta1.Query/Balance");

    // server.unary(request.into_request(), path, codec);
}

// use cosmos_sdk_proto::cosmos::bank::v1beta1::msg_client::MsgClient;

// // use ::cosmos_sdk_proto::ibc::core::client::v1::msg_client::MsgClient;
// // use cosmos_sdk_proto::cosmos::v1
// // use cosmos_sdk_proto::cosmos::v1beta1::msg_client::MsgClient;
// // use cosmos_sdk_proto::cosmos:🏦:v1beta1::msgclient;
// // use std::fmt::Debug;
// // use futures::Future;
// use tonic::transport::channel::Channel;
// use tonic::transport::Error;
// // use tokio::runtime::Runtime;
// use tokio::runtime::Builder;

// // struct MyWrapper(MsgClient<Channel>);

// // impl<T: Debug> Debug for MsgClient<T> {
// //     fn fmt(&self, f: &mut std::fmt::Formatter<'>) -> std::fmt::Result {
// //         write!(f, "({:?})", self)
// //     }
// // }

// fn main() {
//     // let client = MsgClient::connect("gnfd-testnet-fullnode-cosmos-us.nodereal.io:9090").await;
//     // Create a new Tokio runtime
//     let rt = Builder::new_multi_thread()
//         .enable_all()
//         .build()
//         .expect("Failed to create Tokio runtime");

//     // Execute your code within the Tokio runtime
//     rt.block_on(async {
//         let client = get_info();

//         // client

//         client.await.unwrap().send(request);

//         // client.await.unwrap().send(request);

//         // let boxed_future = Box::pin(client);
//         // let result = futures::executor::block_on(boxed_future);
//         // println!("{:?}", result.unwrap());

//         // result.unwrap().send(request)
//     });
// }

// async fn get_info() -> Result<MsgClient<Channel>, Error> {
//     MsgClient::connect("https://gnfd-testnet-fullnode-tendermint-us.bnbchain.org:443").await
// }