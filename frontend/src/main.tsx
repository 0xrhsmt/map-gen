import React from "react";
import ReactDOM from "react-dom/client";

import { SecretjsContextProvider } from "./secretjs/SecretjsContext.tsx";
import App from "./App.tsx";
import "./index.css";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <SecretjsContextProvider>
      <App />
    </SecretjsContextProvider>
  </React.StrictMode>
);
