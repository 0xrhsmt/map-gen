import { SecretNetworkClient, Wallet } from "secretjs";
import { deploy } from "../src/deploy.js";

type Deployment = {
  codeId: string;
  contractCodeHash: string | undefined;
  contractAddress: string;
};

async function main() {
  const wallet = new Wallet(process.env.MNEMONIC);

  const secretjs = new SecretNetworkClient({
    chainId: "pulsar-3",
    url: "https://api.pulsar3.scrttestnet.com",
    wallet: wallet,
    walletAddress: wallet.address,
  });

  const increment = async (deployment: Deployment) => {
    const tx = await secretjs.tx.compute.executeContract(
      {
        sender: wallet.address,
        contract_address: deployment.contractAddress,
        msg: {
          increment: {},
        },
        code_hash: deployment.contractCodeHash,
      },
      { gasLimit: 100_000 }
    );

    console.log(tx);
  };

  const queryCount = async (deployment: Deployment) => {
    let tx = await secretjs.query.compute.queryContract({
      contract_address: deployment.contractAddress,
      code_hash: deployment.contractCodeHash,
      query: {
        get_count: {},
      },
    });
    console.log(tx);
  };

  const deployment = await deploy();
  await increment(deployment);
  await queryCount(deployment);
}

await main();
