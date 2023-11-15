"use client";

import { Editor } from "@monaco-editor/react";
import { useState } from "react";
import getGooseContract from "../utils/contract";

export default function Home() {
  const [code, setCode] = useState("");
  const [privateKey, setPrivateKey] = useState("");

  return (
    <main className="flex min-h-screen flex-col items-center justify-between pt-10 p-20 bg-gray-900 text-gray-100 font-sans">
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
          <div className="flex flex-row items-center justify-between w-full mt-5">
            <button
              className="text-lg bg-gray-700 rounded-lg shadow-lg filter drop-shadow-lg p-2 fill-current text-gray-100 text-center hover:bg-gray-600"
              style={{ width: "45%" }}
              onClick={async () => {
                getGooseContract()
                  .then((a) => {
                    setPrivateKey(a.pk);
                    setCode(a.code);
                  })
                  .catch((e) => {
                    alert(
                      "Keys have finished, please contact someone from the Avail team ASAP!"
                    );
                  });
              }}
            >
              Get Challenge Contract
            </button>

            {privateKey ? (
              <div className="flex flex-row items-center bg-gray-700 rounded-lg shadow-lg filter drop-shadow-lg pl-2`">
                <text
                  className="text-lg rounded-lg shadow-lg filter
                          drop-shadow-lg p-2 fill-current text-gray-100 text-center"
                >
                  {privateKey}
                </text>
              </div>
            ) : null}
          </div>
        </div>
      </div>
    </main>
  );
}
