export * from './Challenge'
export * from './Dapp'
export * from './Epoch'
export * from './EpochConfig'
export * from './Group'

import { Challenge } from './Challenge'
import { Dapp } from './Dapp'
import { Epoch } from './Epoch'
import { EpochConfig } from './EpochConfig'
import { Group } from './Group'

export const accountProviders = { Challenge, Dapp, Epoch, EpochConfig, Group }
