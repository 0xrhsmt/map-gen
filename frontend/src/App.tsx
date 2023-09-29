import { useEffect } from "react";

import { useSecretjs } from "./secretjs/useSecretjs";
import { Maps } from "./components/Map";

type NavBarProps = {
  isLoading: boolean;
  isConnected: boolean;
  clearMap: (args: { withQuery?: boolean }) => void;
};

const NavBar: React.FC<NavBarProps> = ({
  isLoading,
  isConnected,
  clearMap,
}) => {
  return (
    <div className="navbar bg-base-100">
      <div className="navbar-start"></div>
      <div className="navbar-center">
        <a className="btn btn-ghost normal-case text-xl">map-randomgen</a>
      </div>
      <div className="navbar-end">
        {isConnected && (
          <button
            className="btn btn-secondary"
            onClick={() => clearMap({ withQuery: true })}
          >
            {isLoading ? (
              <span className="loading loading-bars loading-sm"></span>
            ) : (
              "CLEAR MAP"
            )}
          </button>
        )}
      </div>
    </div>
  );
};

function App() {
  const {
    isLoading,
    wallet: { isConnected, connect: connectWallet },
    execute: { generate: generateMap, clear: clearMap },
    query: { queryMaps },
    state: { maps },
  } = useSecretjs();

  useEffect(() => {
    queryMaps();
  }, [queryMaps]);

  return (
    <>
      <div className="h-screen">
        <NavBar
          isLoading={isLoading}
          isConnected={isConnected}
          clearMap={clearMap}
        />

        <div className="px-8 py-1">
          {maps && maps.length > 0 ? (
            <Maps maps={maps} />
          ) : (
            <div
              className="flex justify-center items-center w-full"
              style={{ height: "calc(100vh - 64px)" }}
            >
              <div className="text-3xl mb-24">Let's generate our maps</div>
            </div>
          )}
        </div>

        <div className="fixed bottom-8 -translate-x-1/2 left-1/2">
          {isConnected ? (
            <button
              className="btn btn-lg btn-wide btn-primary"
              onClick={() => generateMap({ withQuery: true })}
            >
              {isLoading ? (
                <span className="loading loading-bars loading-md"></span>
              ) : (
                "GENERATE MAP"
              )}
            </button>
          ) : (
            <button
              className="btn btn-lg btn-wide btn-accent"
              onClick={connectWallet}
            >
              CONNECT WALLET
            </button>
          )}
        </div>
      </div>
    </>
  );
}

export default App;
