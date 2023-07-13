import { BrowserRouter, Route, Routes } from "react-router-dom";

import Result from "./Result";
import EditLayout from "./EditLayout";
import SizeConfig from "./SizeConfig";

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<SizeConfig />} />
        <Route path="/result" element={<Result />} />
        <Route path="/edit_layout" element={<EditLayout />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
