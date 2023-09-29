import { useCallback, useContext, useState } from "react";
import { SecretjsContext } from "./SecretjsContext";

import deployment from "../../../contracts/latest-deployment.json";

const contractCodeHash = deployment.contractCodeHash;
const contractAddress = deployment.contractAddress;

export const useSecretjs = () => {
  const { secretjs, secretAddress, connectWallet, disconnectWallet } =
    useContext(SecretjsContext);
  const [count, setCount] = useState<number | null>(null);
  const [maps, setMaps] = useState<string[]>([]);

  const increment = useCallback(async () => {
    if (!secretjs || !secretAddress) {
      return;
    }

    const tx = await secretjs.tx.compute.executeContract(
      {
        sender: secretAddress,
        contract_address: contractAddress,
        msg: {
          increment: {},
        },
        code_hash: contractCodeHash,
      },
      { gasLimit: 100_000 }
    );
    console.debug(tx);
  }, [secretAddress, secretjs]);

  const generate = useCallback(async () => {
    if (!secretjs || !secretAddress) {
      return;
    }

    const tx = await secretjs.tx.compute.executeContract(
      {
        sender: secretAddress,
        contract_address: contractAddress,
        msg: {
          generate: {},
        },
        code_hash: contractCodeHash,
      },
      { gasLimit: 100_000 }
    );
    console.debug(tx);
  }, [secretAddress, secretjs]);

  const queryCount = useCallback(async () => {
    if (!secretjs || !secretAddress) {
      return;
    }

    const tx = (await secretjs.query.compute.queryContract({
      contract_address: contractAddress,
      code_hash: contractCodeHash,
      query: {
        get_count: {},
      },
    })) as { count: number };

    setCount(tx.count);

    return tx.count;
  }, [secretAddress, secretjs]);

  const queryMaps = useCallback(async () => {
    if (!secretjs || !secretAddress) {
      return;
    }

    const tx = (await secretjs.query.compute.queryContract({
      contract_address: contractAddress,
      code_hash: contractCodeHash,
      query: {
        get_maps: {},
      },
    })) as { maps: string[] };

    setMaps(tx.maps);

    return tx.maps;
  }, [secretAddress, secretjs]);

  return {
    connectWallet,
    disconnectWallet,
    increment,
    generate,
    queryCount,
    queryMaps,
    count,
    maps,
  };
};
