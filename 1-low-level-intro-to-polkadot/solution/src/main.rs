fn main() {
    println!("Hello, world!");
}

use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};

/// Represents the different states of the voting process
#[derive(Debug, Clone, ParialEq)]
pub enum VotingState {
    RegistrationOpen,
    VotingActive,
    ResultsFinalized,
}

#[derive(Debug, PartialEq)]
pub enum VotingError {
    Unauthorized,
    InvalidState,
    VoterAlreadyRegistered,
    VoterNotRegistered,
    VoterAlreadyVoted,
    VotingNotActive,
    VotingPeriodExpired,
    VotingPeriodNotExpired,
    InvalidDuration,
    InvalidVoteOption,
}


pub struct VotingMachine {
    current_voting_state: VotingState,
    registered_voters: HashSet<String>,
    votes_cast: HashSet<String>,
    vote_counts: HashMap<String, u32>,
    voting_start_time: Option<u64>,
    voting_end_time: Option<u64>,
    admin_id: String,
}

impl VotingMachine {
    pub fn new(admin_id: String) -> Self {
        let mut vote_counts = Hashmap::new();
        vote_counts.insert("option_A".to_string(), 0);
        vote_counts.insert("option_B".to_string(), 0);
        vote_counts.insert("option_C".to_string(), 0);

        VotingMachine {
            current_voting_state: VotingState::RegistrationOpen,
            registered_voters: HashSet::new(),
            votes_cast: HashSet::new(),
            vote_counts,
            voting_start_time: None,
            voting_end_time: None,
            admin_id,
        }
    }

    fn get_current_time() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    fn is_admin(&self, caller_id: &str) -> bool {
        caller_id == self.admin_id
    }

    /// Check if coting period is currently active
    fn is_voting_period_active(&self) -> bool {
        if let(Some(start), Some(end)) = (self.voting_start_time, self.voting_end_time) {
            let current_time = Self::get_current_time();
            current_time >= start && current_time <= end
        } else {
            false
        }
    }

    /// Start the registration process, only callable by the admin
    pub fn start_registration(&mut self, caller_id: &str) -> Result<(), VotingError> {
        if !self.is_admin(caller_id) {
            return Err(VotingError::Unauthorized);
        }

        self.current_voting_state = VotingState::RegistrationOpen;
        self.registered_voters.clear();
        self.votes_cast.clear();
        self.vote_counts.clear();
        self.voting_start_time = None;
        self.voting_end_time = None;

        self.vote_counts.insert("option_A".to_string(), 0);
        self.vote_counts.insert("option_B".to_string(), 0);
        self.vote_counts.insert("option_C".to_string(), 0);

        Ok(())
    }

    /// Register a new voter 
    pub fn register_voter(&mut self, caller_id: &str) -> Result<(), VotingError> {
        if self.current_voting_state != VotingState::RegistrationOpen {
            return Err(VotingError::InvalidState);
        }

        if self.registered_voters.contains(caller_id) {
            return Err(VotingError::VoterAlreadyRegistered);
        }

        self.registered_voters.insert(caller_id.to_string());
        Ok(())
    }

    /// Start the voting period, only callable by the admin
    pub fn start_voting(&mut self, caller_id: &str, duration_in_seconds: u64) -> Result<(), VotingError> {
        if !self.is_admin(caller_id) {
            return Err(VotingError::Unauthorized);
        }

        if self.current_voting_state != VotingState::RegistrationOpen {
            return Err(VotingState::InvalidState);
        }

        if duration_in_seconds == 0 {
            return Err(VotingError::InvalidDuration);
        }

        let current_time = Self::get_current_time();
        self.current_voting_state = VotingState::VotingActive;
        self.voting_start_time = Some(current_time);
        self.voting_end_time = Some(current_time + duration_in_seconds);

        Ok(())
    }

    /// Cast a vote for the specified option, only valid during VotingActive state, within voting period, by regstered voters who haven't voted
    pub fn cast_vote(&mut self, caller_id: &str, vote_option: &str) -> Result<(), VotingError> {
        if self.current_voting_state != VotingState::VotingActive {
            return Err(VotingError::VotingNotActive);
        }

        if !self.registered_voters.contains(caller_id) {
             return Err(VotingError::VoterNotRegistered);
        }

        if slef.votes_cast.contains(caller_id) {
            return Err(VotingError::VoterAlreadyVoted);
        }

        if !self.is_voting_period_active() {
            return Err(VotingError::VotingPeriodExpired);
        }

        if !self.vote_counts.contains_key(vote_option) {
            return Err(VotingError::InvalidVoteOption);
        }

        self.votes_cast.insert(caller_id.to_string());
        *self.vote_counts.get_mut(vote_option).unwrap() += 1;

        Ok(())
    }

    /// End the voting period and finalize results, only callable by admin during VotingActive state and after voting period ends
    pub fn end_voting(&mut self, caller_id: &str) -> Result<(), VotingError> {
        if !self.is(caller_id) {
            return Err(VotingError::Unauthorized);
        }

        if self.current_voting_state != VotingState::VotingActive {
            return Err(VotingError::InvalidState);
        }

        if self.is_voting_period_active() {
            return Err(VotingError::VotingPeriodNotExpired);
        }

        self.current_voting_state = VotingState::ResultsFinalized;
        Ok(())
    }

    /// Reset the entire application to the initial state, only callable by the admin
    pub fn reset_application(&mut self, caller_id: &str) -> Result<(), VotingError> {
        if !self.is_admin(caller_id) {
            return Err(VotingError::Unauthorized);
        }

        self.current_voting_state = VotingState::RegistrationOpen;
        self.registered_voters.clear();
        self.votes_cast.clear();
        self.vote_counts.clear();
        self.voting_start_time = None;
        self.voting_end_time = None;

        self.vote_counts.insert("option_A".to_string(), 0);
        self.vote_counts.insert("option_B".to_string(), 0);
        self.vote_counts.insert("option_C".to_string(), 0);

        Ok(())
    }

    /// Get the current state of the voting machine
    pub fn get_current_state(&self) -> &VotingState {
        &self.current_voting_state
    }

    /// Get the current vote vote counts after voting ends
    pub fn get_results(&self) -> Option<&Hashmap<String, u32>> {
        if self.current_voting_state == VotingState::ResultsFinalized {
            Some(&self.vote_counts)
        } else {
            None
        }
    }

    /// Get the number of registered voters
    pub fn get_registered_voter_count(&self) -> usize {
        self.registered_voters.len()
    }

    /// Get the number of votes cast
    pub fn get_votes_cast_count(&self) -> usize {
        self.votes_cast.len()
    }
}
