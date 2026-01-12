import type { ParameterState } from "./types";

// Initial state for simulation parameters
export const INITIAL_PARAMETER_STATE = {
  max_n: 60000,
  max_near_n: 100,

  model_scale: {
    length: 0.5,
    width: 0.5,
    height: 0.5,
  },

  bc_pattern: "Cavity-Flow",
  u_lid: 5.0,

  smooth_length: 0.0324,
  cell_scale: 2.0,
  beta: 0.3,
  cs_rate: 0.05,

  dx: { dx: 0.027, dy: 0.027, dz: 0.027 },

  dt: 0.001,
  out_step: 10,
  max_step: 1000,

  restart_file: "sim_checkpoint.bin",
  monitor_particle: 10,

  log: [""],
  isRunning: false,
  guiState: undefined,
} as const satisfies ParameterState;
