import { useCallback, useContext, useState } from "react";
import { SecretjsContext } from "./SecretjsContext";

import deployment from "../../../contracts/latest-deployment.json";

const contractCodeHash = deployment.contractCodeHash;
const contractAddress = deployment.contractAddress;

export const useSecretjs = () => {
  const {
    secretjs,
    secretAddress,
    connectWallet,
    disconnectWallet,
    isWalletConnected,
  } = useContext(SecretjsContext);
  const [maps, setMaps] = useState<string[]>([]);

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

  const generate = useCallback(
    async ({ withQuery }: { withQuery?: boolean }) => {
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
        { gasLimit: 3000_000 }
      );
      if (withQuery) {
        await queryMaps();
      }
    },
    [queryMaps, secretAddress, secretjs]
  );

  return {
    wallet: {
      isConnected: isWalletConnected,
      connect: connectWallet,
      disconnect: disconnectWallet,
    },
    execute: {
      generate,
    },
    query: {
      queryMaps,
    },
    state: {
      maps,
    },
  };
};
