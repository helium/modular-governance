use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct ProxyConfigV0 {
  pub authority: Pubkey,
  pub name: String,
  // Max time to delegate in seconds
  pub max_proxy_time: i64,
  // Optionally sync proxys to seasons,
  // so that they all expire at pre-defined intervals. Creating an election cycle
  pub seasons: Vec<SeasonV0>,
}

// Seasons may overlap, the code will always take the latest `end`
// This allows you to have a season that starts before the previous season ends,
// allowing re-assignment before expiration.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct SeasonV0 {
  pub start: i64,
  pub end: i64,
}

impl ProxyConfigV0 {
  // Binary search for the timestamp closest to but after `unix_time`
  pub fn get_current_season(&self, unix_time: i64) -> Option<SeasonV0> {
    if self.seasons.is_empty() {
      return None;
    }

    let mut ans: Option<SeasonV0> = None;
    let mut low: usize = 0;
    let mut high: usize = self.seasons.len() - 1;

    while low <= high {
      let middle = (high + low) / 2;
      if let Some(current) = self.seasons.get(middle) {
        // Move to the right side if target time is greater
        if current.start <= unix_time {
          ans = Some(current.clone());
          low = middle + 1;
        } else {
          if middle == 0 {
            break;
          }
          high = middle - 1;
        }
      } else {
        break;
      }
    }

    ans
  }
}

// Forms an on chain linked list of proxy voters
#[account]
#[derive(Default, InitSpace)]
pub struct ProxyAssignmentV0 {
  pub voter: Pubkey,
  pub proxy_config: Pubkey,
  pub asset: Pubkey,
  // Current index in the proxy chain.
  pub index: u16,
  // If this is the last in the line, Pubkey::default.
  pub next_voter: Pubkey,
  // The address of the account that paid for rent. Rent on closing
  // proxys always goes to the key who originally paid for them.
  pub rent_refund: Pubkey,
  // Unix timestamp in seconds that this proxy expires
  pub expiration_time: i64,
  pub bump_seed: u8,
}
