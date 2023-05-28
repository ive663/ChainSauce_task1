use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde()]
enum BucketVisibilityTypes {
    #[default]
    VISIBILITY_TYPE_UNSPECIFIED,
    VISIBILITY_TYPE_PUBLIC_READ,
    VISIBILITY_TYPE_PRIVATE,
    VISIBILITY_TYPE_INHERIT,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde()]
enum BucketSourceTypes {
    #[default]
    SOURCE_TYPE_ORIGIN,
    SOURCE_TYPE_BSC_CROSS_CHAIN,
    SOURCE_TYPE_MIRROR_PENDING,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde()]
enum BucketStatusType {
    #[default]
    BUCKET_STATUS_CREATED,
    BUCKET_STATUS_DISCONTINUED,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde()]
pub struct SecondarySpObject {
    sp_address: String,
    total_charge_size: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde()]
pub struct BillingInfo {
    price_time: String,
    total_charge_size: String,
    secondary_sp_objects_size: Vec<SecondarySpObject>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde()]
pub struct BucketInfo {
    owner: String,
    bucket_name: String,
    visibility: BucketVisibilityTypes,
    id: String,
    source_type: BucketSourceTypes,
    create_at: String,
    payment_address: String,
    primary_sp_address: String,
    charged_read_quota: String,
    billing_info: BillingInfo,
    bucket_status: BucketStatusType,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde()]
pub struct BucketInfos {
    bucket_infos: Vec<BucketInfo>,
}

pub async fn get_list_bucket() -> BucketInfos {
    let url =
        "https://gnfd-testnet-fullnode-tendermint-us.bnbchain.org/greenfield/storage/list_buckets";

    let client = Client::new();

    let response = client.get(url).send().await.unwrap();

    let jsoned = response.json::<BucketInfos>().await.unwrap();

    println!("{:#?}", jsoned);

    jsoned
}
