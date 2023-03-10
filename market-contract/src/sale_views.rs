use crate::*;

#[near_bindgen]
impl Contract {
    //returns the number of sales the marketplace has up (as a string)
    pub fn get_supply_sales(&self) -> U64 {
        U64(self.sales.len())
    }

    //returns the number of sales for a given account (result is a string)
    pub fn get_supply_by_owner_id(&self, account_id: AccountId) -> U64 {
        let by_owner_id = self.by_owner_id.get(&account_id);

        if let Some(by_owner_id) = by_owner_id {
            U64(by_owner_id.len())
        } else {
            U64(0)
        }
    }

    //returns paginated sale objects for a given account. (result is a vector of sales)
    pub fn get_sales_by_owner_id(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<Sale> {
        let by_owner_id = self.by_owner_id.get(&account_id);
        let sales = if let Some(by_owner_id) = by_owner_id {
            by_owner_id
        } else {
            return vec![];
        };

        let keys = sales.as_vector();

        let start = u128::from(from_index.unwrap_or(U128(0)));

        keys.iter()
            .skip(start as usize)
            .take(limit.unwrap_or(0) as usize)
            .map(|token_id| self.sales.get(&token_id).unwrap())
            .collect()
    }

    //get the number of sales for an nft contract. (returns a string)
    pub fn get_supply_by_nft_contract_id(&self, nft_contract_id: AccountId) -> U64 {
        let by_nft_contract_id = self.by_nft_contract_id.get(&nft_contract_id);

        if let Some(by_nft_contract_id) = by_nft_contract_id {
            U64(by_nft_contract_id.len())
        } else {
            U64(0)
        }
    }

    //returns paginated sale objects associated with a given nft contract. (result is a vector of sales)
    pub fn get_sales_by_nft_contract_id(
        &self,
        nft_contract_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<Sale> {
        let by_nft_contract_id = self.by_nft_contract_id.get(&nft_contract_id);

        let sales = if let Some(by_nft_contract_id) = by_nft_contract_id {
            by_nft_contract_id
        } else {
            return vec![];
        };

        let keys = sales.as_vector();

        let start = u128::from(from_index.unwrap_or(U128(0)));

        keys.iter()
            .skip(start as usize)
            .take(limit.unwrap_or(0) as usize)
            .map(|token_id| {
                self.sales
                    .get(&format!("{}{}{}", nft_contract_id, DELIMETER, token_id))
                    .unwrap()
            })
            .collect()
    }

    //get a sale information for a given unique sale ID (contract + DELIMITER + token ID)
    pub fn get_sale(&self, nft_contract_token: ContractAndTokenId) -> Option<Sale> {
        self.sales.get(&nft_contract_token)
    }
}
