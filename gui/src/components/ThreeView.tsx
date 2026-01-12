import { useState } from "react";
import { ParameterPanel } from "./ParameterPanel";
import { ThreeCanvas } from "./ThreeCanvas";
import { LogPanel } from "./LogPanel";

export default function ThreeView() {
  const [activeTab, setActiveTab] = useState<"params" | "log">("params");

  return (
    <main style={rootGrid}>
      {/* Tabs */}
      <div style={tabBar}>
        <TabButton active={activeTab === "params"} onClick={() => setActiveTab("params")}>
          Parameters
        </TabButton>
        <TabButton active={activeTab === "log"} onClick={() => setActiveTab("log")}>
          Log
        </TabButton>
      </div>

      {/* Left panel */}
      {activeTab === "params" && (
        <div style={leftPane}>
          <ParameterPanel />
        </div>
      )}

      {/* Right panel */}
      {activeTab === "params" && (
        <div style={rightPane}>
          <ThreeCanvas />
        </div>
      )}

      {/* Log panel */}
      {activeTab === "log" && (
        <div style={logPane}>
          <LogPanel />
        </div>
      )}
    </main>
  );
}

const TabButton: React.FC<{
  active: boolean;
  onClick: () => void;
  children: React.ReactNode;
}> = ({ active, onClick, children }) => (
  <button
    onClick={onClick}
    style={{
      padding: "8px 16px",
      background: active ? "#333" : "transparent",
      color: "#fff",
      border: "none",
      cursor: "pointer",
    }}
  >
    {children}
  </button>
);

const rootGrid: React.CSSProperties = {
  display: "grid",
  gridTemplateRows: "auto 1fr",
  gridTemplateColumns: "30% 1fr",
  background: "#111",
  height: "100vh",
  paddingBottom: 50,
};

const tabBar: React.CSSProperties = {
  gridColumn: "1 / -1",
  display: "flex",
  borderBottom: "1px solid #333",
};

const leftPane: React.CSSProperties = {
  gridRow: 2,
  gridColumn: 1,
  overflowY: "auto",
  overflow: "scroll",
  borderRight: "1px solid #333",
};

const rightPane: React.CSSProperties = {
  gridRow: 2,
  gridColumn: 2,
  position: "relative",
};

const logPane: React.CSSProperties = {
  gridRow: 2,
  gridColumn: "1 / -1",
  overflow: "auto",
};
