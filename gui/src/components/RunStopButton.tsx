import { emit } from "@tauri-apps/api/event";
import { useParameters } from "./providers/parameters/useParameters";
import { useCallback } from "react";
import { runSimulation } from "../api/simulation";

export const RunStopButton: React.FC = () => {
  const { state, dispatch } = useParameters();

  const handleRun = useCallback(async () => {
    dispatch({
      type: "SET_IS_RUNNING",
      value: true,
    });

    try {
      await runSimulation(state);
    } finally {
      dispatch({
        type: "SET_IS_RUNNING",
        value: false,
      });
    }
  }, [dispatch, state]);

  const handleStop = async () => {
    await emit("terra://simulation-stop-event");
    dispatch({
      type: "SET_IS_RUNNING",
      value: false,
    });
  };

  return !state.isRunning ? (
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
