import 'dotenv';
import { RadixDappToolkit, RadixNetwork } from '@radixdlt/radix-dapp-toolkit';

RadixDappToolkit({
  dAppDefinitionAddress: process.env.RADIX_TESTNET_ACCOUNT,
  networkId: RadixNetwork.RCnetV3,
  applicationName: 'Radix Web3 dApp',
  applicationVersion: '1.0.0',
})
t 
