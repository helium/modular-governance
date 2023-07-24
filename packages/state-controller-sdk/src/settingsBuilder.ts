import BN from "bn.js";

export const PERCENTAGE_DIVISOR = 1000000000;

export function settings(): SettingsBuilder {
  return new SettingsBuilder();
}

export class SettingsBuilder {
  nodes: any[] = [];

  resolved(choices: number[]): SettingsBuilder {
    this.nodes.push({
      resolved: { choices },
    });
    return this;
  }

  endTimestamp(endTs: BN): SettingsBuilder {
    this.nodes.push({
      endTimestamp: { endTs },
    });
    return this;
  }

  offsetFromStartTs(offset: BN): SettingsBuilder {
    this.nodes.push({
      offsetFromStartTs: { offset },
    });
    return this;
  }

  numResolved(num: number = 1): SettingsBuilder {
    this.nodes.push({
      numResolved: { n: num },
    });
    return this;
  }

  choiceVoteWeight(weightThreshold: BN): SettingsBuilder {
    this.nodes.push({
      choiceVoteWeight: { weightThreshold },
    });
    return this;
  }

  /// If the percentage is a number, auto convert to PERCENTAGE_DIVISOR. Otherwise use the BN
  choicePercentage(percentage: BN | number): SettingsBuilder {
    if (typeof percentage === "number") {
      percentage = new BN((percentage / 100) * PERCENTAGE_DIVISOR);
    }
    this.nodes.push({
      choicePercentage: { percentage },
    });
    return this;
  }

  top(n: number = 1): SettingsBuilder {
    this.nodes.push({
      top: { n },
    });
    return this;
  }

  and(left: SettingsBuilder, right: SettingsBuilder): SettingsBuilder {
    this.nodes = this.nodes.concat(...left.build(), ...right.build(), {
      and: {},
    });
    return this;
  }

  or(left: SettingsBuilder, right: SettingsBuilder): SettingsBuilder {
    this.nodes = this.nodes.concat(...left.build(), ...right.build(), {
      and: {},
    });
    return this;
  }

  build(): any[] {
    return this.nodes;
  }
}
