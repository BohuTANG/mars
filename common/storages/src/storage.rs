// Copyright 2022 BohuTANG.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::env;

use common_configs::AzblobStorageConfig;
use common_configs::EthConfig;
use common_configs::FsStorageConfig;
use common_configs::S3StorageConfig;
use common_exceptions::ErrorCode;
use common_exceptions::Result;
use opendal::services::azblob;
use opendal::services::fs;
use opendal::services::s3;
use opendal::Operator;

pub async fn init_storage(conf: EthConfig) -> Result<Operator> {
    match conf.storage.storage_type.as_str() {
        "fs" => init_fs_operator(&conf.storage.fs).await,
        "s3" => init_s3_operator(&conf.storage.s3).await,
        "azure" => init_azblob_operator(&conf.storage.azblob).await,
        typ => Err(ErrorCode::Invalid(format!(
            "Unsupported storage type:{}",
            typ
        ))),
    }
}

/// init_fs_operator will init a opendal fs operator.
pub async fn init_fs_operator(cfg: &FsStorageConfig) -> Result<Operator> {
    let mut builder = fs::Backend::build();

    let mut path = cfg.data_path.clone();
    if !path.starts_with('/') {
        path = env::current_dir().unwrap().join(path).display().to_string();
    }
    builder.root(&path);

    Ok(Operator::new(builder.finish().await?))
}

/// init_s3_operator will init a opendal s3 operator with input s3 config.
pub async fn init_s3_operator(cfg: &S3StorageConfig) -> Result<Operator> {
    let mut builder = s3::Backend::build();

    // Endpoint.
    builder.endpoint(&cfg.endpoint_url);

    // Region
    builder.region(&cfg.region);

    // Credential.
    builder.access_key_id(&cfg.access_key_id);
    builder.secret_access_key(&cfg.secret_access_key);

    // Bucket.
    builder.bucket(&cfg.bucket);

    // Root.
    builder.root(&cfg.root);

    Ok(Operator::new(builder.finish().await?))
}

/// init_azblob_operator will init an opendal azblob operator.
pub async fn init_azblob_operator(cfg: &AzblobStorageConfig) -> Result<Operator> {
    let mut builder = azblob::Backend::build();

    // Endpoint
    builder.endpoint(&cfg.azblob_endpoint_url);

    // Container
    builder.container(&cfg.container);

    // Root
    builder.root(&cfg.azblob_root);

    // Credential
    builder.account_name(&cfg.account_name);
    builder.account_key(&cfg.account_key);

    Ok(Operator::new(builder.finish().await?))
}
