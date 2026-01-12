import { INITIAL_PARAMETER_STATE } from "./initialParameterState";
import type { ParameterState, ParameterAction } from "./types";

export const parameterReducer = (
  state: ParameterState,
  action: ParameterAction
): ParameterState => {
  switch (action.type) {
    // ========= Config one update =========
    case "SET_MAX_N":
      return { ...state, max_n: action.value };

    case "SET_MAX_NEAR_N":
      return { ...state, max_near_n: action.value };

    case "SET_MODEL_SCALE":
      return { ...state, model_scale: action.value };

    case "SET_BC_PATTERN":
      return {
        ...state,
        bc_pattern: action.value,
        // reset dependent parameter if BC changes
        u_lid: action.value === "Cavity-Flow" ? state.u_lid : 0,
      };

    case "SET_U_LID":
      return { ...state, u_lid: action.value };

    case "SET_SPH_PARAMS":
      return {
        ...state,
        ...action.value,
      };

    case "SET_DX":
      return { ...state, dx: action.value };

    case "SET_TIME_STEPPING":
      return {
        ...state,
        ...action.value,
      };

    case "SET_MONITOR_PARTICLE":
      return { ...state, monitor_particle: action.value };

    case "SET_RESTART_FILE":
      return { ...state, restart_file: action.value };

    // ========= Meta =========
    case "LOAD_CONFIG":
      return {
        ...state,
        ...action.value,
        // preserve runtime-only fields
        restart_file: action.value.restart_file ?? state.restart_file,
        log: state.log,
      };

    case "RESET_TO_DEFAULT":
      return INITIAL_PARAMETER_STATE;

    // ========= Runtime / UI =========
    case "APPEND_LOG":
      return { ...state, log: [...state.log, action.value] };

    case "CLEAR_LOG":
      return { ...state, log: [] };

    case "SET_IS_RUNNING":
      return { ...state, isRunning: action.value };

    case "SET_GUI_STATE":
      return { ...state, guiState: action.value };

    default: {
      // Exhaustiveness check (compile-time safety)
      return state;
    }
  }
};
