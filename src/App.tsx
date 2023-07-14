import { BrowserRouter, Route, Routes } from "react-router-dom";

import EditLayout from "./EditLayout";
import SizeConfig from "./SizeConfig";

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<SizeConfig />} />
        <Route path="/edit_layout" element={<EditLayout />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
