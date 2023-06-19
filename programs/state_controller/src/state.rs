use std::collections::HashSet;

use anchor_lang::prelude::*;
use proposal::{ProposalState, ProposalV0};

pub const PERCENTAGE_DIVISOR: u32 = 1000000000;

/// Allow building complex operations to decide resolution.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub enum ResolutionNode {
  // Already resolved vote to a specifc choice
  Resolved(Vec<u16>),
  /// Simple: At the specified end timestamp, the proposal is resolved with the choice
  /// that has the most vote weight
  EndTimestamp(i64),
  /// At the specified offset  from start ts, the proposal is resolved with the choice
  OffsetFromStartTs(i64),
  /// The choice crosses this vote weight
  ChoiceVoteWeight(u128),
  /// The choice has this percentage (i32 / PERCENTAGE_DIVISOR)
  ChoicePercentage(i32),
  /// The choice has the maximum percentage of votes (only one may pass)
  #[default]
  Max,
  And,
  Or,
}

impl ResolutionNode {
  pub fn size(&self) -> usize {
    match self {
      ResolutionNode::Resolved(vec) => 4 + vec.len() * 2,
      ResolutionNode::EndTimestamp(_) => 8,
      ResolutionNode::OffsetFromStartTs(_) => 8,
      ResolutionNode::ChoiceVoteWeight(_) => 16,
      ResolutionNode::ChoicePercentage(_) => 4,
      ResolutionNode::Max => 0,
      ResolutionNode::And => 0,
      ResolutionNode::Or => 0,
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
        ResolutionNode::Resolved(choices) => {
          stack.push(Some(choices.clone()));
        }
        ResolutionNode::EndTimestamp(end) => {
          if Clock::get().unwrap().unix_timestamp > *end {
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
        ResolutionNode::OffsetFromStartTs(offset) => match proposal.state {
          ProposalState::Voting(start_ts) => {
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
        ResolutionNode::ChoiceVoteWeight(weight) => stack.push(Some(
          proposal
            .choices
            .iter()
            .enumerate()
            .flat_map(|(index, choice)| {
              if choice.weight > *weight {
                Some(index as u16)
              } else {
                None
              }
            })
            .collect(),
        )),
        ResolutionNode::ChoicePercentage(percent) => {
          let total_weight = proposal
            .choices
            .iter()
            .map(|choice| choice.weight)
            .sum::<u128>();
          let threshold = total_weight
            .checked_mul(*percent as u128)
            .unwrap()
            .checked_div(PERCENTAGE_DIVISOR as u128)
            .unwrap();
          let ret = Some(
            proposal
              .choices
              .iter()
              .enumerate()
              .flat_map(|(index, choice)| {
                if choice.weight > threshold {
                  Some(index as u16)
                } else {
                  None
                }
              })
              .collect(),
          );
          stack.push(ret)
        }
        ResolutionNode::Max => {
          let max = proposal
            .choices
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.weight.cmp(&b.weight));
          let ret = match max {
            Some((index, _)) => Some(vec![index as u16]),
            None => None,
          };

          stack.push(ret)
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
      b"resolution-settings",
      $settings.name.as_bytes(),
      &[$settings.bump_seed],
    ]
  };
}
