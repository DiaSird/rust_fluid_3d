import { useState } from "react";
import reactLogo from "./assets/react.svg";
import viteLogo from "/vite.svg";
import "./App.css";
import ThreeView from "./ThreeView";

function App() {
  const [count, setCount] = useState(0);

  return (
    <div style={{ display: "flex", flexDirection: "column", height: "100vh" }}>
      {/* Header section */}
      <div
        style={{
          height: "120px",
          display: "flex",
          alignItems: "center",
          padding: "0 20px",
          background: "#222",
          color: "#fff",
        }}
      >
        <div style={{ display: "flex", alignItems: "center", gap: "20px" }}>
          {/* Vite logo */}
          <a href="https://vite.dev" target="_blank" rel="noreferrer">
            <img src={viteLogo} className="logo" alt="Vite logo" />
          </a>
          {/* React logo */}
          <a href="https://react.dev" target="_blank" rel="noreferrer">
            <img src={reactLogo} className="logo react" alt="React logo" />
          </a>
          {/* App title */}
          <h1 style={{ margin: 0 }}>Vite + React + Three.js</h1>
        </div>
        {/* Counter button on the right */}
        <div style={{ marginLeft: "auto" }}>
          <button
            onClick={() => setCount((c) => c + 1)}
            style={{ padding: "5px 10px" }}
          >
            Count: {count}
          </button>
        </div>
      </div>

      {/* Three.js rendering area */}
      <div style={{ flex: 1 }}>
        <ThreeView />
      </div>
    </div>
  );
}

export default App;
