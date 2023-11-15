import { useState } from "react";
import { invoke } from "@tauri-apps/api/primitives";
import reactLogo from "./assets/react.svg";
import "./App.css";

function App() {
  const [privateKey, setPrivateKey] = useState("");
  const [programId, setProgramId] = useState("");
  const [functionName, setFunctionName] = useState("");

  return (
    <div className="App dark-mode">
      <header className="App-header">
        <img src={reactLogo} className="App-logo" alt="logo" />
        <p>
          <br />
          <br />
          <input
            className="styled-input"
            type="text"
            placeholder="APrivateKey1zkp..."
            value={privateKey}
            onChange={(e) => setPrivateKey(e.target.value)}
          />
          <br />
          <input
            className="styled-input"
            placeholder="avail_ctf_countryman_xxx.aleo"
            type="text"
            value={programId}
            onChange={(e) => setProgramId(e.target.value)}
          />
          <br />
          <input
            className="styled-input"
            type="text"
            placeholder="some_function"
            value={functionName}
            onChange={(e) => setFunctionName(e.target.value)}
          />
          <br />
          <button className="styled-button">Execute</button>
        </p>
      </header>
    </div>
  );
}

export default App;
