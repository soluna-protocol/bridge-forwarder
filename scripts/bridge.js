import { LCDClient, MsgExecuteContract, MnemonicKey, isTxError, Coins} from '@terra-money/terra.js';
import * as fs from 'fs';
import fetch from 'isomorphic-fetch';
import { CHAIN_ID_SOLANA, hexToUint8Array, nativeToHexString } from "@certusone/wormhole-sdk";


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

// const contract = "terra1zpzp3ag2hlque7r72pxp0a3x0qyfam7lfzpqc2" // to_bytes code

const contract = 'terra1zam63ltnt76qjzsafchs6rw4kzasd0ap5fhtws'


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

let execute = new MsgExecuteContract(
  wallet.key.accAddress, // sender
  contract, // contract account address
  { 
    bridge: {} 
  }, // handle msg
);

let executeTx = await wallet.createAndSignTx({
  msgs: [execute]
});

let executeTxResult = await terra.tx.broadcast(executeTx);

console.log(executeTxResult)
