# Task 2: Introduction to ink! Smart Contract Programming Language

Build and deploy a basic smart contract for a wishlist.

## 1. Storage `(#[ink(storage)])`

For this wishlist contract, the storage must contain:

- owner: An `AccountId` that stores the address of the user who initially deployed the contract. This is essential for implementing access control.
- items: A `Vec<String>` (a vector of strings) that holds the actual list of wishlisted items. This is where the core data of the contract resides.

## 2. Constructor `(#[ink(constructor)])`

The constructor must:

- Set the owner: It will get the address of the account that deployed the contract `(self.env().caller())` and save it to the owner field in the storage.
- Initialize items: It will create an empty `Vec<String>` and store it in the items field.

## 3. Messages `(#[ink(message)])`

We require two message functions:

- add_item: This function allows a user to add a new string to the items vector. It must include a crucial check to ensure that only the owner can call this function.
- get_items: This is a read-only function that returns a copy of the current items vector. It should be callable by anyone, regardless of whether they are the owner, as it does not change the contract's state.
