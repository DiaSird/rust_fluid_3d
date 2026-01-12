import type { GuiState } from "../../../api/simulation";

export type Vector3 = [number, number, number];

export type Matrix3 = [
  [number, number, number],
  [number, number, number],
  [number, number, number]
];

export interface ModelScale {
  length: number;
  width: number;
  height: number;
}

export interface Resolution {
  dx: number;
  dy: number;
  dz: number;
}

/** Boundary Condition **/
export const BC_OPTIONS = [
  "Cavity-Flow",
  "Poiseuille-Flow",
  "Periodic-Flow",
  "LidDrivenCavity",
] as const;

export type BC = (typeof BC_OPTIONS)[number];

export interface Config {
  // Max particles
  max_n: number;
  max_near_n: number;

  // Model size
  model_scale: ModelScale;

  // Boundary condition
  bc_pattern: BC;
  u_lid: number;

  // SPH parameters
  smooth_length: number;
  cell_scale: number;
  beta: number;
  cs_rate: number;

  // Resolution
  dx: Resolution;

  // Time stepping
  dt: number;
  out_step: number;
  max_step: number;

  // Checkpoint / Monitoring
  restart_file?: string; // Option<PathBuf>
  monitor_particle: number;
}

export interface ParameterState extends Config {
  log: string[];
  isRunning: boolean;
  guiState?: GuiState;
}

export type ParameterAction =
  // ========= Config one update =========
  | { type: "SET_MAX_N"; value: number }
  | { type: "SET_MAX_NEAR_N"; value: number }
  | { type: "SET_MODEL_SCALE"; value: ModelScale }
  | { type: "SET_BC_PATTERN"; value: BC }
  | { type: "SET_U_LID"; value: number }
  | {
      type: "SET_SPH_PARAMS";
      value: Partial<
        Pick<Config, "smooth_length" | "cell_scale" | "beta" | "cs_rate">
      >;
    }
  | { type: "SET_DX"; value: Resolution }
  | {
      type: "SET_TIME_STEPPING";
      value: Partial<Pick<Config, "dt" | "out_step" | "max_step">>;
    }
  | { type: "SET_MONITOR_PARTICLE"; value: number }
  | { type: "SET_RESTART_FILE"; value: string }

  // ========= Meta =========
  | { type: "LOAD_CONFIG"; value: Config }
  | { type: "RESET_TO_DEFAULT" }

  // ========= Runtime / UI =========
  | { type: "APPEND_LOG"; value: string }
  | { type: "CLEAR_LOG" }
  | { type: "SET_IS_RUNNING"; value: boolean }
  | { type: "SET_GUI_STATE"; value: GuiState };
