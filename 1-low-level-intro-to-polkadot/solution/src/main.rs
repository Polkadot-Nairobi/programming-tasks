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
}
