use {
    crate::{
        error::{Error, Result},
        keys,
        types::{Block, TransactionStatus},
        Receipt,
    },
    primitive_types::{H160, H256, U256},
    redis::{Commands, ConnectionLike},
    redis_versioned_kv::VersionedKVCommand,
};

pub struct Setter<C> {
    conn: C,
    pub prefix: String,
}

impl<C: ConnectionLike> Setter<C> {
    pub fn new(conn: C, prefix: String) -> Self {
        Self { conn, prefix }
    }

    pub fn clear(&mut self) -> Result<()> {
        redis::cmd("FLUSHALL")
            .arg("SYNC")
            .query(&mut self.conn)
            .map_err(|e| Error::RedisError(e))?;
        Ok(())
    }

    pub fn set_height(&mut self, height: u32) -> Result<()> {
        let height_key = keys::latest_height_key(&self.prefix);
        self.conn.set(height_key, format!("{}", height))?;
        Ok(())
    }

    pub fn set_balance(&mut self, height: u32, address: H160, balance: U256) -> Result<()> {
        let balance_key = keys::balance_key(&self.prefix, address);
        self.conn
            .vkv_set(balance_key, height, serde_json::to_string(&balance)?)
            .map_err(|e| Error::RedisError(e))?;

        Ok(())
    }

    pub fn set_nonce(&mut self, height: u32, address: H160, nonce: U256) -> Result<()> {
        let nonce_key = keys::nonce_key(&self.prefix, address);
        self.conn
            .vkv_set(nonce_key, height, serde_json::to_string(&nonce)?)
            .map_err(|e| Error::RedisError(e))?;

        Ok(())
    }

    pub fn set_byte_code(&mut self, height: u32, address: H160, code: Vec<u8>) -> Result<()> {
        let code_key = keys::code_key(&self.prefix, address);
        self.conn
            .vkv_set(code_key, height, hex::encode(&code))
            .map_err(|e| Error::RedisError(e))?;

        Ok(())
    }

    pub fn set_state(
        &mut self,
        height: u32,
        address: H160,
        index: H256,
        value: H256,
    ) -> Result<()> {
        let key = keys::state_key(&self.prefix, address, index);
        self.conn
            .vkv_set(key, height, serde_json::to_string(&value)?)
            .map_err(|e| Error::RedisError(e))?;
        Ok(())
    }

    pub fn set_block_info(
        &mut self,
        block: Block,
        receipts: Vec<Receipt>,
        statuses: Vec<TransactionStatus>,
    ) -> Result<()> {
        let block_hash = block.header.hash();
        let height = block.header.number;

        let block_hash_key = keys::block_hash_key(&self.prefix, height);
        self.conn
            .set(block_hash_key, serde_json::to_string(&block_hash)?)
            .map_err(|e| Error::RedisError(e))?;

        let block_height_key = keys::block_height_key(&self.prefix, block_hash);
        self.conn
            .set(block_height_key, serde_json::to_string(&height)?)
            .map_err(|e| Error::RedisError(e))?;

        let block_key = keys::block_key(&self.prefix, block_hash);
        self.conn
            .set(block_key, serde_json::to_string(&block)?)
            .map_err(|e| Error::RedisError(e))?;

        let receipt_key = keys::receipt_key(&self.prefix, block_hash);
        self.conn
            .set(receipt_key, serde_json::to_string(&receipts)?)
            .map_err(|e| Error::RedisError(e))?;

        let status_key = keys::status_key(&self.prefix, block_hash);
        self.conn
            .set(status_key, serde_json::to_string(&statuses)?)
            .map_err(|e| Error::RedisError(e))?;

        for (i, tx) in block.transactions.iter().enumerate() {
            let transaction_index_key = keys::transaction_index_key(&self.prefix, tx.hash());
            self.conn
                .set(
                    transaction_index_key,
                    serde_json::to_string(&(block_hash, i as u32))?,
                )
                .map_err(|e| Error::RedisError(e))?;
        }
        Ok(())
    }
}