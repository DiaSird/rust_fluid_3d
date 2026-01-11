import { useReducer, type ReactNode } from "react";
import { ParameterContext } from "./ParameterContext";
import { parameterReducer } from "./parameterReducer";
import { INITIAL_PARAMETER_STATE } from "./initialParameterState";

export const ParameterProvider = ({ children }: { children: ReactNode }) => {
  const [state, dispatch] = useReducer(parameterReducer, INITIAL_PARAMETER_STATE);

  return <ParameterContext.Provider value={{ state, dispatch }}>{children}</ParameterContext.Provider>;
};
