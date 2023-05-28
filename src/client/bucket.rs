use greenfield_sdk_proto::greenfield::storage::QueryListBucketsResponse;
use prost::Message;
use std::rc::Rc;
use tendermint_rpc::{Client, HttpClient};

pub struct BucketModule {
    rpc: Rc<HttpClient>,
}

impl BucketModule {
    pub fn new(rpc: Rc<HttpClient>) -> Self {
        BucketModule { rpc }
    }

    pub async fn list_bucket(&self) -> Result<QueryListBucketsResponse, anyhow::Error> {
        let query = self
            .rpc
            .abci_query(
                Some("/greenfield.storage.Query/ListBuckets".to_string()),
                vec![],
                None,
                false,
            )
            .await
            .unwrap();

        let resp = QueryListBucketsResponse::decode(query.value.as_slice())?;
        Ok(resp)
    }
}
