import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useSearchParams } from "react-router-dom";
import { Box, Drawer, Grid, Stack, TextField, Slider, Divider, Typography, InputLabel, Select, MenuItem, Checkbox, Button } from "@mui/material";
import SeatCard from "./components/SeatCard";

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
        <Box padding={2}>
          <TextField label="出席番号" type="number" defaultValue={sampleStudent.id} margin="normal" />
          <Divider />

          <TextField label="名前" defaultValue={sampleStudent.name} margin="normal" />
          <Divider />

          <InputLabel id="gender-select">性別</InputLabel>
          <Select
            labelId="gender-select"
          >
            <MenuItem value="Male">男</MenuItem>
            <MenuItem value="Female">女</MenuItem>
          </Select>
          <Divider />

          <Typography gutterBottom>学力</Typography>
          <Slider
            defaultValue={3}
            step={1}
            max={5}
            min={1}
            marks
            valueLabelDisplay="auto"
          />
          <Divider />

          <Typography gutterBottom>運動能力</Typography>
          <Slider
            defaultValue={3}
            step={1}
            max={5}
            min={1}
            marks
            valueLabelDisplay="auto"
          />
          <Divider />

          <Typography gutterBottom>リーダーシップ</Typography>
          <Slider
            defaultValue={3}
            step={1}
            max={5}
            min={1}
            marks
            valueLabelDisplay="auto"
          />
          <Divider />

          <Typography gutterBottom>支援が必要</Typography>
          <Checkbox />
          <Divider />

          <Stack direction="row" spacing={2}>
            <Button variant="outlined" onClick={toggleDrawer}>キャンセル</Button>
            <Button variant="contained" onClick={toggleDrawer}>保存</Button>
          </Stack>
        </Box>
      </Drawer>
    </div>
  );
}

export default EditLayout;