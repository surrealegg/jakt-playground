import React from "react";
import ReactDOM from "react-dom/client";
import { App } from "./App";
import { loader } from "@monaco-editor/react";
import "./index.css";

// TODO: It is incomplete
loader.init().then((monaco) => {
  monaco.languages.register({ id: "jakt" });

  monaco.languages.setMonarchTokensProvider("jakt", {
    tokenizer: {
      root: [
        [
          /[A-Za-z_$][\w$]*/,
          {
            cases: {
              "@operators": "operator",
              "@typeKeywords": "type",
              "@keywords": "keyword",
              "@default": "identifier",
            },
          },
        ],
      ],
    },
    operators: ["not", "and", "or"],
    typeKeywords: [
      "String",
      "i8",
      "u8",
      "i16",
      "u16",
      "i32",
      "u32",
      "i64",
      "u64",
      "f32",
      "f64",
      "bool",
      "usize",
      "c_char",
      "c_int",
    ],
    keywords: [
      "true",
      "false",
      "class",
      "struct",
      "if",
      "else",
      "while",
      "for",
      "loop",
      "return",
      "break",
      "continue",
      "function",
      "extern",
      "throws",
      "defer",
      "unsafe",
      "throw",
      "try",
      "catch",
      "cpp",
      "mutable",
      "let",
      "anonymous",
      "raw",
    ],
  });
});

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
