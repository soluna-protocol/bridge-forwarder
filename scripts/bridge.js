import { LCDClient, MsgExecuteContract, MnemonicKey, isTxError, Coins} from '@terra-money/terra.js';
import * as fs from 'fs';
import fetch from 'isomorphic-fetch';

// Fetch gas prices and convert to `Coin` format.
const gasPrices = await (await fetch('https://bombay-fcd.terra.dev/v1/txs/gas_prices')).json();
const gasPricesCoins = new Coins(gasPrices);

const terra = new LCDClient({
  URL: "https://bombay-lcd.terra.dev/",
  chainID: "bombay-12",
  gasPrices: gasPricesCoins,
  gasAdjustment: "1.5",
  gas: 10000000,
});

const mk = new MnemonicKey({
  mnemonic: 'popular raven ginger mechanic blind celery uncle will upon tilt midnight cannon wheat issue picture grass either family scheme world salad rice obtain auction'
})

// const mk = new MnemonicKey({
//   mnemonic: 'satisfy adjust timber high purchase tuition stool faith fine install that you unaware feed domain license impose boss human eager hat rent enjoy dawn'
// })

// // connect to localterra
// const terra = new LCDClient({
//   URL: 'http://localhost:1317',
//   chainID: 'localterra'
// });

const wallet = terra.wallet(mk);

const contract = "terra1hf37ztxxne8tlv6dmzl6370ndyjg8f7sxm6mkr"

// let execute = new MsgExecuteContract(
//   wallet.key.accAddress, // sender
//   contract, // contract account address
//   { 
//     approve_bridge: {
//       amount: "53",
//     } 
//   }, // handle msg
// );
// let executeTx = await wallet.createAndSignTx({
//   msgs: [execute]
// });

// let executeTxResult = await terra.tx.broadcast(executeTx);

const h = "6e653901c71453b1b317b7d7cface7ede623fd4a193f5fd516a86e409fafe8ab"

const recipeintAddress = new Uint8Array(Buffer.from(h, "hex"))

console.log(recipeintAddress)

let execute = new MsgExecuteContract(
  wallet.key.accAddress, // sender
  contract, // contract account address
  { 
    bridge: {
      amount: "100",
      recipient_chain: 1,
      recipient: Buffer.from(recipeintAddress).toString("base64"),
      nonce: 69,
    } 
  }, // handle msg
);

let executeTx = await wallet.createAndSignTx({
  msgs: [execute]
});

let executeTxResult = await terra.tx.broadcast(executeTx);

console.log(executeTxResult)
