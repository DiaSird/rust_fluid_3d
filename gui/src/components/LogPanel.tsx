import { useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { useParameters } from "./ParameterContext";

export const LogPanel: React.FC = () => {
  const { log, setLog } = useParameters();

  useEffect(() => {
    const unlistenPromise = listen<string>("terra://simulation-log", (event) => {
      // console.log(JSON.stringify(event.payload));
      setLog((prev) => [...prev, JSON.stringify(event.payload)]);
    });

    // return () => {
    //   unlistenPromise.then((unlisten) => unlisten());
    // };
  }, [setLog]);

  return (
    <div
      style={{
        background: "#111",
        color: "#0f0",
        padding: "8px",
        height: "100%",
        overflowY: "auto",
        fontFamily: "monospace",
        fontSize: "12px",
      }}
    >
      {log.split("\n").map((log, i) => (
        <div key={i} style={{ whiteSpace: "pre-wrap" }}>
          {log}
        </div>
      ))}
    </div>
  );
};
