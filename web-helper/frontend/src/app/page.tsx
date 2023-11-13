"use client";

import { Editor } from "@monaco-editor/react";
import { useState } from "react";
import getGooseContract from "../utils/contract";

export default function Home() {
  const [code, setCode] = useState("");
  const [privateKey, setPrivateKey] = useState("");

  return (
    <main className="flex min-h-screen flex-col items-center justify-between pt-10 p-20 bg-gray-900 text-gray-100 font-sans">
      <h1 className="text-4xl font-normal pb-5">Goose Contract Generator</h1>
      <div className="flex flex-col items-center justify-center w-full p-8 bg-gray-800 rounded-lg shadow-lg filter drop-shadow-lg fill-current text-gray-100 text-center">
        <div className="flex flex-col items-center justify-center w-full">
          <Editor
            height="75vh"
            language="c"
            options={{
              readOnly: true,
              fontFamily: "monospace",
              minimap: { enabled: false },
              scrollbar: { vertical: "hidden" },
              fontSize: 14,
            }}
            theme="vs-dark"
            value={code}
          />
          <div className="flex flex-row items-center justify-between w-full">
            <div>
              <label className="text-lg font-normal pt-5">Private Key: </label>
              {privateKey ? (
                <text className="text-lg bg-gray-700 rounded-lg shadow-lg filter drop-shadow-lg p-2 fill-current text-gray-100 text-center">
                  {privateKey}
                </text>
              ) : null}
            </div>
            <button
              className="p-2 mt-4 bg-gray-700 rounded-lg shadow-lg filter drop-shadow-lg fill-current text-gray-100 text-center"
              style={{ width: "45%" }}
              onClick={async () => {
                let a = await getGooseContract();
                setPrivateKey(a.private_key);
                setCode(a.contract);
              }}
            >
              Get Challenge Contract
            </button>
          </div>
        </div>
      </div>
    </main>
  );
}
