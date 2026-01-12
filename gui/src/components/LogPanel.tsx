import { useParameters } from "./providers/parameters/useParameters";

export const LogPanel: React.FC = () => {
  const { state } = useParameters();
  const { log } = state;

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
      {log.map((log, i) => (
        <div key={i} style={{ whiteSpace: "pre-wrap" }}>
          {log}
        </div>
      ))}
    </div>
  );
};
