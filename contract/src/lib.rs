use near_sdk::{env, near_bindgen, U128};
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Inventory {
    pub items: Vec<Item>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Item {
    pub name: String,
    pub quantity: U128,
    pub price: U128,
}

#[near_bindgen]
impl Inventory {
    pub fn new() -> Self {
        Self { items: vec![] }
    }
    pub fn add_item(&mut self, name: String, quantity: U128, price: U128) {
        assert!(
            self.get_item(name.clone()).is_some(),
            b"Item already exists"
        );
        self.items.push(Item {
            name,
            quantity,
            price,
        });
    }
    pub fn update_item_qunatity(&mut self, name: String, new_quantity: U128) {
        for item in self.items.iter_mut() {
            if item.name == name {
                item.quantity = new_quantity;
                new_quantity
            }
        }
    }

    pub fn update_item_price(&mut self, name: String, new_price: U128) {
        for item in self.items.iter_mut() {
            if item.name == name {
                item.price = new_price;
                new_price
            }
        }
    }

    pub fn get_item(&self, name: String) -> Option<Item> {
        for item in self.items.iter() {
            if item.name == name {
                Some(item.clone())
            }
        }
        None
    }
    pub fn get_all_items(&self) -> Vec<Item> {
        self.items.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(predecessor_account_id: String) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id,
            input: vec![],
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view: false,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
    }

    #[test]
    fn test_add_item() {
        let context = get_context("bob_near".to_string());
        testing_env!(context);
        let mut contract = Inventory::new();
        contract.add_item("apple".to_string(), U128(10), U128(10));
        assert_eq!(contract.get_item("apple".to_string()).unwrap().name, "apple");
    }

    #[test]
    fn test_update_item_quantity() {
        let context = get_context("bob_near".to_string());
        testing_env!(context);
        let mut contract = Inventory::new();
        contract.add_item("apple".to_string(), U128(10), U128(10));
        contract.update_item_qunatity("apple".to_string(), U128(20));
        assert_eq!(
            contract.get_item("apple".to_string()).unwrap().quantity,
            U128(20)
        );
    }

    #[test]
    fn test_update_item_price() {
        let context = get_context("bob_near".to_string());
        testing_env!(context);
        let mut contract = Inventory::new();
        contract.add_item("apple".to_string(), U128(10), U128(10));
        contract.update_item_price("apple".to_string(), U128(20));
        assert_eq!(
            contract.get_item("apple".to_string()).unwrap().price,
            U128(20)
        );
    }

    #[test]
    fn test_get_all_items() {
        let context = get
}
