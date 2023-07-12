import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useSearchParams } from "react-router-dom";
import { Card, CardContent, Checkbox, Drawer, FormControl, Grid, Input, InputLabel, TextField, Typography } from "@mui/material";
import SeatCard from "./components/SeatCard";
import { Slider } from "@mui/base";

function EditLayout() {

  const [searchParams, _] = useSearchParams();

  const width = Number(searchParams.get("width"));
  const depth = Number(searchParams.get("depth"));

  const [drawerIsOpen, setDrawerIsOpen] = useState(false);

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
              <SeatCard student={sampleStudent} onClick={toggleDrawer} />
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
          <InputLabel htmlFor="student_name">名前</InputLabel>
          <Input id="student_name" />
        </FormControl>

        <FormControl fullWidth>
          <InputLabel htmlFor="student_academic_ability">学力</InputLabel>
          <Slider
            defaultValue={3}
            step={1}
            max={5}
            min={1}
            marks
          />
        </FormControl>

        <FormControl fullWidth>
          <InputLabel htmlFor="student_exercise_ability">運動能力</InputLabel>
          <Slider
            defaultValue={3}
            step={1}
            max={5}
            min={1}
            marks
          />
        </FormControl>

        <FormControl fullWidth>
          <InputLabel htmlFor="student_leadership_ability">リーダーシップ</InputLabel>
          <Slider
            defaultValue={3}
            step={1}
            max={5}
            min={1}
            marks
          />
        </FormControl>

        <FormControl fullWidth>
          <InputLabel htmlFor="student_needs_assistance">支援が必要</InputLabel>
          <Checkbox id="student_needs_assistance" />
        </FormControl>
      </Drawer>
    </div>
  );
}

export default EditLayout;