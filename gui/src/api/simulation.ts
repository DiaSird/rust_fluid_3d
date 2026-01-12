import { invoke } from "@tauri-apps/api/core";
import { type Config } from "../components/providers/parameters/types";

export const runSimulation = async (config: Config) => {
  try {
    await invoke("run_simulation", { config });
  } catch (err) {
    console.error(err);
  }
};
