import { invoke } from "@tauri-apps/api/core";

export interface Config {
  length: number;
  width: number;
  height: number;
  n_axis: number;
  smooth_length: number;
  cell_size: number;
  beta: number;
  cs_rate: number;
  dx: number;
  dy: number;
  dz: number;
}

export const runSimulation = async (config: Config) => {
  try {
    await invoke("set_model_config", { config });
    alert("Simulation started.");
  } catch (err) {
    console.error(err);
    alert(`Simulation failed: ${err}`);
  }
};
