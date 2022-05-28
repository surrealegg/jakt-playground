import Editor from "@monaco-editor/react";
import Ansi from "ansi-to-react";
import { useEffect, useRef, useState } from "react";
import { FiSettings, FiPlay, FiMoon, FiSun } from "react-icons/fi";
import { classNames, getGist } from "./Utils";
import { KeyCode, editor } from "monaco-editor";
import { useSearchParams } from "react-router-dom";

interface CompilerResponse {
  code: number;
  stdout: string;
  stderr: string;
}

const DEFAULT_CODE = `function main() {
    println("Hello Friends! :^)")
}
`;

export function App() {
  const [searchParams] = useSearchParams();
  const [code, setCode] = useState(0);
  const [status, setStatus] = useState(0);
  const [statusText, setStatusText] = useState("");
  const [stdout, setStdout] = useState("");
  const [stderr, setStderr] = useState("");
  const [ran, setRan] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const editorRef = useRef<editor.IStandaloneCodeEditor>(null);
  const [theme, setTheme] = useState<Theme>("light");

  useEffect(() => {
    const temp = localStorage.getItem("theme");
    if (temp === "light" || temp === "dark") setTheme(temp);
  }, []);

  async function run(execute: boolean) {
    setRan(true);
    setIsLoading(true);

    try {
      const response = await fetch(
        `${import.meta.env.VITE_SERVER_URL}/compile?execute=${execute}`,
        {
          body: editorRef.current?.getValue(),
          method: "POST",
        }
      );

      setStatus(response.status);
      if (response.status !== 200) {
        setStatusText(response.statusText);
        setIsLoading(false);
        return;
      }

      const json: CompilerResponse = await response.json();
      setStdout(json.stdout);
      setStderr(json.stderr);
      setCode(json.code);
    } catch (_) {
      setStatus(503);
      setStatusText("Service Unavailable");
    }

    setIsLoading(false);
  }

  function onMount(editor: editor.IStandaloneCodeEditor) {
    editor.addCommand(KeyCode.F5, () => {
      run(true);
    });

    editor.addCommand(KeyCode.F6, () => {
      run(false);
    });

    const id = searchParams.get("gist");
    if (id)
      getGist(id, (content) => editor.setValue(content)).catch(console.error);
    else editor.setValue(DEFAULT_CODE);

    // @ts-ignore
    editorRef.current = editor;
  }

  function changeTheme() {
    const newTheme = theme === "dark" ? "light" : "dark";
    setTheme(newTheme);
    localStorage.setItem("theme", newTheme);
  }

  return (
    <div className={`${theme} h-full`}>
      <div className="flex flex-col h-full overflow-hidden dark:bg-dark">
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
            <span className="jaktKeybind">(F5)</span>
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
            <span className="jaktKeybind">(F6)</span>
          </button>
          <button
            onClick={() => changeTheme()}
            className={theme === "dark" ? "text-white" : "text-black"}
          >
            {theme === "light" ? <FiMoon size={18} /> : <FiSun size={18} />}
          </button>
        </div>
        <div
          className={`grid ${
            ran ? "grid-cols-2" : "grid-cols-1"
          } w-full h-full gap-2`}
        >
          <Editor
            defaultValue="// Loading..."
            onMount={onMount}
            theme={theme === "dark" ? "vs-dark" : "vs"}
            defaultLanguage="jakt"
          />
          {ran && (
            <div className="relative dark:text-white">
              <pre className="overflow-y-scroll absolute top-0 left-0 right-0 bottom-0 w-full h-full">
                {!isLoading &&
                  (status !== 200 ? (
                    <p className="font-bold text-red-600">
                      {status}: {statusText}
                    </p>
                  ) : (
                    <>
                      <p>
                        <span className="font-bold">
                          Program Exited with code:
                        </span>{" "}
                        {code}
                      </p>
                      <p className="font-bold">Stdout:</p>
                      <Ansi className="break-words">{stdout}</Ansi>
                      <p className="font-bold">Stderr:</p>
                      <Ansi className="break-words">{stderr}</Ansi>
                    </>
                  ))}
              </pre>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
