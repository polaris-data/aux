// Explicit public market reads; no credentials, subscriptions or writes.
import { PhoenixClient, ExtendedClient } from '../js/index.mjs';
const phoenix = new PhoenixClient();
const extended = new ExtendedClient({ network: 'testnet' });
try {
  console.log('Phoenix mainnet markets:', (await phoenix.listMarkets()).length);
  console.log('Extended testnet markets:', (await extended.getMarkets()).data.length);
} finally {
  await Promise.all([phoenix.close(), extended.close()]);
}
