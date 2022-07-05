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

use web3::types::Block;
use web3::types::BlockId;
use web3::types::BlockNumber;
use web3::types::Transaction;
use web3::types::U64;

use crate::exceptions::Result;
use crate::workers::ContextRef;
use crate::ErrorCode;

pub struct BlockWorker {
    ctx: ContextRef,
    numbers: Vec<usize>,
}

impl BlockWorker {
    pub fn create(ctx: &ContextRef) -> BlockWorker {
        Self {
            ctx: ctx.clone(),
            numbers: vec![],
        }
    }

    // Push a block number.
    pub fn push(&mut self, number: usize) -> Result<()> {
        self.numbers.push(number);
        Ok(())
    }

    // Push range of block numbers.
    pub fn push_batch(&mut self, nums: Vec<usize>) -> Result<()> {
        self.numbers.extend(nums);
        Ok(())
    }

    // Get the blocks.
    #[tracing::instrument(level = "info", skip(self))]
    pub async fn execute(&self) -> Result<Vec<Block<Transaction>>> {
        let http = web3::transports::Http::new(self.ctx.get_rpc_url())?;
        let web3 = web3::Web3::new(web3::transports::Batch::new(http));

        let mut blocks = vec![];

        let mut callbacks = vec![];
        for num in &self.numbers {
            let block = web3
                .eth()
                .block_with_txs(BlockId::Number(BlockNumber::Number(U64::from(*num))));
            callbacks.push(block);
        }
        let _ = web3.transport().submit_batch().await?;

        // Get the callback.
        for cb in callbacks {
            let r = cb.await?;
            match r {
                None => {
                    return Err(ErrorCode::ExportBlockError(
                        "Cannot export block, please make sure eth node sync is already",
                    ));
                }
                Some(blk) => {
                    let len = blk.transactions.len();
                    blocks.push(blk);

                    self.ctx.get_progress().incr_blocks(1);
                    self.ctx.get_progress().incr_txs(len);
                }
            }
        }

        Ok(blocks)
    }
}
