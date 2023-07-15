import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { Box, Drawer, Grid, Stack, TextField, Divider, Typography, InputLabel, Select, MenuItem, Checkbox, Button, IconButton, Tooltip, Rating, Backdrop } from "@mui/material";
import SeatCard from "./components/SeatCard";
import ResultDialog from "./components/ResultDialog";

import RotateLeftIcon from '@mui/icons-material/RotateLeft';
import { Dna } from "react-loader-spinner";

import type { Student } from "./types/Student";
import type { ExecutionResult } from "./types/ExecutionResult";
import SizeConfigDialog from "./components/SizeConfigDialog";

function EditLayout() {
  const defaultStudent: Student = {
    id: 0,
    name: "",
    academic_ability: 3,
    exercise_ability: 3,
    leadership_ability: 3,
    needs_assistance: false,
    gender: "Male"
  };

  const [width, setWidth] = useState(5);
  const [depth, setDepth] = useState(5);

  const [drawerIsOpen, setDrawerIsOpen] = useState(false);
  const [backdropIsOpen, setBackdropIsOpen] = useState(false);

  const [editedPosition, setEditedPosition] = useState([-1, -1]);

  const [editedStudent, setEditedStudent] = useState(defaultStudent);

  const [nameInputIsError, setNameInputIsError] = useState(false);
  const [idInputIsError, setIdInputIsError] = useState(false);

  const [nameInputHelperText, setNameInputHelperText] = useState("");
  const [idInputHelperText, setIdInputHelperText] = useState("");

  const [sizeConfigIsOpen, setSizeConfigIsOpen] = useState(false);
  const [resultIsOpen, setResultIsOpen] = useState(false);

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

  const [result, setResults] = useState<(Student | null)[][]>(() => {
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

  async function ChangeSize(newWidth: number, newDepth: number) {
    if (await window.confirm("既に入力された情報はリセットされます。よろしいですか？")) {
      setWidth(newWidth);
      setDepth(newDepth);

      const seats = [];
      for (let i = 0; i < newDepth; i++) {
        const row = [];
        for (let j = 0; j < newWidth; j++) {
          row.push(null);
        }
        seats.push(row);
      }
      setSeats(seats);
      setSizeConfigIsOpen(false);
    }
  }

  useEffect(() => {
    listen("change_size", (_) => {
      setSizeConfigIsOpen(true);
    })
  });

  function toggleDrawer() {
    setDrawerIsOpen(!drawerIsOpen);
    setNameInputHelperText("");
    setIdInputHelperText("");
    setNameInputIsError(false);
    setIdInputIsError(false);
  }

  function setStudent(row: number, col: number, student: Student) {
    const newSeats = [...seats];
    newSeats[row][col] = student;
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
      .then((res) => {
        const executionResult = res as ExecutionResult;
        setResults(executionResult.new_seat_assignment);
        setResultIsOpen(true);
      })
      .catch((err) => {
        window.alert(err);
      })
      .finally(() => {
        setBackdropIsOpen(false);
      });
  }

  const Seats = (props: { width: number, depth: number, seats: (Student | null)[][] }) => {
    const elements = [];
    for (let i = 0; i < props.width * props.depth; i++) {
      let x = i % props.width;
      let y = Math.floor(i / props.width);
      elements.push(
        <Grid item xs={1}>
          <SeatCard
            student={props.seats[y][x]}
            onClick={() => {
              setEditedPosition([x, y]);
              if (props.seats[y][x] !== null) {
                setEditedStudent(props.seats[y][x]!);
              } else {
                setEditedStudent(defaultStudent);
              }

              toggleDrawer();
            }}
          />
        </Grid>
      );
    }
    return (
      <Grid container spacing={2} columns={props.width}>
        {elements}
      </Grid>
    );
  };

  return (
    <Box padding={1}>
      <Stack spacing={2}>
        <Seats width={width} depth={depth} seats={seats} />
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
            value={editedStudent.id}
            onChange={(e) => {
              setEditedStudent({ ...editedStudent, id: Number(e.target.value) })
              setIdInputIsError(false);
              setIdInputHelperText("");
            }}
            error={idInputIsError}
            helperText={idInputHelperText}
          />
          <Divider />

          <TextField
            label="名前"
            margin="normal"
            sx={{ boxShadow: 0 }}
            value={editedStudent.name}
            onChange={(e) => {
              setEditedStudent({ ...editedStudent, name: e.target.value })
              setNameInputIsError(false);
              setNameInputHelperText("");
            }}
            error={nameInputIsError}
            helperText={nameInputHelperText}
          />
          <Divider />

          <InputLabel id="gender-select">性別</InputLabel>
          <Select
            labelId="gender-select"
            value={editedStudent.gender}
            onChange={(e) => setEditedStudent({ ...editedStudent, gender: e.target.value })}
          >
            <MenuItem value="Male">男</MenuItem>
            <MenuItem value="Female">女</MenuItem>
          </Select>
          <Divider />

          <Typography gutterBottom>学力</Typography>
          <Rating
            value={editedStudent.academic_ability}
            onChange={(_, v) => {
              setEditedStudent({ ...editedStudent, academic_ability: v || editedStudent.academic_ability });
            }}
          />
          <Divider />

          <Typography gutterBottom>運動能力</Typography>
          <Rating
            value={editedStudent.exercise_ability}
            onChange={(_, v) => setEditedStudent({ ...editedStudent, exercise_ability: v || editedStudent.exercise_ability })}
          />
          <Divider />

          <Typography gutterBottom>リーダーシップ</Typography>
          <Rating
            value={editedStudent.leadership_ability}
            onChange={(_, v) => setEditedStudent({ ...editedStudent, leadership_ability: v || editedStudent.leadership_ability })}
          />
          <Divider />

          <Typography gutterBottom>要支援</Typography>
          <Checkbox
            checked={editedStudent.needs_assistance}
            onChange={(e) => setEditedStudent({ ...editedStudent, needs_assistance: e.target.checked })}
          />
          <Divider />

          <Stack direction="row" spacing={2}>
            <Button variant="outlined" onClick={toggleDrawer} sx={{ boxShadow: 0 }}>キャンセル</Button>
            <Button
              variant="contained"
              onClick={() => {
                let canBeSaved = true;
                if (editedStudent.name === "") {
                  setNameInputIsError(true);
                  setNameInputHelperText("名前を入力してください。");
                  canBeSaved = false;
                }

                const studentIds = seats.flat().map((student) => student?.id).filter((id) => id !== null) as number[];
                if (studentIds.includes(editedStudent.id) && editedStudent.id !== seats[editedPosition[1]][editedPosition[0]]?.id) {
                  setIdInputIsError(true);
                  setIdInputHelperText("番号が重複しています。");
                  canBeSaved = false;
                }

                if (canBeSaved) {
                  setStudent(editedPosition[1], editedPosition[0], editedStudent);
                  toggleDrawer();
                }
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

      <SizeConfigDialog
        open={sizeConfigIsOpen}
        defaultWidth={width}
        defaultDepth={depth}
        onClose={() => setSizeConfigIsOpen(false)}
        onSave={(width, depth) => ChangeSize(width, depth)}
      />
      <ResultDialog
        seats={result}
        open={resultIsOpen}
        onCloseClick={() => {
          setResultIsOpen(false);
        }}
      />
    </Box>
  );
}

export default EditLayout;