import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { save, open, confirm, message } from "@tauri-apps/api/dialog";
import { writeTextFile, readTextFile } from "@tauri-apps/api/fs";

import { Box, Drawer, Grid, Stack, TextField, Divider, Typography, InputLabel, Select, MenuItem, Checkbox, Button, IconButton, Tooltip, Rating, Backdrop } from "@mui/material";
import SeatCard from "./components/SeatCard";
import ResultDialog from "./components/ResultDialog";

import RotateLeftIcon from '@mui/icons-material/RotateLeft';
import { Dna } from "react-loader-spinner";

import type { Student } from "./types/Student";
import type { ExecutionResult } from "./types/ExecutionResult";
import ConfigDialog from "./components/ConfigDialog";

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

  async function changeSize(newWidth: number, newDepth: number) {
    const compressSeats = (seats: (Student | null)[][]) => {
      const rowCompressed = seats.filter((row) => row.some((student) => student !== null));
      if (rowCompressed.length === 0) {
        return [[null]];
      }
      let colCompressed = [];
      for (let i = 0; i < rowCompressed[0].length; i++) {
        const col = [];
        for (let j = 0; j < rowCompressed.length; j++) {
          col.push(rowCompressed[j][i]);
        }
        if (col.some((student) => student !== null)) {
          colCompressed.push(col);
        }
      }

      let res = [];
      for (let i = 0; i < colCompressed[0].length; i++) {
        const row = [];
        for (let j = 0; j < colCompressed.length; j++) {
          row.push(colCompressed[j][i]);
        }
        res.push(row);
      }

      return res;
    };

    const compressedSeats = compressSeats(seats);

    let canBeChanged = compressedSeats.length <= newDepth && compressedSeats[0].length <= newWidth;
    if (!canBeChanged) {
      canBeChanged = await confirm("一部の入力情報が削除されます。よろしいですか？", { title: "警告", type: "warning" });
    }
    if (canBeChanged) {
      setWidth(newWidth);
      setDepth(newDepth);

      const seats = compressedSeats;
      for (let i = 0; i < newDepth; i++) {
        if (seats[i] === undefined) seats.push([]);
        for (let j = compressedSeats[i].length; j < newWidth; j++) {
          seats[i].push(null);
        }
      }
      setSeats(seats);
      setSizeConfigIsOpen(false);
    }
  }


  useEffect(() => {
    listen("change_size", (_) => {
      setSizeConfigIsOpen(true);
    });

    listen("save", async (_) => {
      const path = await save({ defaultPath: "seats.json", filters: [{ name: "JSON", extensions: ["json"] }] });
      if (path) {
        writeTextFile(path, JSON.stringify(seats));
      }
    });

    listen("open", async (_) => {
      const path = await open({ filters: [{ name: "JSON", extensions: ["json"] }] });
      if (path) {
        try {
          const seats = JSON.parse(await readTextFile(String(path))) as (Student | null)[][];
          setSeats(seats);
          setWidth(seats[0].length);
          setDepth(seats.length);
        } catch (err) {
          await message("ファイルの読み込みに失敗しました。", { title: "エラー", type: "error" });
        }
      }
    });
  }, [seats, width, depth, sizeConfigIsOpen]);

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
        message(err, { title: "エラー", type: "error" });
      })
      .finally(() => {
        setBackdropIsOpen(false);
      });
  }

  async function saveResult() {
    const path = await save({ defaultPath: "result.json", filters: [{ name: "JSON", extensions: ["json"] }] });
    if (path) {
      writeTextFile(path, JSON.stringify(result));
    }
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
                  if (await confirm("入力情報をリセットします。よろしいですか？", { title: "警告", type: "warning" })) {
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

      <ConfigDialog
        open={sizeConfigIsOpen}
        defaultWidth={width}
        defaultDepth={depth}
        onClose={() => setSizeConfigIsOpen(false)}
        onSave={(width, depth) => changeSize(width, depth)}
      />
      <ResultDialog
        seats={result}
        open={resultIsOpen}
        onCloseClick={() => {
          setResultIsOpen(false);
        }}
        onSave={saveResult}
      />
    </Box>
  );
}

export default EditLayout;