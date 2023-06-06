use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, PartialEq)]
pub enum ProposalState {
    #[default]
    Voting,
    /// The proposal is resolved and the choice specified choice indices won
    Resolved(Vec<u16>),
    /// Allow voting controller to set to a custom state,
    /// this allows for the implementation of more complex
    /// states like Vetoed, drafts, signing off, etc.
    /// This could have been an int, but then UIs would need to understand
    /// the calling contract to grab an enum from it. Rather just have something clean
    /// even though it takes a bit more space.
    Custom(String),
}

#[derive(InitSpace, AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct Choice {
    /// Total vote weight behind this choice. u128 to support u64 tokens multiplied by a large multiplier (as in helium)
    pub weight: u128,
    #[max_len(200)]
    pub name: String,
    /// Any other data that you may want to put in here
    #[max_len(200)]
    pub uri: Option<String>,
}

#[account]
#[derive(Default)]
pub struct ProposalV0 {
    pub owner: Pubkey,
    pub state: ProposalState,
    pub created_at: i64,
    /// Signer that controls voting and vote weights
    pub vote_controller: Pubkey,
    /// Signer that controls the transitions of `ProposalState`
    /// You can either use the default `state-controller` smart contract
    /// Or you can implement a program that calls the `resolve_v0` method.
    /// The vote can only be resolved when this `resolution_settings` PDA signs `resolve_v0`. This allows
    /// you to trigger resolution on either (a) a vote, (b) a timestamp, or (c) some custom trigger with clockwork
    pub state_controller: Pubkey,
    /// Optional program that will be called with `on_vote_v0` after every vote
    /// Defaults to the owner of `resolution_settings`, which allows it to reactively call resolve_v0
    pub on_vote_hook: Pubkey,
    pub seed: Vec<u8>,
    pub name: String,
    /// URI to json containing name, description, etc
    pub uri: String,
    pub tags: Vec<String>,
    pub choices: Vec<Choice>,
    pub bump_seed: u8,
}

impl anchor_lang::Space for ProposalV0 {
    const INIT_SPACE: usize = std::mem::size_of::<Self>();
}

#[macro_export]
macro_rules! proposal_seeds {
    ( $proposal:expr ) => {
        &[b"proposal", &$proposal.seed[..], &[$proposal.bump_seed]]
    };
}

pub struct Vote {
    pub choice: u16,
    pub weight: u128,
}

impl ProposalV0 {
    pub fn vote(&mut self, vote: Vote) {
        self.choices[vote.choice as usize].weight = self.choices[vote.choice as usize]
            .weight
            .checked_add(vote.weight)
            .unwrap();
    }

    pub fn remove_vote(&mut self, vote: Vote) {
        self.choices[vote.choice as usize].weight = self.choices[vote.choice as usize]
            .weight
            .checked_sub(vote.weight)
            .unwrap();
    }
}
