import { useState } from "react";
import { invoke } from "@tauri-apps/api/primitives";
import reactLogo from "./assets/react.svg";
import "./App.css";

function App() {
  const [privateKey, setPrivateKey] = useState("");
  const [programId, setProgramId] = useState("");
  const [functionName, setFunctionName] = useState("");
  const [magic_phrase, setMagicPhrase] = useState("");

  return (
    <div className="App dark-mode">
      <header className="App-header">
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
          <input
            className="styled-input"
            type="text"
            placeholder="magic_phrase"
            value={magic_phrase}
            onChange={(e) => setMagicPhrase(e.target.value)}
          ></input>
          <button
            className="styled-button"
            onClick={() =>
              call_execute(privateKey, programId, functionName, magic_phrase)
            }
          >
            Execute
          </button>
        </p>
      </header>
    </div>
  );
}

function call_execute(
  privateKey: string,
  programId: string,
  functionName: string,
  magic_phrase: string
) {
  invoke("execute_program", {
    private_key: "APrivateKey1zkpGowLYHT1mLL8atgSTdvzL1EwfB65CqD93zMvNT5aVDVS",
    program_id: "avail_ctf_countryman_37381.aleo",
    function_name: "main",
    input: "123321scalar",
  })
    .then((res) => {
      console.log(res);
    })
    .catch((err) => {
      console.error(err);
    });
}

export default App;
