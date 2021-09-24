import { Coin, isTxError, LCDClient, MnemonicKey, Msg, MsgInstantiateContract, MsgStoreCode, StdFee, Wallet, WasmAPI } from "@terra-money/terra.js";
import info from "./constant";
import * as path from "path";
import * as fs from "fs";

(async () => {
  // Create LCDClient for Bombay-11 TestNet
  const terra: LCDClient = new LCDClient({
    URL: info.NETWORK,
    chainID: info.CHAIN_ID
  });

  // Get deployer wallet
  const wallet = terra.wallet(new MnemonicKey({mnemonic: info.WALLET_SEEDS}));
  console.log("Wallet: ", wallet.key.accAddress);

  // Deploy wasm to testnet
  console.log("Start deploy wasm ");
  const codeId = await storeCode(
    terra, 
    wallet, 
    path.resolve(__dirname, "../artifacts/interview_challenge.wasm")
  );
  console.log("Done");
  console.log("\nCodeId: ", codeId);

  // Instantiate contract
  console.log("\n\nInstantiate contract");
  const result = await instantiateContract(terra, wallet, wallet, codeId, {
    owner: wallet.key.accAddress,
    users: [],
  });
  const contractAddress = result.logs[0].events.find((event) => {
    return event.type == "instantiate_contract";
  })?.attributes.find((attribute) => { 
    return attribute.key == "contract_address"; 
  })?.value as string;
  console.log(" Done!", `${"contractAddress"}=${contractAddress}`);
  
})();

/**
 * @notice Upload contract code to LCDClient. Return code ID.
 */
async function storeCode(
  terra: LCDClient,
  deployer: Wallet,
  filepath: string
): Promise<number> {
  const code = fs.readFileSync(filepath).toString("base64");
  const result = await sendTransaction(terra, deployer, [
    new MsgStoreCode(deployer.key.accAddress, code),
  ]);
  return parseInt(result.logs[0].eventsByType.store_code.code_id[0]);
}

/**
 * @notice Send a transaction. Return result if successful, throw error if failed.
 */
async function sendTransaction(
  terra: LCDClient,
  sender: Wallet,
  msgs: Msg[],
  verbose = false
) {
  const tx = await sender.createAndSignTx({
    msgs,
    fee: new StdFee(2000000, [new Coin("uusd", 1000000)]),
  });

  const result = await terra.tx.broadcast(tx);

  // Print the log info
  if (verbose) {
    console.log("\nTxHash:", result.txhash);
    try {
      console.log("Raw log:",
        JSON.stringify(JSON.parse(result.raw_log), null, 2)
      );
    } catch {
      console.log("Failed to parse log! Raw log:", result.raw_log);
    }
  }

  if (isTxError(result)) {
    throw new Error(
      "Transaction failed!" +
        `\n${"code"}: ${result.code}` +
        `\n${"codespace"}: ${result.codespace}` +
        `\n${"raw_log"}: ${result.raw_log}`
    );
  }

  return result;
}

/**
 * @notice Instantiate a contract from an existing code ID. Return contract address.
 */
export async function instantiateContract(
  terra: LCDClient,
  deployer: Wallet,
  admin: Wallet,
  codeId: number,
  instantiateMsg: Record<string, unknown>
) {
  const result = await sendTransaction(terra, deployer, [
    new MsgInstantiateContract(
      deployer.key.accAddress,
      admin.key.accAddress,
      codeId,
      instantiateMsg
    ),
  ]);
  return result;
};
