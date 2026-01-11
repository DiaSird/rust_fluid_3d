import { emit } from "@tauri-apps/api/event";
import { useParameters } from "./providers/parameters/ParameterContext";
import { useCallback, useState } from "react";
import { runSimulation } from "../api/simulation";

export const RunStopButton: React.FC = () => {
  const config = useParameters();
  const [isRunning, setIsRunning] = useState(false);

  const handleRun = useCallback(async () => {
    setIsRunning(true);
    try {
      await runSimulation(config);
    } finally {
      setIsRunning(false);
    }
  }, [config]);

  const handleStop = async () => {
    await emit("terra://simulation-stop-event");
    setIsRunning(false);
  };

  return !isRunning ? (
    <button style={runStyle} onClick={handleRun}>
      Run
    </button>
  ) : (
    <button style={stopStyle} onClick={handleStop}>
      Stop
    </button>
  );
};

const runStyle: React.CSSProperties = {
  width: "100%",
  padding: "8px",
  background: "#0a0",
  color: "#fff",
  border: "none",
  marginBottom: "10px",
};

const stopStyle: React.CSSProperties = {
  ...runStyle,
  background: "#aa0025",
};
