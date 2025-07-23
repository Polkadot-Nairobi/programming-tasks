use std::time::Duration;
#[derive(PartialEq, Debug)]
pub enum State {
    RegistrationClosed,
    RegistrationOpen,
    VotingActive,
    ResultsFinalized,
}

pub struct VotingState {
    pub current_voting_state: State,
    pub registered_voters: Vec<String>,
    pub votes_cast: Vec<String>,   
    pub vote_counts: std::collections::HashMap<String, u32>,
    pub voting_start_time: u64,
    pub voting_end_time: u64,         
    pub admin_id: String,           
}
pub fn start_registration(caller_id: &str, voting_state: &mut VotingState) -> Result<(), String> {
    if voting_state.current_voting_state == State::RegistrationOpen {
        return Err("Registration is already open.".to_string());
    }
    if caller_id != voting_state.admin_id {
        return Err("Only the admin can start registration.".to_string());
    }

    voting_state.registered_voters.clear();
    voting_state.votes_cast.clear();
    voting_state.vote_counts.clear();

    voting_state.current_voting_state = State::RegistrationOpen;

    Ok(())
}
pub fn register_voter(caller_id: &str, voting_state: &mut VotingState) -> Result<(), String> {
    if voting_state.current_voting_state != State::RegistrationOpen {
        return Err("Registration is not open.".to_string());
    }
    if voting_state
        .registered_voters
        .contains(&caller_id.to_string())
    {
        return Err("Voter is already registered.".to_string());
    }
    voting_state.registered_voters.push(caller_id.to_string());

    Ok(())
}
pub fn start_voting(
    caller_id: &str,
    voting_state: &mut VotingState,
    duration_in_units: u64,
) -> Result<(), String> {
    if caller_id != voting_state.admin_id {
        return Err("Only the admin can start voting.".to_string());
    }
    if voting_state.current_voting_state == State::VotingActive {
        return Err("Voting is already active.".to_string());
    }
    assert!(duration_in_units > 0, "Duration must be positive value.");

    voting_state.current_voting_state = State::VotingActive;
    let start_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Failed to retrieve system time")
        .as_secs();
    voting_state.voting_start_time = start_time;
    let duration = Duration::from_secs(duration_in_units * 60 * 60); // Convert hours to seconds
    voting_state.voting_end_time = start_time + duration.as_secs();

    Ok(())
}

pub fn cast_vote(
    caller_id: &str,
    vote_option: &str,
    voting_state: &mut VotingState,
) -> Result<(), String> {
    if voting_state.current_voting_state != State::VotingActive {
        return Err("Voting is not active.".to_string());
    }
    if !voting_state
        .registered_voters
        .contains(&caller_id.to_string())
    {
        return Err("Caller is not a registered voter.".to_string());
    }
    if voting_state.votes_cast.contains(&caller_id.to_string()) {
        return Err("Caller has already voted.".to_string());
    }

    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Failed to retrieve system time")
        .as_secs();
    if current_time < voting_state.voting_start_time || current_time > voting_state.voting_end_time
    {
        return Err("Voting is not within the active period.".to_string());
    }
    if !voting_state.vote_counts.contains_key(vote_option) {
        return Err("Invalid vote option.".to_string());
    }
    voting_state.votes_cast.push(caller_id.to_string());
    let count = voting_state
        .vote_counts
        .entry(vote_option.to_string())
        .or_insert(0);
    *count += 1;
    Ok(())
}

pub fn end_voting(caller_id: &str, voting_state: &mut VotingState) -> Result<(), String> {
    if caller_id != voting_state.admin_id {
        return Err("Only the admin can end voting.".to_string());
    }
    if voting_state.current_voting_state != State::VotingActive {
        return Err("Voting is not active.".to_string());
    }

    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Failed to retrieve system time")
        .as_secs();
    if current_time < voting_state.voting_end_time {
        return Err("Voting period is still active.".to_string());
    }

    
    voting_state.current_voting_state = State::ResultsFinalized;

    Ok(())
}

