import { createContext } from "react";
import type { ParameterState, ParameterAction } from "./types";

export type ParameterContextType = {
  state: ParameterState;
  dispatch: React.Dispatch<ParameterAction>;
};

export const ParameterContext = createContext<ParameterContextType | null>(null);
