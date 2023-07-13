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
  // const [editedStudent, setEditedStudent] = useState<(Student | null)>(null);

  const [idValue, setIdValue] = useState(0);
  const [nameValue, setNameValue] = useState("");
  const [genderValue, setGenderValue] = useState("Male");
  const [academicAbilityValue, setAcademicAbilityValue] = useState(3);
  const [exerciseAbilityValue, setExerciseAbilityValue] = useState(3);
  const [leadershipAbilityValue, setLeadershipAbilityValue] = useState(3);
  const [needsAssistanceValue, setNeedsAssistanceValue] = useState(false);

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
                <Grid item xs={1}>
                  <SeatCard
                    student={seats[y][x]}
                    onClick={() => {
                      setEditedPosition([x, y]);
                      if (seats[y][x] !== null) {
                        setIdValue(seats[y][x]!.id);
                        setNameValue(seats[y][x]!.name);
                        setGenderValue(seats[y][x]!.gender);
                        setAcademicAbilityValue(seats[y][x]!.academic_ability);
                        setExerciseAbilityValue(seats[y][x]!.exercise_ability);
                        setLeadershipAbilityValue(seats[y][x]!.leadership_ability);
                        setNeedsAssistanceValue(seats[y][x]!.needs_assistance);
                      } else {
                        setIdValue(0);
                        setNameValue("");
                        setGenderValue("Male");
                        setAcademicAbilityValue(3);
                        setExerciseAbilityValue(3);
                        setLeadershipAbilityValue(3);
                        setNeedsAssistanceValue(false);
                      }

                      toggleDrawer();
                    }}
                  />
                </Grid>
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
        onClose={toggleDrawer}
      >
        <Box padding={2}>
          <TextField
            label="出席番号"
            type="number"
            margin="normal"
            value={idValue}
            onChange={(e) => setIdValue(Number(e.target.value))}
          />
          <Divider />

          <TextField
            label="名前"
            margin="normal"
            value={nameValue}
            onChange={(e) => setNameValue(e.target.value)}
          />
          <Divider />

          <InputLabel id="gender-select">性別</InputLabel>
          <Select
            labelId="gender-select"
            value={genderValue}
            onChange={(e) => setGenderValue(e.target.value)}
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
            value={academicAbilityValue}
            onChange={(_, v) => setAcademicAbilityValue(v as number)}
          />
          <Divider />

          <Typography gutterBottom>運動能力</Typography>
          <Slider
            step={1}
            max={5}
            min={1}
            marks
            valueLabelDisplay="auto"
            value={exerciseAbilityValue}
            onChange={(_, v) => setExerciseAbilityValue(v as number)}
          />
          <Divider />

          <Typography gutterBottom>リーダーシップ</Typography>
          <Slider
            step={1}
            max={5}
            min={1}
            marks
            valueLabelDisplay="auto"
            value={leadershipAbilityValue}
            onChange={(_, v) => setLeadershipAbilityValue(v as number)}
          />
          <Divider />

          <Typography gutterBottom>要支援</Typography>
          <Checkbox
            checked={needsAssistanceValue}
            onChange={(e) => setNeedsAssistanceValue(e.target.checked)}
          />
          <Divider />

          <Stack direction="row" spacing={2}>
            <Button variant="outlined" onClick={toggleDrawer}>キャンセル</Button>
            <Button
              variant="contained"
              onClick={() => {
                setStudent(editedPosition[1], editedPosition[0], idValue, nameValue, academicAbilityValue, exerciseAbilityValue, leadershipAbilityValue, needsAssistanceValue, genderValue);
                toggleDrawer();
              }}
            >
              保存
            </Button>
          </Stack>
        </Box>
      </Drawer>
    </Box>
  );
}

export default EditLayout;