pub fn reset_application(caller_id: &str, voting_state: &mut VotingState) -> Result<(), String> {
    if caller_id != voting_state.admin_id {
        return Err("Only the admin can reset the application.".to_string());
    }
    voting_state.current_voting_state = State::RegistrationOpen;

    voting_state.registered_voters.clear();
    voting_state.votes_cast.clear();
    voting_state.vote_counts.clear();
    voting_state.voting_start_time = 0;
    voting_state.voting_end_time = 0;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_start_registration() {
        let mut voting_state = VotingState {
            current_voting_state: State::RegistrationClosed,
            registered_voters: Vec::new(),
            votes_cast: Vec::new(),
            vote_counts: std::collections::HashMap::new(),
            voting_start_time: 0,
            voting_end_time: 0,
            admin_id: "admin123".to_string(),
        };
        
        let result = start_registration("admin123", &mut voting_state);
        assert!(result.is_ok());
        assert_eq!(voting_state.current_voting_state, State::RegistrationOpen);
        assert!(voting_state.registered_voters.is_empty());
        assert!(voting_state.votes_cast.is_empty());
        assert!(voting_state.vote_counts.is_empty());
        let result = start_registration("admin123", &mut voting_state);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Registration is already open.".to_string()
        );
    }
    #[test]
    fn test_start_registration_wrong_admin() {
        let mut voting_state = VotingState {
            current_voting_state: State::RegistrationClosed,
            registered_voters: Vec::new(),
            votes_cast: Vec::new(),
            vote_counts: std::collections::HashMap::new(),
            voting_start_time: 0,
            voting_end_time: 0,
            admin_id: "admin123".to_string(),
        };
        let result = start_registration("wrong_admin", &mut voting_state);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Only the admin can start registration.".to_string()
        );
    }
    #[test]
    fn test_register_voter() {
        let mut voting_state = VotingState {
            current_voting_state: State::RegistrationOpen,
            registered_voters: Vec::new(),
            votes_cast: Vec::new(),
            vote_counts: std::collections::HashMap::new(),
            voting_start_time: 0,
            voting_end_time: 0,
            admin_id: "admin123".to_string(),
        };
        let result = register_voter("voter1", &mut voting_state);
        assert!(result.is_ok());
        assert_eq!(voting_state.registered_voters, vec!["voter1".to_string()]);
    }
    #[test]
    fn test_register_voter_double_registration() {
        let mut voting_state = VotingState {
            current_voting_state: State::RegistrationOpen,
            registered_voters: Vec::new(),
            votes_cast: Vec::new(),
            vote_counts: std::collections::HashMap::new(),
            voting_start_time: 0,
            voting_end_time: 0,
            admin_id: "admin123".to_string(),
        };
        let _ = register_voter("voter1", &mut voting_state);
        let result = register_voter("voter1", &mut voting_state);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Voter is already registered.".to_string()
        );
    }
    #[test]
    fn test_register_voter_closed_window() {
        let mut voting_state = VotingState {
            current_voting_state: State::RegistrationOpen,
            registered_voters: Vec::new(),
            votes_cast: Vec::new(),
            vote_counts: std::collections::HashMap::new(),
            voting_start_time: 0,
            voting_end_time: 0,
            admin_id: "admin123".to_string(),
        };
        voting_state.current_voting_state = State::RegistrationClosed;
        let result = register_voter("voter2", &mut voting_state);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Registration is not open.".to_string());
    }
    #[test]
    fn test_start_voting() {
        let mut voting_state = VotingState {
            current_voting_state: State::RegistrationOpen,
            registered_voters: vec!["voter1".to_string()],
            votes_cast: Vec::new(),
            vote_counts: std::collections::HashMap::new(),
            voting_start_time: 0,
            voting_end_time: 0,
            admin_id: "admin123".to_string(),
        };
        
        let result = start_voting("admin123", &mut voting_state, 2);
        assert!(result.is_ok());
        assert_eq!(voting_state.current_voting_state, State::VotingActive);
    }
    #[test]
    fn test_start_voting_wrong_admin() {
        let mut voting_state = VotingState {
            current_voting_state: State::RegistrationOpen,
            registered_voters: vec!["voter1".to_string()],
            votes_cast: Vec::new(),
            vote_counts: std::collections::HashMap::new(),
            voting_start_time: 0,
            voting_end_time: 0,
            admin_id: "admin123".to_string(),
        };
        
        let result = start_voting("wrong_admin", &mut voting_state, 2);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Only the admin can start voting.".to_string()
        );
    }
    #[test]
    fn test_start_voting_already_active() {
        let mut voting_state = VotingState {
            current_voting_state: State::VotingActive,
            registered_voters: vec!["voter1".to_string()],
            votes_cast: Vec::new(),
            vote_counts: std::collections::HashMap::new(),
            voting_start_time: 0,
            voting_end_time: 0,
            admin_id: "admin123".to_string(),
        };
       
        let _ = start_voting("admin123", &mut voting_state, 2);
        let result = start_voting("admin123", &mut voting_state, 2);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Voting is already active.".to_string());
    }
    #[test]
    fn test_cast_vote() {
        let mut voting_state = VotingState {
            current_voting_state: State::VotingActive,
            registered_voters: vec!["voter1".to_string()],
            votes_cast: Vec::new(),
            vote_counts: vec![("option_A".to_string(), 0), ("option_B".to_string(), 0)]
                .into_iter()
                .collect(),
            voting_start_time: 0,
            voting_end_time: 0,
            admin_id: "admin123".to_string(),
        };
        voting_state.voting_start_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        voting_state.voting_end_time = voting_state.voting_start_time + 3600;
        
        let result = cast_vote("voter1", "option_A", &mut voting_state);
        assert!(result.is_ok());
        assert_eq!(voting_state.votes_cast, vec!["voter1".to_string()]);
        assert_eq!(voting_state.vote_counts.get("option_A").unwrap(), &1);
        assert_eq!(voting_state.vote_counts.get("option_B").unwrap(), &0);
    }

    #[test]
    fn test_cast_vote_double_vote() {
        let mut voting_state = VotingState {
            current_voting_state: State::VotingActive,
            registered_voters: vec!["voter1".to_string()],
            votes_cast: Vec::new(),
            vote_counts: vec![("option_A".to_string(), 0), ("option_B".to_string(), 0)]
                .into_iter()
                .collect(),
            voting_start_time: 0,
            voting_end_time: 0,
            admin_id: "admin123".to_string(),
        };
         
        voting_state.voting_start_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        voting_state.voting_end_time = voting_state.voting_start_time + 3600;
        
        let _ = cast_vote("voter1", "option_A", &mut voting_state);
        let result = cast_vote("voter1", "option_A", &mut voting_state);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Caller has already voted.".to_string());
    }
    #[test]
    fn test_cast_vote_before_time() {
        let mut voting_state = VotingState {
            current_voting_state: State::VotingActive,
            registered_voters: vec!["voter1".to_string()],
            votes_cast: Vec::new(),
            vote_counts: vec![("option_A".to_string(), 0), ("option_B".to_string(), 0)]
                .into_iter()
                .collect(),
            voting_start_time: 0,
            voting_end_time: 0,
            admin_id: "admin123".to_string(),
        };
        
        voting_state.voting_start_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Failed to retrieve system time")
            .as_secs()
            + 7200; // Set start time to 2 hours in
        voting_state.voting_end_time = voting_state.voting_start_time + 3600;
        let result = cast_vote("voter1", "option_A", &mut voting_state);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Voting is not within the active period.".to_string()
        );
    }

    #[test]
    fn test_cast_vote_invalid_option() {
        let mut voting_state = VotingState {
            current_voting_state: State::VotingActive,
            registered_voters: vec!["voter1".to_string()],
            votes_cast: Vec::new(),
            vote_counts: vec![("option_A".to_string(), 0), ("option_B".to_string(), 0)]
                .into_iter()
                .collect(),
            voting_start_time: 0,
            voting_end_time: 0,
            admin_id: "admin123".to_string(),
        };
        
        voting_state.voting_start_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        voting_state.voting_end_time = voting_state.voting_start_time + 3600;
        let result = cast_vote("voter1", "invalid_option", &mut voting_state);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid vote option.".to_string());
    }

    #[test]
    fn test_cast_vote_non_registered_voter() {       
        let mut voting_state = VotingState {
            current_voting_state: State::VotingActive,
            registered_voters: vec!["voter1".to_string()],
            votes_cast: Vec::new(),
            vote_counts: vec![("option_A".to_string(), 0), ("option_B".to_string(), 0)]
                .into_iter()
                .collect(),
            voting_start_time: 0,
            voting_end_time: 0,
            admin_id: "admin123".to_string(),
        };
        voting_state.voting_start_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("TFailed to retrieve system time")
            .as_secs();
        voting_state.voting_end_time = voting_state.voting_start_time + 3600;
        let result = cast_vote("non_registered_voter", "option_A", &mut voting_state);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Caller is not a registered voter.".to_string()
        );
    }
    #[test]
    fn test_end_voting() {
        let mut voting_state = VotingState {
            current_voting_state: State::VotingActive,
            registered_voters: vec!["voter1".to_string()],
            votes_cast: Vec::new(),
            vote_counts: std::collections::HashMap::new(),
            voting_start_time: 0,
            voting_end_time: 0,
            admin_id: "admin123".to_string(),
        };
        let result = end_voting("admin123", &mut voting_state);
        assert!(result.is_ok());
        assert_eq!(voting_state.current_voting_state, State::ResultsFinalized);
    }
    #[test]
    fn test_end_voting_wrong_admin() {
        let mut voting_state = VotingState {
            current_voting_state: State::VotingActive,
            registered_voters: vec!["voter1".to_string()],
            votes_cast: Vec::new(),
            vote_counts: std::collections::HashMap::new(),
            voting_start_time: 0,
            voting_end_time: 0,
            admin_id: "admin123".to_string(),
        };
        voting_state.voting_start_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Failed to retrieve system time")
            .as_secs();
        voting_state.voting_end_time = voting_state.voting_start_time + 3600;
       
        let result = end_voting("wrong_admin", &mut voting_state);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Only the admin can end voting.".to_string()
        );
    }
    #[test]
    fn test_reset_application() {
        let mut voting_state = VotingState {
            current_voting_state: State::VotingActive,
            registered_voters: vec!["voter1".to_string()],
            votes_cast: Vec::new(),
            vote_counts: std::collections::HashMap::new(),
            voting_start_time: 0,
            voting_end_time: 0,
            admin_id: "admin123".to_string(),
        };
        
        let result = reset_application("admin123", &mut voting_state);
        assert!(result.is_ok());
        assert_eq!(voting_state.current_voting_state, State::RegistrationOpen);
        assert!(voting_state.registered_voters.is_empty());
        assert!(voting_state.votes_cast.is_empty());
        assert!(voting_state.vote_counts.is_empty());
    }
    #[test]
    fn test_reset_application_wrong_admin() {
        let mut voting_state = VotingState {
            current_voting_state: State::VotingActive,
            registered_voters: vec!["voter1".to_string()],
            votes_cast: Vec::new(),
            vote_counts: std::collections::HashMap::new(),
            voting_start_time: 0,
            voting_end_time: 0,
            admin_id: "admin123".to_string(),
        };
        
        let result = reset_application("wrong_admin", &mut voting_state);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Only the admin can reset the application.".to_string()
        );
    }
}
