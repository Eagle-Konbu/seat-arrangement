import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useSearchParams } from "react-router-dom";
import { Box, Drawer, Grid, Stack, TextField, Slider, Divider, Typography, InputLabel, Select, MenuItem, Checkbox, Button, IconButton, Tooltip, Rating, Backdrop } from "@mui/material";
import SeatCard from "./components/SeatCard";

import RotateLeftIcon from '@mui/icons-material/RotateLeft';
import { Dna } from "react-loader-spinner";

function EditLayout() {
  const sampleStudent = { id: 1, name: "田中 太郎", academic_ability: 3, exercise_ability: 3, leadership_ability: 3, needs_assistance: false, gender: "Male" };

  const [searchParams, _] = useSearchParams();

  const width = Number(searchParams.get("width"));
  const depth = Number(searchParams.get("depth"));

  const [drawerIsOpen, setDrawerIsOpen] = useState(false);
  const [backdropIsOpen, setBackdropIsOpen] = useState(false);

  const [editedPosition, setEditedPosition] = useState([-1, -1]);

  const [idValue, setIdValue] = useState(0);
  const [nameValue, setNameValue] = useState("");
  const [genderValue, setGenderValue] = useState("Male");
  const [academicAbilityValue, setAcademicAbilityValue] = useState(3);
  const [exerciseAbilityValue, setExerciseAbilityValue] = useState(3);
  const [leadershipAbilityValue, setLeadershipAbilityValue] = useState(3);
  const [needsAssistanceValue, setNeedsAssistanceValue] = useState(false);

  const [nameInputIsError, setNameInputIsError] = useState(false);

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

  function resetStudent(row: number, col: number) {
    const newSeats = [...seats];
    newSeats[row][col] = null;
    setSeats(newSeats);
  }

  function solve() {
    setBackdropIsOpen(true);
    invoke("solve", { currentSeatAssignment: seats })
      .then((result) => {
        console.log(result);
      })
      .catch((err) => {
        window.alert(err);
      })
      .finally(() => {
        setBackdropIsOpen(false);
      });
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
        <Button fullWidth variant="contained" onClick={solve}>席替え実行</Button>
      </Stack>

      <Drawer
        anchor="right"
        open={drawerIsOpen}
        onClose={toggleDrawer}
      >
        <Box padding={2}>
          <Stack direction="row">
            <Tooltip
              title="リセット"
              arrow
            >
              <IconButton
                onClick={async () => {
                  if (await window.confirm("入力情報をリセットします。よろしいですか？")) {
                    resetStudent(editedPosition[1], editedPosition[0]);
                    toggleDrawer();
                  }
                }}
                sx={{ boxShadow: 0 }}
              >
                <RotateLeftIcon />
              </IconButton>
            </Tooltip>
          </Stack>


          <TextField
            label="出席番号"
            type="number"
            margin="normal"
            sx={{ boxShadow: 0 }}
            value={idValue}
            onChange={(e) => setIdValue(Number(e.target.value))}
          />
          <Divider />

          <TextField
            label="名前"
            margin="normal"
            sx={{ boxShadow: 0 }}
            value={nameValue}
            onChange={(e) => {
              setNameValue(e.target.value);
              setNameInputIsError(false);
            }}
            error={nameInputIsError}
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
          <Rating
            value={academicAbilityValue}
            onChange={(_, v) => setAcademicAbilityValue(v || academicAbilityValue)}
          />
          <Divider />

          <Typography gutterBottom>運動能力</Typography>
          <Rating
            value={exerciseAbilityValue}
            onChange={(_, v) => setExerciseAbilityValue(v || exerciseAbilityValue)}
          />
          <Divider />

          <Typography gutterBottom>リーダーシップ</Typography>
          <Rating
            value={leadershipAbilityValue}
            onChange={(_, v) => setLeadershipAbilityValue(v || leadershipAbilityValue)}
          />
          <Divider />

          <Typography gutterBottom>要支援</Typography>
          <Checkbox
            checked={needsAssistanceValue}
            onChange={(e) => setNeedsAssistanceValue(e.target.checked)}
          />
          <Divider />

          <Stack direction="row" spacing={2}>
            <Button variant="outlined" onClick={toggleDrawer} sx={{ boxShadow: 0 }}>キャンセル</Button>
            <Button
              variant="contained"
              onClick={() => {
                if (nameValue === "") {
                  setNameInputIsError(true);
                  return;
                }
                setStudent(editedPosition[1], editedPosition[0], idValue, nameValue, academicAbilityValue, exerciseAbilityValue, leadershipAbilityValue, needsAssistanceValue, genderValue);
                toggleDrawer();
              }}
            >
              保存
            </Button>
          </Stack>
        </Box>
      </Drawer>

      <Backdrop
        sx={{ color: "#fff", zIndex: (theme) => theme.zIndex.drawer + 1 }}
        open={backdropIsOpen}
      >
        <Dna
          height={80}
          width={80}
        />
      </Backdrop>
    </Box>
  );
}

export default EditLayout;