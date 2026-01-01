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

  dt: number;
  out_step: number;
  max_step: number;
}

export const runSimulation = async (config: Config) => {
  try {
    // Run tauri command
    await invoke("run_simulation", { config });
    alert("Simulation started.");
  } catch (err) {
    console.error(err);
    alert(`Simulation failed: ${err}`);
  }
};
