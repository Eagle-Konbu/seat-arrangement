import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useSearchParams } from "react-router-dom";
import { Box, Drawer, Grid, Stack, TextField, Slider, Divider, Typography, InputLabel, Select, MenuItem, Checkbox, Button } from "@mui/material";
import SeatCard from "./components/SeatCard";

function EditLayout() {
  const sampleStudent = { id: 1, name: "田中 太郎", academic_ability: 3, exercise_ability: 3, leadership_ability: 3, needs_assistance: false, gender: "Male" };

  const [searchParams, _] = useSearchParams();

  const width = Number(searchParams.get("width"));
  const depth = Number(searchParams.get("depth"));

  const [drawerIsOpen, setDrawerIsOpen] = useState(false);

  const [editedPosition, setEditedPosition] = useState([-1, -1]);
  const [editedStudent, setEditedStudent] = useState(sampleStudent);

  const [seats, setSeats] = useState<(Student | null)[][]>(() => {
    const seats = [];
    for (let i = 0; i < depth; i++) {
      const row = [];
      for (let j = 0; j < width; j++) {
        row.push(null);
      }
      seats.push(row);
    }
    return seats;
  });

  function toggleDrawer() {
    setDrawerIsOpen(!drawerIsOpen);
  }

  function setStudent(row: number, col: number, id: number, name: string, academic_ability: number, exercise_ability: number, leadership_ability: number, needs_assistance: boolean, gender: string) {
    const newSeats = [...seats];
    newSeats[row][col] = { id, name, academic_ability, exercise_ability, leadership_ability, needs_assistance, gender };
    setSeats(newSeats);
  }

  return (
    <Box padding={1}>
      <Stack spacing={2}>
        <Grid container spacing={2} columns={width}>
          {(() => {
            const elements = [];
            for (let i = 0; i < width * depth; i++) {
              let x = i % width;
              let y = Math.floor(i / width);
              elements.push(
                <SeatCard
                  student={sampleStudent}
                  onClick={() => {
                    setEditedPosition([x, y]);
                    toggleDrawer();
                  }}
                />
              );
            }
            return elements;
          })()}
        </Grid>
        <Button fullWidth variant="contained">席替え実行</Button>
      </Stack>

      <Drawer
        anchor="right"
        open={drawerIsOpen}
      >
        <Box padding={2}>
          <TextField label="出席番号" type="number" margin="normal" />
          <Divider />

          <TextField label="名前" margin="normal" />
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
            step={1}
            max={5}
            min={1}
            marks
            valueLabelDisplay="auto"
          />
          <Divider />

          <Typography gutterBottom>運動能力</Typography>
          <Slider
            step={1}
            max={5}
            min={1}
            marks
            valueLabelDisplay="auto"
          />
          <Divider />

          <Typography gutterBottom>リーダーシップ</Typography>
          <Slider
            step={1}
            max={5}
            min={1}
            marks
            valueLabelDisplay="auto"
          />
          <Divider />

          <Typography gutterBottom>支援が必要</Typography>
          <Checkbox
          />
          <Divider />

          <Stack direction="row" spacing={2}>
            <Button variant="outlined" onClick={toggleDrawer}>キャンセル</Button>
            <Button variant="contained" onClick={toggleDrawer}>保存</Button>
          </Stack>
        </Box>
      </Drawer>
    </Box>
  );
}

export default EditLayout;