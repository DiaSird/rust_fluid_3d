import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";

export const LogPanel: React.FC = () => {
  const [logs, setLogs] = useState<string[]>([]);

  useEffect(() => {
    const unlistenPromise = listen<string>("simulation-log", (event) => {
      setLogs((prev) => [...prev, event.payload]);
    });

    return () => {
      unlistenPromise.then((unlisten) => unlisten());
    };
  }, []);

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
      {logs.map((log, i) => (
        <div key={i} style={{ whiteSpace: "pre-wrap" }}>
          {log}
        </div>
      ))}
    </div>
  );
};
