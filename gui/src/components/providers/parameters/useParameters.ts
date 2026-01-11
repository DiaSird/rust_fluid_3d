import { useContext } from "react";
import { ParameterContext } from "./ParameterContext";
import type { ParameterContextProps } from "./types";

export const useParameters = (): ParameterContextProps => {
  const context = useContext(ParameterContext);
  if (!context) {
    throw new Error("useParameters must be used within a ParameterProvider");
  }
  return context;
};
