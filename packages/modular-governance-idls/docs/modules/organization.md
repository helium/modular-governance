[Documentation](../README.md) / [Modules](../modules.md) / organization

# Module: organization

## Table of contents

### Type Aliases

- [Organization](organization.md#organization)

### Variables

- [IDL](organization.md#idl)

## Type Aliases

### Organization

Ƭ **Organization**: `Object`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `accounts` | [{ `name`: ``"organizationV0"`` ; `type`: { `fields`: [{ `name`: ``"numProposals"`` ; `type`: ``"u32"``  }, { `docs`: [``"Authority to create proposals under this organization"``] ; `name`: ``"authority"`` ; `type`: ``"publicKey"``  }, { `name`: ``"defaultProposalConfig"`` ; `type`: ``"publicKey"``  }, { `name`: ``"proposalProgram"`` ; `type`: ``"publicKey"``  }, { `name`: ``"name"`` ; `type`: ``"string"``  }, { `name`: ``"uri"`` ; `type`: ``"string"``  }, { `name`: ``"bumpSeed"`` ; `type`: ``"u8"``  }] ; `kind`: ``"struct"``  }  }] |
| `instructions` | [{ `accounts`: [{ `isMut`: ``true`` ; `isSigner`: ``true`` ; `name`: ``"payer"``  }, { `isMut`: ``true`` ; `isSigner`: ``false`` ; `name`: ``"organization"`` ; `pda`: { `seeds`: [{ `kind`: ``"const"`` ; `type`: ``"string"`` ; `value`: ``"organization"``  }, { `kind`: ``"arg"`` ; `path`: ``"args.name"`` ; `type`: { `defined`: ``"InitializeOrganizationArgsV0"``  }  }]  }  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"systemProgram"``  }] ; `args`: [{ `name`: ``"args"`` ; `type`: { `defined`: ``"InitializeOrganizationArgsV0"``  }  }] ; `name`: ``"initializeOrganizationV0"``  }, { `accounts`: [{ `isMut`: ``true`` ; `isSigner`: ``true`` ; `name`: ``"payer"``  }, { `isMut`: ``false`` ; `isSigner`: ``true`` ; `name`: ``"authority"``  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"owner"``  }, { `isMut`: ``true`` ; `isSigner`: ``false`` ; `name`: ``"proposal"``  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"proposalConfig"``  }, { `isMut`: ``true`` ; `isSigner`: ``false`` ; `name`: ``"organization"`` ; `relations`: [``"proposal_program"``, ``"authority"``]  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"proposalProgram"``  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"systemProgram"``  }] ; `args`: [{ `name`: ``"args"`` ; `type`: { `defined`: ``"InitializeProposalArgsV0"``  }  }] ; `name`: ``"initializeProposalV0"``  }] |
| `name` | ``"organization"`` |
| `types` | [{ `name`: ``"InitializeOrganizationArgsV0"`` ; `type`: { `fields`: [{ `name`: ``"name"`` ; `type`: ``"string"``  }, { `name`: ``"authority"`` ; `type`: ``"publicKey"``  }, { `name`: ``"defaultProposalConfig"`` ; `type`: ``"publicKey"``  }, { `name`: ``"proposalProgram"`` ; `type`: ``"publicKey"``  }, { `name`: ``"uri"`` ; `type`: ``"string"``  }] ; `kind`: ``"struct"``  }  }, { `name`: ``"ChoiceArg"`` ; `type`: { `fields`: [{ `name`: ``"name"`` ; `type`: ``"string"``  }, { `docs`: [``"Any other data that you may want to put in here"``] ; `name`: ``"uri"`` ; `type`: { `option`: ``"string"``  }  }] ; `kind`: ``"struct"``  }  }, { `name`: ``"InitializeProposalArgsV0"`` ; `type`: { `fields`: [{ `name`: ``"name"`` ; `type`: ``"string"``  }, { `name`: ``"uri"`` ; `type`: ``"string"``  }, { `name`: ``"maxChoicesPerVoter"`` ; `type`: ``"u16"``  }, { `name`: ``"choices"`` ; `type`: { `vec`: { `defined`: ``"ChoiceArg"``  }  }  }, { `name`: ``"tags"`` ; `type`: { `vec`: ``"string"``  }  }] ; `kind`: ``"struct"``  }  }] |
| `version` | ``"0.1.0"`` |

#### Defined in

organization.ts:1

## Variables

### IDL

• `Const` **IDL**: [`Organization`](organization.md#organization)

#### Defined in

organization.ts:235
