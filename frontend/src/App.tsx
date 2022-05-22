import Editor from "@monaco-editor/react";
import Ansi from "ansi-to-react";
import { useState } from "react";
import { FiSettings, FiPlay } from "react-icons/fi";
import { classNames } from "./Utils";

interface CompilerResponse {
  code: number;
  stdout: string;
  stderr: string;
}

const DEFAULT_CODE = `function main() {
  println(":^)");
}
`;

export function App() {
  const [input, setInput] = useState(DEFAULT_CODE);
  const [code, setCode] = useState(0);
  const [stdout, setStdout] = useState("");
  const [stderr, setStderr] = useState("");
  const [ran, setRan] = useState(false);
  const [isLoading, setIsLoading] = useState(false);

  async function run(execute: boolean) {
    setRan(true);
    setIsLoading(true);
    const response = await fetch(
      `${import.meta.env.VITE_SERVER_URL}/compile?execute=${execute}`,
      {
        body: input,
        method: "POST",
      }
    );
    const json: CompilerResponse = await response.json();
    setStdout(json.stdout);
    setStderr(json.stderr);
    setCode(json.code);
    setIsLoading(false);
  }

  return (
    <div className="flex flex-col h-full">
      <div className="flex flex-row space-x-4 p-4 box-border">
        <button
          onClick={() => run(true)}
          disabled={isLoading}
          className={classNames({
            jaktBtn: true,
            loading: isLoading,
          })}
        >
          <FiPlay className="mr-2" /> Run
        </button>
        <button
          onClick={() => run(false)}
          disabled={isLoading}
          className={classNames({
            jaktBtn: true,
            loading: isLoading,
          })}
        >
          <FiSettings className="mr-2" /> Compile
        </button>
      </div>
      <div
        className={`grid ${ran ? "grid-cols-2" : "grid-cols-1"} h-full gap-2`}
      >
        <div>
          <Editor
            onChange={(value) => {
              if (value) setInput(value);
            }}
            className="h-full"
            theme="vs"
            defaultLanguage="jakt"
            defaultValue={input}
          />
        </div>
        {ran && (
          <pre className="overflow-auto">
            {!isLoading && (
              <div className="space-y-4">
                <p className="font-bold ">Program Exited with code: {code}</p>
                <p className="font-bold ">Stdout:</p>
                <Ansi className="break-words">{stdout}</Ansi>
                <p className="font-bold ">Stderr:</p>
                <Ansi className="break-words">{stderr}</Ansi>
              </div>
            )}
          </pre>
        )}
      </div>
    </div>
  );
}
