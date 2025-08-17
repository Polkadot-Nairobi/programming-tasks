#![cfg_attr(not(feature = "std"), no_std, no_main)]

// State machine for data
#[ink::scale_derive(Encode, Decode, TypeInfo)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, ink::storage::traits::StorageLayout)]
enum VotingStateMachine {
    #[default]
    Open,
    Active,
    Tallying,
    Finalized,
}

impl VotingStateMachine {
    const fn message(&self) -> &'static str {
        match self {
            VotingStateMachine::Open => "Voting open! Cast your vote now.",
            VotingStateMachine::Active => "Voting active...", // TODO return active votes too
            VotingStateMachine::Tallying => "Vote tallying in progress...", // TODO Return all votes
            VotingStateMachine::Finalized => "Voting finalized", // TODO
        }
    }
}

#[ink::contract]
mod voting_machine {
    use crate::VotingStateMachine;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(Default)]
    pub struct VotingMachine {
        state: VotingStateMachine,
    }

    impl VotingMachine {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn vote_state_transition(&mut self) {
            match self.state {
                VotingStateMachine::Open => self.state = VotingStateMachine::Active,
                VotingStateMachine::Active => self.state = VotingStateMachine::Tallying,
                VotingStateMachine::Tallying => self.state = VotingStateMachine::Finalized,
                _ => self.state = VotingStateMachine::Finalized,
            }
        }

        #[ink(message)]
        pub fn get(&self) -> String {
            self.state.message().to_string()
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let voting_machine = VotingMachine::new();
            assert_eq!(
                voting_machine.get(),
                VotingStateMachine::default().message()
            );
        }
    }

    // TODO /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    // ///
    // /// When running these you need to make sure that you:
    // /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    // /// - Are running a Substrate node which contains `pallet-contracts` in the background
    // #[cfg(all(test, feature = "e2e-tests"))]
    // mod e2e_tests {
    //     /// Imports all the definitions from the outer scope so we can use them here.
    //     use super::*;

    //     /// A helper function used for calling contract messages.
    //     use ink_e2e::ContractsBackend;

    //     /// The End-to-End test `Result` type.
    //     type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    //     /// We test that we can read and write a value from the on-chain contract.
    //     #[ink_e2e::test]
    //     async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
    //         // Given
    //         let mut constructor = VotingMachineRef::new(false);
    //         let contract = client
    //             .instantiate("voting_machine", &ink_e2e::bob(), &mut constructor)
    //             .submit()
    //             .await
    //             .expect("instantiate failed");
    //         let mut call_builder = contract.call_builder::<VotingMachine>();

    //         let get = call_builder.get();
    //         let get_result = client.call(&ink_e2e::bob(), &get).dry_run().await?;
    //         assert!(matches!(get_result.return_value(), false));

    //         // When
    //         let flip = call_builder.flip();
    //         let _flip_result = client
    //             .call(&ink_e2e::bob(), &flip)
    //             .submit()
    //             .await
    //             .expect("flip failed");

    //         // Then
    //         let get = call_builder.get();
    //         let get_result = client.call(&ink_e2e::bob(), &get).dry_run().await?;
    //         assert!(matches!(get_result.return_value(), true));

    //         Ok(())
    //     }
    // }
}
