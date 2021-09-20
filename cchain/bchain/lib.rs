///Blockchain logic

use std::collections::HashMap;
use std::time::SystemTime;
use blake2::{Blake2b, Digest};


#[derive(Debug,Clone)]
pub struct Blockchain{
    pub blocks: Vec<Block>,

    pub accounts: HashMap<String, Account>,

    pending_transactions: Vec<Transaction>
    
}

pub trait WorldState {
    fn get_user_ids(&self) -> Vec<String>;
    fn get_account_by_id_mut(&mut self, id: &String) -> Option<&mut Account>; 
    fn get_account_by_id(&self, id: &String) -> Option<& Account>;
    fn create_account(&mut self, id: String, account_type: AccountType) -> Result<(),&'static str>;
    
}

#[derive(Debug,Clone)]
pub struct Block {
    pub(crate) transactions: Vec<Transaction>, 
    prev_hash: Option<String>, 
    hash: Option<String>, 
    nonce: u128, 
}

#[derive(Clone,Debug)]
pub struct Transaction{
    nonce: u128,

    from: String,

    created_at: SystemTime,

    pub(crate) record: TransactionData, 

    signature: Option<String>, 

}


#[derive(Clone, Debug, PartialEq)]
pub enum TransactionData{
    CreateUserAccount(String), 
    ChangeStoreValue {key: String, value: String},
    TransferTokens{to:String, amount:u128},
    CreateTokens{receiver: String , amount:u128},
}



#[derive(Clone,Debug)]
pub struct Account{
    
    store: HashMap<String,String>, 

    acc_type: AccountType, 

    tokens: u128,
}

#[derive(Clone,Debug)]
pub enum AccountType{
    User,
    Contract, 
    Validator{
        correctly_validated_blocks: u128,
        incorrectly_validated_blocks: u128,
        you_get_the_idea: bool,
    }
}


impl Blockchain {

    pub fn new() -> Self {
        Blockchain {
            blocks: Vec::new(),
            accounts: HashMap::new(),
            pending_transactions: Vec::new(),
        }
    }


    pub fn append_block(&mut self, block:Block) -> Result<(), String> {

        let is_genesis = self.len() == 0; 

        if !block.verify_own_hash() {
            return Err("The block is incorrect!!".into());
        }

        if !(block.prev_hash == self.get_last_block_hash()) {
            return Err("The new block has to point to the previous block".into());
        }

        let old_state = self.accounts.clone();

        for(i,transaction) in block.transactions.iter().enumerate() {
            
            if let Err(err) = transaction.execute(self, &is_genesis) {
                self.accounts = old_state;

                return Err(format!("Error {} {} ",i+1,err));
                
            }
        }

        self.blocks.push(block);

        Ok(())

    }

    pub fn len(&self) -> usize {
        self.blocks.len()
    }

    pub fn get_last_block_hash(&self) -> Option<String> {
        if self.len() == 0 {
            return None; 
        }
        self.blocks[self.len()-1].hash.clone()
    }

}


impl Block {
    pub fn new(prev_hash: Option<String>) -> Self {
        Block{
            nonce: 0,
            hash: None,
            prev_hash,
            transactions: Vec::new(),
        }
    }

    pub fn set_nonce(&mut self, nonce: u128){
        self.nonce = nonce; 
        self.update_hash();
    }

    pub fn calculate_hash(&self) -> Vec<u8> {
        let mut hasher = Blake2b::new();

        for transaction in self.transactions.iter() {
            hasher.update(transaction.calculate_hash());
        }

        let block_as_string = format!("{:?}", (&self.prev_hash, &self.nonce));
        hasher.update(&block_as_string);

        return Vec::from(hasher.finalize().as_ref());
    }

    pub fn add_transaction(&mut self, transaction: Transaction){
        self.transactions.push(transaction);
        self.update_hash();
    }

    pub fn get_transaction_count(&self) -> usize {
        self.transactions.len()
    }

