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

use std::io::Write;

use goldenfile::Mint;
use mars::BlockWorker;
use mars::Result;

use crate::common::create_config;
use crate::common::create_ctx;

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_blocks_worker() -> Result<()> {
    let mut mint = Mint::new("tests/it/testdata");
    let mut file = mint.new_goldenfile("blocks.txt").unwrap();

    let conf = create_config();
    let ctx = create_ctx(&conf);
    let range: Vec<usize> = (conf.start_block..conf.end_block + 1).collect();

    let mut worker = BlockWorker::create(&ctx);
    worker.push_batch(range)?;

    let blocks = worker.execute().await?;
    let blocks_str = serde_json::to_string(&blocks)?;
    writeln!(file, "{}", blocks_str).unwrap();

    Ok(())
}
