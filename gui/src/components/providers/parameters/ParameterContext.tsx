import { createContext, useContext } from "react";

import type { ParameterContextProps } from "./types";

export const ParameterContext = createContext<ParameterContextProps | undefined>(undefined);

export const useParameters = (): ParameterContextProps => {
  const context = useContext(ParameterContext);
  if (!context) throw new Error("useParameters must be used within a ParameterProvider");
  return context;
};
