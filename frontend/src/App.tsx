import Editor from "@monaco-editor/react";
import Ansi from "ansi-to-react";
import { useState } from "react";
import { FiSettings, FiPlay } from "react-icons/fi";

interface CompilerResponse {
  success: boolean;
  output: string;
  error: string;
}

const DEFAULT_CODE = `function main() {
  println(":^)");
}
`;

export function App() {
  const [code, setCode] = useState(DEFAULT_CODE);
  const [output, setOutput] = useState("");
  const [ran, setRan] = useState(false);

  async function run(execute: boolean) {
    setRan(true);
    const response = await fetch(
      `http://localhost:8080/compile?execute=${execute}`,
      {
        body: code,
        method: "POST",
      }
    );
    const json: CompilerResponse = await response.json();
    setOutput(json.success ? json.output : `${json.output}${json.error}`);
  }

  return (
    <div className="flex flex-col h-full">
      <div className="flex flex-row space-x-4 p-4 box-border">
        <button onClick={() => run(true)} className="jakt-btn">
          <FiPlay className="mr-2" /> Run
        </button>
        <button onClick={() => run(false)} className="jakt-btn">
          <FiSettings className="mr-2" /> Compile
        </button>
      </div>
      <div
        className={`grid ${ran ? "grid-cols-2" : "grid-cols-1"} h-full gap-2`}
      >
        <div>
          <Editor
            onChange={(value) => {
              if (value) setCode(value);
            }}
            className="h-full"
            theme="vs"
            defaultLanguage="jakt"
            defaultValue={code}
          />
        </div>
        {ran && (
          <pre className="overflow-auto">
            <Ansi className="break-words">{output}</Ansi>
          </pre>
        )}
      </div>
    </div>
  );
}
