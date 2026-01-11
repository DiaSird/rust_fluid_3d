import { invoke } from "@tauri-apps/api/core";
import { type Config } from "./components/ParameterContext";

export const runSimulation = async (config: Config) => {
  try {
    // Run tauri command
    await invoke("run_simulation", { config });
    // alert("Simulation started.");
  } catch (err) {
    console.error(err);
    // alert(`Simulation failed: ${err}`);
  }
};
