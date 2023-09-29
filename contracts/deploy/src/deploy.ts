import { SecretNetworkClient, Wallet } from "secretjs";
import * as fs from "fs";
import dotenv from "dotenv";
dotenv.config();

export async function deploy() {
  if (!process.env.MNEMONIC) {
    throw new Error("No MNEMONIC in .env");
  }
  const wallet = new Wallet(process.env.MNEMONIC);
  const contract_wasm = fs.readFileSync("../contract.wasm.gz");

  const secretjs = new SecretNetworkClient({
    chainId: "pulsar-3",
    url: "https://api.pulsar3.scrttestnet.com",
    wallet: wallet,
    walletAddress: wallet.address,
  });

  console.log("storing code...");

  let storeTx = await secretjs.tx.compute.storeCode(
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
  if (storeTx.arrayLog === undefined) {
    throw new Error("No arrayLog in response");
  }
  const codeId = storeTx.arrayLog.find(
    (log) => log.type === "message" && log.key === "code_id"
  )!.value;

  console.log("getting code hash...");

  const res = await secretjs.query.compute.codeHashByCodeId({
    code_id: codeId,
  });
  const contractCodeHash = res.code_hash;

  console.log("instantiating contract...");

  const initMsg = { count: 0 };
  let initTx = await secretjs.tx.compute.instantiateContract(
    {
      code_id: codeId,
      sender: wallet.address,
      code_hash: contractCodeHash,
      init_msg: initMsg,
      label: "map randomgen" + Math.ceil(Math.random() * 10000),
    },
    {
      gasLimit: 400_000,
    }
  );
  if (initTx.arrayLog === undefined) {
    throw new Error("No arrayLog in response");
  }
  const contractAddress = initTx.arrayLog.find(
    (log) => log.type === "message" && log.key === "contract_address"
  )!.value;

  const deployment = {
    codeId: codeId,
    contractCodeHash: contractCodeHash,
    contractAddress: contractAddress,
  };

  fs.writeFileSync("../latest-deployment.json", JSON.stringify(deployment));
  console.log("latest deployment is...\n", deployment);

  return deployment;
}
