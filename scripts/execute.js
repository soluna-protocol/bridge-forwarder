import { LCDClient, MsgExecuteContract, MnemonicKey, isTxError, Coins} from '@terra-money/terra.js';
import "dotenv/config";
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
  mnemonic: process.env.MNEMONIC
})

const wallet = terra.wallet(mk);

const contract = "terra1y23p9f3tjrsssmev5j0m2j5rn3re32zlxggfn6"

const execute = new MsgExecuteContract(
  wallet.key.accAddress, // sender
  contract, // contract account address
  { rebase: { } }, // handle msg
);

const executeTx = await wallet.createAndSignTx({
  msgs: [execute]
});

const executeTxResult = await terra.tx.broadcast(executeTx);

// let result = await terra.wasm.contractQuery(
//   contract,
//   { get_time: { } } // query msg
// );

console.log(executeTxResult)