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

use mars::Config;
use mars::Context;
use mars::ContextRef;

pub fn create_config() -> Config {
    let provider_uri = "https://mainnet.infura.io/v3/6e83aaa316ef4a8c947b949364f81619".to_string();
    Config {
        provider_uri,
        start_block: 50000,
        end_block: 50000,
        batch_size: 100,
        max_worker: 4,
    }
}

pub fn create_ctx(conf: &Config) -> ContextRef {
    Context::create(conf)
}
