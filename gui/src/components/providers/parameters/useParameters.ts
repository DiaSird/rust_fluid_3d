import { useContext } from "react";
import { ParameterContext } from "./ParameterContext";
import type { ParameterContextType } from "./ParameterContext";

/**
 * useParameters
 *
 * Custom hook to access and modify simulation parameters managed
 * by {@link ParameterProvider}.
 *
 * This hook provides:
 * - `state`: the current parameter state (Config + UI/runtime fields)
 * - `dispatch`: a reducer dispatch function to update parameters
 *
 * ⚠️ Must be used inside `<ParameterProvider>`.
 *
 * @example
 * ```tsx
 * const { state, dispatch } = useParameters();
 *
 * // -----------------------------
 * // Read values
 * // -----------------------------
 *
 * console.log(state.max_n);
 * console.log(state.model_scale.length);
 * console.log(state.dx.dx);
 *
 * // -----------------------------
 * // Update single parameters
 * // -----------------------------
 *
 * dispatch({
 *   type: "SET_MAX_N",
 *   value: 80000,
 * });
 *
 * dispatch({
 *   type: "SET_U_LID",
 *   value: 7.5,
 * });
 *
 * // -----------------------------
 * // Update grouped parameters
 * // -----------------------------
 *
 * dispatch({
 *   type: "SET_SPH_PARAMS",
 *   value: {
 *     smooth_length: 0.03,
 *     beta: 0.25,
 *   },
 * });
 *
 * dispatch({
 *   type: "SET_TIME_STEPPING",
 *   value: {
 *     dt: 0.0005,
 *     out_step: 20,
 *     max_step: 2000,
 *   },
 * });
 *
 * // -----------------------------
 * // Update structured values
 * // -----------------------------
 *
 * dispatch({
 *   type: "SET_MODEL_SCALE",
 *   value: {
 *     length: 1.0,
 *     width: 0.5,
 *     height: 0.5,
 *   },
 * });
 *
 * dispatch({
 *   type: "SET_DX",
 *   value: {
 *     dx: 0.02,
 *     dy: 0.02,
 *     dz: 0.02,
 *   },
 * });
 *
 * // -----------------------------
 * // Change boundary condition
 * // -----------------------------
 *
 * dispatch({
 *   type: "SET_BC_PATTERN",
 *   value: "Cavity-Flow",
 * });
 *
 * // -----------------------------
 * // Load / reset config
 * // -----------------------------
 *
 * dispatch({
 *   type: "LOAD_CONFIG",
 *   value: loadedConfig,
 * });
 *
 * dispatch({
 *   type: "RESET_TO_DEFAULT",
 * });
 *
 * // -----------------------------
 * // Runtime / UI actions
 * // -----------------------------
 *
 * dispatch({
 *   type: "APPEND_LOG",
 *   value: "Simulation started",
 * });
 *
 * dispatch({
 *   type: "CLEAR_LOG",
 * });
 * ```
 */
export const useParameters = (): ParameterContextType => {
  const context = useContext(ParameterContext);
  if (!context) {
    throw new Error("useParameters must be used within a ParameterProvider");
  }
  return context;
};
