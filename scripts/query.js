import { LCDClient, MsgInstantiateContract, MsgStoreCode, MnemonicKey, isTxError, Coins} from '@terra-money/terra.js';
import fetch from 'isomorphic-fetch';
import "dotenv/config";


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
  mnemonic: process.env.MNEMONIC
})

// // connect to localterra
// const terra = new LCDClient({
//   URL: 'http://localhost:1317',
//   chainID: 'localterra'
// });

const wallet = terra.wallet(mk);

// const result = await terra.wasm.contractQuery(
//   "terra1spgmfack053u53e6mj47xq3rxfe4cx8cgtlgpq",
//   { 
//     allowance: { 
//       owner: "terra1cw4swygjqhg05u07n7trv6hra76nzd47gweav4",
//       spender: "terra1pseddrv0yfsn76u4zxrjmtf45kdlmalswdv39a"
//     }
//   } // query msg
// );

// const result = await terra.wasm.contractQuery(
//   "terra1spgmfack053u53e6mj47xq3rxfe4cx8cgtlgpq",
//   { 
//     balance: { 
//       address: "terra18rzlpedq4aamu52u2lpcne6hwcld9cra5vsydl",
//     }
//   } // query msg
// );

const result = await terra.wasm.contractQuery(
  "terra1y23p9f3tjrsssmev5j0m2j5rn3re32zlxggfn6",
  { 
    get_time: {}
  } // query msg
);

console.log(result)