# Task 1: Low Level Introduction to Polkadot

Program a simple voting state machine that has distinct states and defines user interactions that trigger state transitions.

01. Create an enum to represent the different states of the voting process. It must include at least three distinct states:

- RegistrationOpen
- VotingActive
- ResultsFinalized
The starting state should always be RegistrationOpen state.

02. Represent the current state using arrays/ vectors. This should include:

- current_voting_state: Stores the current state of our state machine
- registered_voters: A collection to track unique voter identifiers.
- votes_cast: A collection to record which voters have already cast a vote.
- vote_counts: A map to store the tally for each voting option (e.g., {'option_A': count, 'option_B': count}).
- voting_start_time and voting_end_time: Timestamps or block numbers to define the active voting period.
- admin_id: A designated identifier for the administrator who can trigger privileged actions.

03. Implement functions/methods that will act as our state transition logic. Each function should take a caller_id (representing the user or administrator initiating the action) and any necessary parameters. Each function should return a success/failure status or an error message.

- start_registration(caller_id): Only callable by admin_id. Must not already be in RegistrationOpen state. Sets current_voting_state to RegistrationOpen. Clears previous voter and vote data.
- register_voter(caller_id): Only valid if current_voting_state is RegistrationOpen. caller_id must not already be registered. Adds caller_id to registered_voters.
- start_voting(caller_id, duration_in_units): Only callable by admin_id. Must be in RegistrationOpen state. duration_in_units must be a positive value. Sets current_voting_state to VotingActive. Records voting_start_time and calculates voting_end_time.
- cast_vote(caller_id, vote_option): Only valid if current_voting_state is VotingActive. caller_id must be in registered_voters. caller_id must not have already voted. Current time/block must be within voting_start_time and voting_end_time. vote_option must be a valid option. Records caller_id as having voted. Increments vote_counts for the chosen vote_option.
- end_voting(caller_id): Only callable by admin_id. Must be in VotingActive state. Current time/block must be past voting_end_time. Sets current_voting_state to ResultsFinalized.
- reset_application(caller_id): Only callable by admin_id. Resets current_voting_state to RegistrationOpen. Clears all voter and vote data.
