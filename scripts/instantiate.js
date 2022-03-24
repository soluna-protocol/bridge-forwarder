import { LCDClient, MsgInstantiateContract, MsgStoreCode, MnemonicKey, isTxError, Coins} from '@terra-money/terra.js';
import fetch from 'isomorphic-fetch';
import { CHAIN_ID_SOLANA, hexToUint8Array, nativeToHexString } from "@certusone/wormhole-sdk";
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

const wallet = terra.wallet(mk);

const code_id = 56401; 

const target = hexToUint8Array(
  nativeToHexString("B3Qnkdcv1aRkHFqp6UBRpdz2Addu27pN23xhUmRWwNPM", CHAIN_ID_SOLANA) ?? ""
);

console.log(Buffer.from(target).toString("base64"))

const instantiate = new MsgInstantiateContract(
  wallet.key.accAddress,
  wallet.key.accAddress,
  code_id, // code ID
  {
    receiver: wallet.key.accAddress,
    bank: "terra1t2eehshcueggptcge5prr4vx8wrztx3v8vwku7",
    bridge: "terra1pseddrv0yfsn76u4zxrjmtf45kdlmalswdv39a",
    target: Buffer.from(target).toString("base64"),
    period: 60,
  }, // InitMsg
);

const instantiateTx = await wallet.createAndSignTx({
  msgs: [instantiate],
});
const instantiateTxResult = await terra.tx.broadcast(instantiateTx);

console.log(instantiateTxResult);

if (isTxError(instantiateTxResult)) {
  throw new Error(
    `instantiate failed. code: ${instantiateTxResult.code}, codespace: ${instantiateTxResult.codespace}, raw_log: ${instantiateTxResult.raw_log}`
  );
}

const {
  instantiate_contract: { contract_address },
} = instantiateTxResult.logs[0].eventsByType;

console.log(contract_address)