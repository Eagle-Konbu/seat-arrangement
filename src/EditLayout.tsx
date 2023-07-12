import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useSearchParams } from "react-router-dom";
import { Card, CardContent, Drawer, FormControl, Grid, InputLabel, TextField, Typography } from "@mui/material";
import SeatCard from "./components/SeatCard";

function EditLayout() {

  const [searchParams, _] = useSearchParams();

  const width = Number(searchParams.get("width"));
  const depth = Number(searchParams.get("depth"));

  const [drawerIsOpen, setDrawerIsOpen] = useState(true);

  const sampleStudent = { id: 1, name: "田中 太郎", academic_ability: 3, exercise_ability: 3, leadership_ability: 3, needs_assistance: false, gender: "Male" };

  function toggleDrawer() {
    setDrawerIsOpen(!drawerIsOpen);
  }

  return (
    <div>
      <Grid container spacing={2} columns={width}>
        {(() => {
          const elements = [];
          for (let i = 0; i < width * depth; i++) {
            elements.push(
              <SeatCard student={sampleStudent} />
            );
          }
          return elements;
        })()}
      </Grid>

      <Drawer
        anchor="right"
        open={drawerIsOpen}
      >
        <FormControl fullWidth>
          <InputLabel id="student_name">名前</InputLabel>
          <TextField id="student_name" label="名前" variant="standard" />
        </FormControl>
      </Drawer>
    </div>
  );
}

export default EditLayout;