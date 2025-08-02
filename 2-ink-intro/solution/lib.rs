#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

use ink_lang as ink;

#[ink::contract]
pub mod wishlist {
    use ink_storage::collections::Vec as StorageVec;

    /// The wishlist contract.
    #[ink(storage)]
    #[derive(Default)]
    pub struct Wishlist {
        /// The owner of the wishlist (the account that deployed it).
        /// Only the owner can modify the wishlist.
        owner: AccountId,
        
        /// The list of items in the wishlist.
        /// Stored as a vector of strings in the contract's storage.
        items: StorageVec<String>,
    }

    /// Custom error types for our contract operations
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Returned if a non-owner tries to modify the wishlist
        NotOwner,
        /// Returned if someone tries to add an empty item
        EmptyItem,
    }

    /// Result type for our contract operations
    pub type Result<T> = core::result::Result<T, Error>;

    impl Wishlist {
        /// Constructor that initializes the contract.
        /// 
        /// This function runs only once when the contract is deployed.
        /// It sets the owner to the account that deployed the contract
        /// and initializes an empty wishlist.
        #[ink(constructor)]
        pub fn new() -> Self {
            // Get the AccountId of whoever is deploying the contract
            // This will be stored as the owner who has special permissions
            let owner = Self::env().caller();
            
            // Initialize the contract with an empty wishlist
            Self {
                owner,
                items: StorageVec::new(),
            }
        }

        /// Adds an item to the wishlist.
        /// 
        /// This is a state-changing function that modifies the contract storage.
        /// Only the owner of the contract can call this function.
        /// 
        /// # Arguments
        /// * `item` - A string representing the item to add to the wishlist
        /// 
        /// # Errors
        /// * `NotOwner` - If the caller is not the owner of the contract
        /// * `EmptyItem` - If the provided item is an empty string
        #[ink(message)]
        pub fn add_item(&mut self, item: String) -> Result<()> {
            // Get the AccountId of whoever is calling this function
            let caller = self.env().caller();
            
            // Verify that the caller is the owner of the contract
            // This access control mechanism ensures only the deployer can modify
            if caller != self.owner {
                return Err(Error::NotOwner);
            }
            
            // Validate that the item is not empty
            // This prevents adding meaningless entries to the wishlist
            if item.is_empty() {
                return Err(Error::EmptyItem);
            }
            
            // Add the item to our wishlist vector
            // This modifies the contract's storage, which costs gas to execute
            self.items.push(item);
            
            // Return success result
            Ok(())
        }

        /// Retrieves all items from the wishlist.
        /// 
        /// This is a read-only function that returns a copy of the wishlist.
        /// Anyone can call this function as it doesn't modify the contract state.
        /// 
        /// # Returns
        /// A vector containing all wishlist items
        #[ink(message)]
        pub fn get_items(&self) -> Vec<String> {
            // Return a copy of all items in the wishlist
            // Note: We return Vec<String> rather than &StorageVec<String>
            // because we need to move the data out of storage
            self.items.iter().cloned().collect()
        }

        /// Returns the owner of the wishlist.
        /// 
        /// This is a read-only function that allows anyone to check who owns the wishlist.
        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner
        }

        /// Returns the number of items in the wishlist.
        /// 
        /// This is a utility function that provides information about the wishlist size.
        #[ink(message)]
        pub fn get_item_count(&self) -> u32 {
            self.items.len() as u32
        }
    }

    /// Unit tests for our wishlist contract
    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn new_wishlist_has_correct_owner() {
            // Arrange - Get the default accounts provided by the test environment
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();
            
            // Act - Set Alice as the caller and create a new wishlist
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(accounts.alice);
            let wishlist = Wishlist::new();
            
            // Assert - The owner should be Alice's account
            assert_eq!(wishlist.get_owner(), accounts.alice);
        }

        #[ink::test]
        fn owner_can_add_item() {
            // Arrange - Set up accounts and create a wishlist
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(accounts.alice);
            let mut wishlist = Wishlist::new();
            
            // Act - Try to add an item as the owner
            let result = wishlist.add_item(String::from("New Laptop"));
            
            // Assert - Operation should succeed
            assert_eq!(result, Ok(()));
            assert_eq!(wishlist.get_item_count(), 1);
            assert_eq!(wishlist.get_items(), vec!["New Laptop"]);
        }

        #[ink::test]
        fn non_owner_cannot_add_item() {
            // Arrange - Create a wishlist with Alice as owner
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(accounts.alice);
            let mut wishlist = Wishlist::new();
            
            // Act - Try to add an item as Bob (not the owner)
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(accounts.bob);
            let result = wishlist.add_item(String::from("Forbidden Item"));
            
            // Assert - Should fail with NotOwner error
            assert_eq!(result, Err(Error::NotOwner));
            assert_eq!(wishlist.get_item_count(), 0);
        }

        #[ink::test]
        fn anyone_can_get_items() {
            // Arrange - Create a wishlist and add some items
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(accounts.alice);
            let mut wishlist = Wishlist::new();
            let _ = wishlist.add_item(String::from("Item 1"));
            let _ = wishlist.add_item(String::from("Item 2"));
            
            // Act - Bob (not the owner) tries to read items
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(accounts.bob);
            let items = wishlist.get_items();
            
            // Assert - Bob should be able to see the items
            assert_eq!(items, vec!["Item 1", "Item 2"]);
        }

        #[ink::test]
        fn cannot_add_empty_item() {
            // Arrange - Create a wishlist
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(accounts.alice);
            let mut wishlist = Wishlist::new();
            
            // Act - Try to add an empty item
            let result = wishlist.add_item(String::from(""));
            
            // Assert - Should fail with EmptyItem error
            assert_eq!(result, Err(Error::EmptyItem));
            assert_eq!(wishlist.get_item_count(), 0);
        }
    }
}