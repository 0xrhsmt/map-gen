import { SecretNetworkClient, Wallet } from "secretjs";
import * as fs from "fs";
import dotenv from "dotenv";
dotenv.config();

async function main() {
  const wallet = new Wallet(process.env.MNEMONIC);
  const contract_wasm = fs.readFileSync("../contract.wasm.gz");

  const secretjs = new SecretNetworkClient({
    chainId: "pulsar-3",
    url: "https://api.pulsar3.scrttestnet.com",
    wallet: wallet,
    walletAddress: wallet.address,
  });

  let tx = await secretjs.tx.compute.storeCode(
    {
      sender: wallet.address,
      wasm_byte_code: contract_wasm,
      source: "",
      builder: "",
    },
    {
      gasLimit: 4_000_000,
    }
  );
  if (tx.arrayLog === undefined) {
    throw new Error("No arrayLog in response");
  }

  const codeId = tx.arrayLog.find(
    (log) => log.type === "message" && log.key === "code_id"
  )!.value;

  const res = await secretjs.query.compute.codeHashByCodeId({
    code_id: codeId,
  });
  const contractCodeHash = res.code_hash;

  const initMsg = { count: 0 };
  let tx2 = await secretjs.tx.compute.instantiateContract(
    {
      code_id: codeId,
      sender: wallet.address,
      code_hash: contractCodeHash,
      init_msg: initMsg,
      label: "secret raffle" + Math.ceil(Math.random() * 10000),
    },
    {
      gasLimit: 400_000,
    }
  );
  if (tx2.arrayLog === undefined) {
    throw new Error("No arrayLog in response");
  }

  const contractAddress = tx2.arrayLog.find(
    (log) => log.type === "message" && log.key === "contract_address"
  )!.value;

  console.log("codeId: ", codeId);
  console.log("contractCodeHash", contractCodeHash);
  console.log("contractAddress", contractAddress);
}
await main();
