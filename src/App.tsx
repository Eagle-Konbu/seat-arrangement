import { BrowserRouter, Route, Routes } from "react-router-dom";
import "./App.css";

import Result from "./Result";
import EditLayout from "./EditLayout";

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Result />} />
        <Route path="/edit_layout" element={<EditLayout />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
