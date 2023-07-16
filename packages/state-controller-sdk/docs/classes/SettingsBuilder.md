[@helium/state-controller-sdk - v0.0.1](../README.md) / [Exports](../modules.md) / SettingsBuilder

# Class: SettingsBuilder

## Table of contents

### Constructors

- [constructor](SettingsBuilder.md#constructor)

### Properties

- [nodes](SettingsBuilder.md#nodes)

### Methods

- [and](SettingsBuilder.md#and)
- [build](SettingsBuilder.md#build)
- [choicePercentage](SettingsBuilder.md#choicepercentage)
- [choiceVoteWeight](SettingsBuilder.md#choicevoteweight)
- [endTimestamp](SettingsBuilder.md#endtimestamp)
- [offsetFromStartTs](SettingsBuilder.md#offsetfromstartts)
- [or](SettingsBuilder.md#or)
- [resolved](SettingsBuilder.md#resolved)
- [top](SettingsBuilder.md#top)

## Constructors

### constructor

• **new SettingsBuilder**()

## Properties

### nodes

• **nodes**: `any`[] = `[]`

#### Defined in

[packages/state-controller-sdk/src/settingsBuilder.ts:10](https://github.com/DeWiCats/modular-governance/blob/9f88f14/packages/state-controller-sdk/src/settingsBuilder.ts#L10)

## Methods

### and

▸ **and**(`left`, `right`): [`SettingsBuilder`](SettingsBuilder.md)

#### Parameters

| Name | Type |
| :------ | :------ |
| `left` | [`SettingsBuilder`](SettingsBuilder.md) |
| `right` | [`SettingsBuilder`](SettingsBuilder.md) |

#### Returns

[`SettingsBuilder`](SettingsBuilder.md)

#### Defined in

[packages/state-controller-sdk/src/settingsBuilder.ts:58](https://github.com/DeWiCats/modular-governance/blob/9f88f14/packages/state-controller-sdk/src/settingsBuilder.ts#L58)

___

### build

▸ **build**(): `any`[]

#### Returns

`any`[]

#### Defined in

[packages/state-controller-sdk/src/settingsBuilder.ts:72](https://github.com/DeWiCats/modular-governance/blob/9f88f14/packages/state-controller-sdk/src/settingsBuilder.ts#L72)

___

### choicePercentage

▸ **choicePercentage**(`percentage`): [`SettingsBuilder`](SettingsBuilder.md)

#### Parameters

| Name | Type |
| :------ | :------ |
| `percentage` | `number` \| `BN` |

#### Returns

[`SettingsBuilder`](SettingsBuilder.md)

#### Defined in

[packages/state-controller-sdk/src/settingsBuilder.ts:41](https://github.com/DeWiCats/modular-governance/blob/9f88f14/packages/state-controller-sdk/src/settingsBuilder.ts#L41)

___

### choiceVoteWeight

▸ **choiceVoteWeight**(`weightThreshold`): [`SettingsBuilder`](SettingsBuilder.md)

#### Parameters

| Name | Type |
| :------ | :------ |
| `weightThreshold` | `BN` |

#### Returns

[`SettingsBuilder`](SettingsBuilder.md)

#### Defined in

[packages/state-controller-sdk/src/settingsBuilder.ts:33](https://github.com/DeWiCats/modular-governance/blob/9f88f14/packages/state-controller-sdk/src/settingsBuilder.ts#L33)

___

### endTimestamp

▸ **endTimestamp**(`endTs`): [`SettingsBuilder`](SettingsBuilder.md)

#### Parameters

| Name | Type |
| :------ | :------ |
| `endTs` | `BN` |

#### Returns

[`SettingsBuilder`](SettingsBuilder.md)

#### Defined in

[packages/state-controller-sdk/src/settingsBuilder.ts:19](https://github.com/DeWiCats/modular-governance/blob/9f88f14/packages/state-controller-sdk/src/settingsBuilder.ts#L19)

___

### offsetFromStartTs

▸ **offsetFromStartTs**(`offset`): [`SettingsBuilder`](SettingsBuilder.md)

#### Parameters

| Name | Type |
| :------ | :------ |
| `offset` | `BN` |

#### Returns

[`SettingsBuilder`](SettingsBuilder.md)

#### Defined in

[packages/state-controller-sdk/src/settingsBuilder.ts:26](https://github.com/DeWiCats/modular-governance/blob/9f88f14/packages/state-controller-sdk/src/settingsBuilder.ts#L26)

___

### or

▸ **or**(`left`, `right`): [`SettingsBuilder`](SettingsBuilder.md)

#### Parameters

| Name | Type |
| :------ | :------ |
| `left` | [`SettingsBuilder`](SettingsBuilder.md) |
| `right` | [`SettingsBuilder`](SettingsBuilder.md) |

#### Returns

[`SettingsBuilder`](SettingsBuilder.md)

#### Defined in

[packages/state-controller-sdk/src/settingsBuilder.ts:65](https://github.com/DeWiCats/modular-governance/blob/9f88f14/packages/state-controller-sdk/src/settingsBuilder.ts#L65)

___

### resolved

▸ **resolved**(`choices`): [`SettingsBuilder`](SettingsBuilder.md)

#### Parameters

| Name | Type |
| :------ | :------ |
| `choices` | `number`[] |

#### Returns

[`SettingsBuilder`](SettingsBuilder.md)

#### Defined in

[packages/state-controller-sdk/src/settingsBuilder.ts:12](https://github.com/DeWiCats/modular-governance/blob/9f88f14/packages/state-controller-sdk/src/settingsBuilder.ts#L12)

___

### top

▸ **top**(`n?`): [`SettingsBuilder`](SettingsBuilder.md)

#### Parameters

| Name | Type | Default value |
| :------ | :------ | :------ |
| `n` | `number` | `1` |

#### Returns

[`SettingsBuilder`](SettingsBuilder.md)

#### Defined in

[packages/state-controller-sdk/src/settingsBuilder.ts:51](https://github.com/DeWiCats/modular-governance/blob/9f88f14/packages/state-controller-sdk/src/settingsBuilder.ts#L51)
