#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

use ink_lang as ink;

#[ink::contract]
pub mod voting_contract {
    use ink_storage::{
        collections::{HashMap as StorageHashMap, Vec as StorageVec},
        traits::{SpreadAllocate, Packed},
    };
    
    /// Enum representing the different states of the voting process
    #[derive(scale::Encode, scale::Decode, Debug, PartialEq, Eq, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum VotingState {
        RegistrationOpen,
        VotingActive,
        ResultsFinalized,
    }

    /// Custom error types
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Caller is not authorized (not admin)
        NotAuthorized,
        /// Voting state transition is not allowed
        InvalidStateTransition,
        /// Voter is already registered
        AlreadyRegistered,
        /// Voter is not registered
        NotRegistered,
        /// Voter has already cast a vote
        AlreadyVoted,
        /// Voting period has not started or has ended
        VotingPeriodNotActive,
        /// Invalid voting duration
        InvalidDuration,
        /// Invalid vote option
        InvalidVoteOption,
        /// Voting has already ended
        VotingAlreadyEnded,
    }

    /// Result type for contract operations
    pub type Result<T> = core::result::Result<T, Error>;

    /// Vote options available in the voting process
    #[derive(scale::Encode, scale::Decode, Debug, PartialEq, Eq, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum VoteOption {
        OptionA,
        OptionB,
    }

    /// Defines the storage of our contract
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct VotingContract {
        /// Current state of the voting process
        current_voting_state: VotingState,
        
        /// Collection to track unique voter identifiers
        registered_voters: StorageVec<AccountId>,
        
        /// Collection to record which voters have already cast a vote
        votes_cast: StorageVec<AccountId>,
        
        /// Map to store the tally for each voting option
        vote_counts: StorageHashMap<VoteOption, u32>,
        
        /// Timestamps to define the active voting period
        voting_start_time: Timestamp,
        voting_end_time: Timestamp,
        
        /// Administrator identifier
        admin_id: AccountId,
        
        /// Block number when registration started (for time calculation)
        registration_start_block: BlockNumber,
    }

    impl VotingContract {
        /// Constructor to initialize our contract
        #[ink(constructor)]
        pub fn new() -> Self {
            // This will initialize all storage fields with default values
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                // Set the initial state to RegistrationOpen
                contract.current_voting_state = VotingState::RegistrationOpen;
                
                // Initialize vote counts for both options
                contract.vote_counts.insert(&VoteOption::OptionA, &0);
                contract.vote_counts.insert(&VoteOption::OptionB, &0);
                
                // Set the admin to the caller of this constructor
                contract.admin_id = Self::env().caller();
                
                // Initialize time values
                contract.voting_start_time = 0;
                contract.voting_end_time = 0;
                contract.registration_start_block = 0;
            })
        }

        /// Returns the current voting state
        #[ink(message)]
        pub fn get_current_state(&self) -> VotingState {
            self.current_voting_state.clone()
        }

        /// Returns whether a voter is registered
        #[ink(message)]
        pub fn is_registered(&self, voter: AccountId) -> bool {
            self.registered_voters.iter().any(|v| *v == voter)
        }

        /// Returns whether a voter has cast their vote
        #[ink(message)]
        pub fn has_voted(&self, voter: AccountId) -> bool {
            self.votes_cast.iter().any(|v| *v == voter)
        }

        /// Returns the vote count for a specific option
        #[ink(message)]
        pub fn get_vote_count(&self, option: VoteOption) -> u32 {
            *self.vote_counts.get(&option).unwrap_or(&0)
        }

        /// Returns the start time of voting
        #[ink(message)]
        pub fn get_voting_start_time(&self) -> Timestamp {
            self.voting_start_time
        }

        /// Returns the end time of voting
        #[ink(message)]
        pub fn get_voting_end_time(&self) -> Timestamp {
            self.voting_end_time
        }

        /// Returns the admin ID
        #[ink(message)]
        pub fn get_admin_id(&self) -> AccountId {
            self.admin_id
        }

        /// Only callable by admin_id. Must not already be in RegistrationOpen state.
        /// Sets current_voting_state to RegistrationOpen. Clears previous voter and vote data.
        #[ink(message)]
        pub fn start_registration(&mut self) -> Result<()> {
            let caller = self.env().caller();
            
            // Check if the caller is the admin
            if caller != self.admin_id {
                return Err(Error::NotAuthorized);
            }
            
            // Reset all data
            self.registered_voters.clear();
            self.votes_cast.clear();
            *self.vote_counts.get_mut(&VoteOption::OptionA).unwrap() = 0;
            *self.vote_counts.get_mut(&VoteOption::OptionB).unwrap() = 0;
            
            // Set state to RegistrationOpen
            self.current_voting_state = VotingState::RegistrationOpen;
            
            // Record registration start time
            self.registration_start_block = self.env().block_number();
            
            Ok(())
        }

        /// Only valid if current_voting_state is RegistrationOpen. 
        /// caller_id must not already be registered. Adds caller_id to registered_voters.
        #[ink(message)]
        pub fn register_voter(&mut self) -> Result<()> {
            let caller = self.env().caller();
            
            // Check if we're in the correct state
            if self.current_voting_state != VotingState::RegistrationOpen {
                return Err(Error::InvalidStateTransition);
            }
            
            // Check if voter is already registered
            if self.is_registered(caller) {
                return Err(Error::AlreadyRegistered);
            }
            
            // Register the voter
            self.registered_voters.push(caller);
            
            Ok(())
        }

        /// Only callable by admin_id. Must be in RegistrationOpen state.
        /// duration_in_units must be a positive value. Sets current_voting_state to VotingActive.
        /// Records voting_start_time and calculates voting_end_time.
        #[ink(message)]
        pub fn start_voting(&mut self, duration_in_units: Timestamp) -> Result<()> {
            let caller = self.env().caller();
            
            // Check if the caller is the admin
            if caller != self.admin_id {
                return Err(Error::NotAuthorized);
            }
            
            // Check if we're in the correct state
            if self.current_voting_state != VotingState::RegistrationOpen {
                return Err(Error::InvalidStateTransition);
            }
            
            // Check if duration is positive
            if duration_in_units <= 0 {
                return Err(Error::InvalidDuration);
            }
            
            // Set voting times
            self.voting_start_time = self.env().block_timestamp();
            self.voting_end_time = self.voting_start_time + duration_in_units;
            
            // Transition to VotingActive state
            self.current_voting_state = VotingState::VotingActive;
            
            Ok(())
        }

        /// Only valid if current_voting_state is VotingActive.
        /// caller_id must be in registered_voters. caller_id must not have already voted.
        /// Current time must be within voting_start_time and voting_end_time.
        /// vote_option must be a valid option. Records caller_id as having voted.
        /// Increments vote_counts for the chosen vote_option.
        #[ink(message)]
        pub fn cast_vote(&mut self, vote_option: VoteOption) -> Result<()> {
            let caller = self.env().caller();
            
            // Check if we're in the correct state
            if self.current_voting_state != VotingState::VotingActive {
                return Err(Error::InvalidStateTransition);
            }
            
            // Check if voter is registered
            if !self.is_registered(caller) {
                return Err(Error::NotRegistered);
            }
            
            // Check if voter has already voted
            if self.has_voted(caller) {
                return Err(Error::AlreadyVoted);
            }
            
            // Check if we're within the voting period
            let current_time = self.env().block_timestamp();
            if current_time < self.voting_start_time || current_time > self.voting_end_time {
                return Err(Error::VotingPeriodNotActive);
            }
            
            // Check if vote option is valid
            match vote_option {
                VoteOption::OptionA | VoteOption::OptionB => (),
                // In this implementation we only accept OptionA and OptionB
            }
            
            // Record that the voter has voted
            self.votes_cast.push(caller);
            
            // Increment vote count for the chosen option
            let current_count = self.vote_counts.get(&vote_option).unwrap_or(&0);
            self.vote_counts.insert(vote_option, current_count + 1);
            
            Ok(())
        }

        /// Only callable by admin_id. Must be in VotingActive state.
        /// Current time must be past voting_end_time. Sets current_voting_state to ResultsFinalized.
        #[ink(message)]
        pub fn end_voting(&mut self) -> Result<()> {
            let caller = self.env().caller();
            
            // Check if the caller is the admin
            if caller != self.admin_id {
                return Err(Error::NotAuthorized);
            }
            
            // Check if we're in the correct state
            if self.current_voting_state != VotingState::VotingActive {
                return Err(Error::InvalidStateTransition);
            }
            
            // Check if voting period has ended
            let current_time = self.env().block_timestamp();
            if current_time < self.voting_end_time {
                return Err(Error::VotingPeriodNotActive);
            }
            
            // Transition to ResultsFinalized state
            self.current_voting_state = VotingState::ResultsFinalized;
            
            Ok(())
        }

        /// Only callable by admin_id. Resets current_voting_state to RegistrationOpen.
        /// Clears all voter and vote data.
        #[ink(message)]
        pub fn reset_application(&mut self) -> Result<()> {
            let caller = self.env().caller();
            
            // Check if the caller is the admin
            if caller != self.admin_id {
                return Err(Error::NotAuthorized);
            }
            
            // Reset all data
            self.registered_voters.clear();
            self.votes_cast.clear();
            *self.vote_counts.get_mut(&VoteOption::OptionA).unwrap() = 0;
            *self.vote_counts.get_mut(&VoteOption::OptionB).unwrap() = 0;
            
            // Reset times
            self.voting_start_time = 0;
            self.voting_end_time = 0;
            self.registration_start_block = 0;
            
            // Set state to RegistrationOpen
            self.current_voting_state = VotingState::RegistrationOpen;
            
            Ok(())
        }
    }

    /// Unit tests in Rust are normally defined within a module named `tests`
    #[cfg(test)]
    mod tests {
        use super::*;

        use ink_lang as ink;

        #[ink::test]
        fn new_voting_contract_has_correct_initial_state() {
            // Arrange & Act
            let contract = VotingContract::new();
            
            // Assert
            assert_eq!(contract.get_current_state(), VotingState::RegistrationOpen);
        }

        #[ink::test]
        fn register_voter_works() {
            // Arrange
            let mut contract = VotingContract::new();
            
            // Act
            let result = contract.register_voter();
            
            // Assert
            assert_eq!(result, Ok(()));
            assert!(contract.is_registered(AccountId::from([0x01; 32])));
        }

        #[ink::test]
        fn start_voting_by_non_admin_fails() {
            // Arrange
            let mut contract = VotingContract::new();
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();
            
            // Act - called by non-admin
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(accounts.bob);
            let result = contract.start_voting(1000);
            
            // Assert
            assert_eq!(result, Err(Error::NotAuthorized));
        }

        #[ink::test]
        fn cast_vote_by_unregistered_voter_fails() {
            // Arrange
            let mut contract = VotingContract::new();
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();
            
            // Start voting to reach VotingActive state
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(accounts.alice);
            let _ = contract.start_voting(1000);
            
            // Change caller to an unregistered voter
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(accounts.bob);
            
            // Act
            let result = contract.cast_vote(VoteOption::OptionA);
            
            // Assert
            assert_eq!(result, Err(Error::NotRegistered));
        }
    }
}