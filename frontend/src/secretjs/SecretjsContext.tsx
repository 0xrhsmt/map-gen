import React, { createContext, useCallback, useEffect, useState } from "react";
import { SecretNetworkClient } from "secretjs";

export type SecretjsContextType = {
  secretjs?: SecretNetworkClient | null;
  secretAddress?: string | null;
  isWalletConnected: boolean;
  connectWallet?: () => void;
  disconnectWallet?: () => void;
};

export const SecretjsContext = createContext<SecretjsContextType>({});
const SECRET_CHAIN_ID = "pulsar-3";
const SECRET_LCD = "https://api.pulsar3.scrttestnet.com";

export const SecretjsContextProvider: React.FC<React.PropsWithChildren> = ({
  children,
}) => {
  const [secretjs, setSecretjs] = useState<SecretNetworkClient | null>(null);
  const [secretAddress, setSecretAddress] = useState<string | null>("");

  const setupKeplr = useCallback(async () => {
    if (
      !window.keplr ||
      !window.getEnigmaUtils ||
      !window.getOfflineSignerOnlyAmino
    ) {
      alert("Please install keplr extension");
      return;
    }

    await window.keplr.enable(SECRET_CHAIN_ID);
    window.keplr.defaultOptions = {
      sign: {
        preferNoSetFee: false,
        disableBalanceCheck: true,
      },
    };

    const keplrOfflineSigner =
      window.getOfflineSignerOnlyAmino(SECRET_CHAIN_ID);
    const accounts = await keplrOfflineSigner.getAccounts();

    const secretAddress = accounts[0].address;

    const secretjs = new SecretNetworkClient({
      url: SECRET_LCD,
      chainId: SECRET_CHAIN_ID,
      wallet: keplrOfflineSigner,
      walletAddress: secretAddress,
      encryptionUtils: window.getEnigmaUtils(SECRET_CHAIN_ID),
    });

    setSecretAddress(secretAddress);
    setSecretjs(secretjs);
  }, []);

  const connectWallet = useCallback(async () => {
    try {
      if (!window.keplr) {
        console.log("intall keplr!");
      } else {
        await setupKeplr();
        localStorage.setItem("keplrAutoConnect", "true");
      }
    } catch (error) {
      alert(
        "An error occurred while connecting to the wallet. Please try again."
      );
    }
  }, [setupKeplr]);

  const disconnectWallet = useCallback(() => {
    // reset secretjs and secretAddress
    setSecretAddress("");
    setSecretjs(null);

    // disable auto connect
    localStorage.setItem("keplrAutoConnect", "false");

    // console.log for success
    console.log("Wallet disconnected!");
  }, []);

  useEffect(() => {
    const autoConnect = localStorage.getItem("keplrAutoConnect");

    if (autoConnect === "true") {
      connectWallet();
    }
  }, [connectWallet]);

  return (
    <SecretjsContext.Provider
      value={{
        secretjs,
        secretAddress,
        connectWallet,
        disconnectWallet,
        isWalletConnected: !!secretjs,
      }}
    >
      {children}
    </SecretjsContext.Provider>
  );
};
