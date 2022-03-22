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

const token = "terra1spgmfack053u53e6mj47xq3rxfe4cx8cgtlgpq"

const bridge = "terra1pseddrv0yfsn76u4zxrjmtf45kdlmalswdv39a"

const target = hexToUint8Array(
  nativeToHexString("B3Qnkdcv1aRkHFqp6UBRpdz2Addu27pN23xhUmRWwNPM", CHAIN_ID_SOLANA) ?? ""
);

console.log(Buffer.from(target).toString("base64"))

let execute1 = new MsgExecuteContract(
  wallet.key.accAddress,
  token,
  {
    increase_allowance: {
      spender: bridge,
      amount: "1000000",
      expires: {
        never: {},
      },
    },
  },
  {}
)

// let execute2 = new MsgExecuteContract(
//   wallet.key.accAddress,
//   bridge,
//   {
//     initiate_transfer: {
//       asset: {
//         amount: "1000000",
//         info: {
//           token: {
//             contract_addr: token,
//           },
//         },
//       },
//       recipient_chain: CHAIN_ID_SOLANA,
//       recipient: Buffer.from(target).toString("base64"),
//       fee: "0",
//       nonce: 69,
//     },
//   },
//   {}
// )

let executeTx = await wallet.createAndSignTx({
  msgs: [execute1]
});

let executeTxResult = await terra.tx.broadcast(executeTx);

console.log(executeTxResult)
