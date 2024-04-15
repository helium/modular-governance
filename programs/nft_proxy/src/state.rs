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
  pub seasons: Vec<i64>,
}

impl ProxyConfigV0 {
  // Binary search for the timestamp closest to but after `unix_time`
  pub fn get_current_season_end(&self, unix_time: i64) -> Option<i64> {
    if self.seasons.is_empty() {
      return None;
    }

    let mut ans: Option<i64> = None;
    let mut low: usize = 0;
    let mut high: usize = self.seasons.len() - 1;

    while low <= high {
      let middle = (high + low) / 2;
      if let Some(current) = self.seasons.get(middle) {
        // Move to the left side if target time is greater
        if *current > unix_time {
          ans = Some(*current);
          // move left side
          if middle == 0 {
            break;
          }
          high = middle - 1;
        } else {
          low = middle + 1;
        }
      } else {
        break;
      }
    }

    ans
  }
}

// Forms an on chain linked list of proxys
#[account]
#[derive(Default, InitSpace)]
pub struct ProxyV0 {
  pub owner: Pubkey,
  pub proxy_config: Pubkey,
  pub asset: Pubkey,
  // Current index in the proxy chain.
  pub index: u16,
  // If this is the last in the line, Pubkey::default.
  pub next_owner: Pubkey,
  // The address of the account that paid for rent. Rent on closing
  // proxys always goes to the key who originally paid for them.
  pub rent_refund: Pubkey,
  // Unix timestamp in seconds that this proxy expires
  pub expiration_time: i64,
  pub bump_seed: u8,
}