    pub(crate) fn update_hash(&mut self){
        self.hash = Some(byte_vector_to_string(&self.calculate_hash()));
    }

    pub fn verify_own_hash(&self) -> bool {
        if self.hash.is_some() && self.hash.as_ref().unwrap().eq( &byte_vector_to_string( &self.calculate_hash())) {
            return true;
        }
        false
    }
}


impl Transaction {
    pub fn new(from: String, transaction_data: TransactionData, nonce: u128 ) -> Self {
        Transaction{
            from,
            nonce,
            record: transaction_data,
            created_at: SystemTime::now(),
            signature: None,
        }
    }

    pub fn execute<T: WorldState>(&self, world_state: &mut T, is_initial: &bool ) -> Result <(), &'static str> {

        if let Some(_account) = world_state.get_account_by_id(&self.from) {
        } else {
            if !is_initial {
                return Err("Account does not exists!");
            }
        }

        return match &self.record {

            TransactionData::CreateUserAccount(account) => {
                world_state.create_account (account.into(),  AccountType::User)
            }

            TransactionData::CreateTokens {receiver, amount } => {

                if !is_initial {
                    return Err("Token creation is only ava. on initial creation");
                }

                return if let Some(account) = world_state.get_account_by_id_mut(receiver){
                    account.tokens += *amount;
                    Ok(())
                }else{
                    Err("Receiver Account does not exists")
                };
            }

            TransactionData::TransferTokens { to, amount } => {
                let recv_tokens: u128; 
                let sender_tokens: u128;

                if let Some(recv) = world_state.get_account_by_id_mut(to) {
                    recv_tokens = recv.tokens;
                }else{
                    return Err("Receiver Account does not exists!");
                }

                if let Some(sender) = world_state.get_account_by_id_mut(&self.from) {
                    sender_tokens = sender.tokens;
                }else{
                    return Err("That account does not exists");
                }

                let balance_recv_new = recv_tokens.checked_add(*amount);
                let balance_sender_new = sender_tokens.checked_sub(*amount);

                if balance_recv_new.is_some() && balance_sender_new.is_some() {
                    //missing logic
                    return Ok(());
                } else {
                    return Err("Averspent or Arithmetic error");
                }
            }
            
            _ => {
                Err("Unknown transaction")
            }
        };
    }

    pub fn calculate_hash(&self) -> Vec<u8> {
        let mut hasher = Blake2b::new();
        let transaction_as_string = format!("{:?}", (&self.created_at, &self.record, &self.from, &self.nonce));
        hasher.update(&transaction_as_string);
        return Vec::from(hasher.finalize().as_ref());
    }

    pub fn check_signature(&self) -> bool {
        if !(self.is_signed()) {
            return false;
        }
        false
    }

    pub fn is_signed(&self) -> bool {
        self.signature.is_some()
    }
}

impl WorldState for Blockchain {
    fn get_user_ids(&self) -> Vec<String> {
        self.accounts.keys().map(|s| s.clone()).collect()
    }

    fn get_account_by_id_mut(&mut self, id: &String) -> Option<&mut Account>{
        self.accounts.get_mut(id)
    }

    fn get_account_by_id(& self, id: &String) -> Option<& Account> {
        self.accounts.get(id)
    }

    fn create_account(&mut self, id: String, account_type: AccountType) -> Result<(), &'static str> {
        return if !self.get_user_ids().contains(&id) {
            let acc = Account::new(account_type);
            self.accounts.insert(id,acc);
            Ok(())
        } else {
            Err("User exists!")
        };
    }
}



        

impl Account {
    pub fn new(account_type: AccountType) -> Self {
        return Self{
            tokens: 0, 
            acc_type: account_type, 
            store: HashMap::new()
        }
    }

}


fn byte_vector_to_string(arr: &Vec<u8>) -> String {
    arr.iter().map(|&c| c as char).collect()
}
