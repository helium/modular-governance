import { PublicKey } from "@solana/web3.js";
import {
  useAnchorAccount,
  useAnchorAccounts,
} from "@helium/helium-react-hooks";
import { Proposal } from "@helium/modular-governance-idls/lib/types/proposal";
import { Organization } from "@helium/modular-governance-idls/lib/types/organization";
import { StateController } from "@helium/modular-governance-idls/lib/types/state_controller";
import { proposalKey } from "@helium/organization-sdk";
import { useMemo } from "react";

export const useProposalConfig = (key: PublicKey | undefined) =>
  // @ts-ignore
  useAnchorAccount<Proposal, "proposalConfigV0">(key, "proposalConfigV0");

export const useProposal = (key: PublicKey | undefined) =>
  // @ts-ignore
  useAnchorAccount<Proposal, "proposalV0">(key, "proposalV0");

export const useOrganization = (key: PublicKey | undefined) =>
  // @ts-ignore
  useAnchorAccount<Organization, "organizationV0">(key, "organizationV0");

export const useResolutionSettings = (key: PublicKey | undefined) =>
  // @ts-ignore
  useAnchorAccount<StateController, "resolutionSettingsV0">(
    key,
    "resolutionSettingsV0"
  );

export const useOrganizationProposals = (
  organizationKey: PublicKey | undefined
) => {
  const { info: organization } = useOrganization(organizationKey);

  const proposalKeys = useMemo(
    () =>
      // @ts-ignore
      Array(organization?.numProposals)
        .fill(0)
        .map((_, index) => proposalKey(organizationKey, index)[0])
        .reverse(),
    // @ts-ignore
    [organization?.numProposals]
  );
  // @ts-ignore
  return useAnchorAccounts<Proposal, "proposalV0">(proposalKeys, "proposalV0");
};
