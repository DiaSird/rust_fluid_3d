import { useParameters } from "./ParameterContext";

export const LogPanel: React.FC = () => {
  const { log } = useParameters();

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
