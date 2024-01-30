use std::collections::HashSet;

use anchor_lang::prelude::*;
use proposal::{ProposalState, ProposalV0};

pub const PERCENTAGE_DIVISOR: u32 = 1000000000;

/// Allow building complex operations to decide resolution.
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum ResolutionNode {
  // Already resolved vote to a specifc choice
  Resolved {
    choices: Vec<u16>,
  },
  /// Simple: At the specified end timestamp, the proposal is resolved with the choice
  /// that has the most vote weight
  EndTimestamp {
    end_ts: i64,
  },
  /// At the specified offset  from start ts, the proposal is resolved with the choice
  OffsetFromStartTs {
    offset: i64,
  },
  /// The choice crosses this vote weight
  ChoiceVoteWeight {
    weight_threshold: u128,
  },
  /// The choice has this percentage (i32 / PERCENTAGE_DIVISOR)
  ChoicePercentage {
    percentage: i32,
  },
  /// Top n choices are resolved
  Top {
    n: u16,
  },
  /// Requires that a number of choices are resolved by other resolvers
  /// before returning non None
  NumResolved {
    n: u16,
  },
  And,
  Or,
}

impl Default for ResolutionNode {
  fn default() -> Self {
    ResolutionNode::Top { n: 1 }
  }
}

impl ResolutionNode {
  pub fn size(&self) -> usize {
    match self {
      ResolutionNode::Resolved { choices } => 4 + choices.len() * 2,
      ResolutionNode::EndTimestamp { .. } => 8,
      ResolutionNode::OffsetFromStartTs { .. } => 8,
      ResolutionNode::ChoiceVoteWeight { .. } => 16,
      ResolutionNode::ChoicePercentage { .. } => 4,
      ResolutionNode::Top { .. } => 4,
      ResolutionNode::And => 0,
      ResolutionNode::Or => 0,
      ResolutionNode::NumResolved { .. } => 4,
    }
  }
}

/// Reverse polish notation calculator
/// https://en.wikipedia.org/wiki/Reverse_Polish_notation
/// Do this to have a flat structure since rust doesn't like unbounded nesting of types
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct ResolutionStrategy {
  pub nodes: Vec<ResolutionNode>,
}

pub fn intersect<T: std::cmp::Eq + std::hash::Hash + Clone>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
  let unique_a: HashSet<_> = a.iter().collect();
  let unique_b: HashSet<_> = b.iter().collect();

  unique_a
    .intersection(&unique_b)
    .map(|&x| x.clone())
    .collect()
}

pub fn union<T: std::cmp::Eq + std::hash::Hash + Clone>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
  let unique_a: HashSet<_> = a.iter().collect();
  let unique_b: HashSet<_> = b.iter().collect();

  unique_a.union(&unique_b).map(|&x| x.clone()).collect()
}

impl ResolutionStrategy {
  pub fn resolution(&self, proposal: &ProposalV0) -> Option<Vec<u16>> {
    let mut stack: Vec<Option<Vec<u16>>> = vec![];
    for input in &self.nodes {
      match input {
        ResolutionNode::Resolved { choices } => {
          stack.push(Some(choices.clone()));
        }
        ResolutionNode::EndTimestamp { end_ts } => {
          if Clock::get().unwrap().unix_timestamp > *end_ts {
            stack.push(Some(
              proposal
                .choices
                .iter()
                .enumerate()
                .map(|i| i.0 as u16)
                .collect(),
            ));
          } else {
            stack.push(None);
          }
        }
        ResolutionNode::OffsetFromStartTs { offset } => match proposal.state {
          ProposalState::Voting { start_ts } => {
            if Clock::get().unwrap().unix_timestamp > start_ts + offset {
              stack.push(Some(
                proposal
                  .choices
                  .iter()
                  .enumerate()
                  .map(|i| i.0 as u16)
                  .collect(),
              ));
            } else {
              stack.push(None);
            }
          }
          _ => stack.push(None),
        },
        ResolutionNode::ChoiceVoteWeight { weight_threshold } => stack.push(Some(
          proposal
            .choices
            .iter()
            .enumerate()
            .flat_map(|(index, choice)| {
              if choice.weight >= *weight_threshold {
                Some(index as u16)
              } else {
                None
              }
            })
            .collect(),
        )),
        ResolutionNode::ChoicePercentage { percentage } => {
          let total_weight = proposal
            .choices
            .iter()
            .map(|choice| choice.weight)
            .sum::<u128>();
          let threshold = total_weight
            .checked_mul(*percentage as u128)
            .unwrap()
            .checked_div(PERCENTAGE_DIVISOR as u128)
            .map(|result| {
              let remainder = total_weight
                .checked_mul(*percentage as u128)
                .unwrap()
                .checked_rem(PERCENTAGE_DIVISOR as u128)
                .unwrap();
              result
                .checked_add(if remainder > 0 { 1 } else { 0 })
                .unwrap()
            })
            .unwrap();
          let ret = Some(
            proposal
              .choices
              .iter()
              .enumerate()
              .flat_map(|(index, choice)| {
                if threshold == 0 {
                  None
                } else if choice.weight >= threshold {
                  Some(index as u16)
                } else {
                  None
                }
              })
              .collect(),
          );
          stack.push(ret)
        }
        ResolutionNode::Top { n } => {
          let mut vec = proposal.choices.iter().enumerate().collect::<Vec<_>>();

          vec.sort_by(|(_, a), (_, b)| b.weight.cmp(&a.weight));

          stack.push(Some(
            vec
              .iter()
              .map(|(index, _)| *index as u16)
              .take(*n as usize)
              .collect(),
          ))
        }
        ResolutionNode::And => {
          let left = stack.pop().unwrap();
          let right = stack.pop().unwrap();

          let ret = match (left, right) {
            (Some(left), Some(right)) => Some(intersect(left, right)),
            _ => None,
          };

          stack.push(ret)
        }
        ResolutionNode::Or => {
          let left = stack.pop().unwrap();
          let right = stack.pop().unwrap();

          let ret = match (left, right) {
            (Some(left), Some(right)) => Some(union(left, right)),
            (Some(left), None) => Some(left),
            (None, Some(right)) => Some(right),
            _ => None,
          };

          stack.push(ret)
        }
        ResolutionNode::NumResolved { n } => {
          let curr = stack.first().unwrap();
          match curr {
            Some(vec) if vec.len() >= *n as usize => stack.push(Some(vec.clone())),
            _ => stack.push(None),
          }
        }
      }
    }

    stack.pop().unwrap()
  }
}

#[account]
#[derive(Default)]
pub struct ResolutionSettingsV0 {
  pub name: String,
  pub settings: ResolutionStrategy,
  pub bump_seed: u8,
}

#[macro_export]
macro_rules! resolution_setting_seeds {
  ( $settings:expr ) => {
    &[
      b"resolution_settings",
      $settings.name.as_bytes(),
      &[$settings.bump_seed],
    ]
  };
}
