import { Button, FormControl, Grid, InputLabel, MenuItem, Select, Typography } from "@mui/material";
import { invoke } from "@tauri-apps/api/tauri";
import { useState } from "react";

function SizeConfig() {
  const [width, setWidth] = useState(5);
  const [depth, setDepth] = useState(5);

  async function openEditScreen() {
    await invoke("open_seats_edit_window", { width, depth }).catch((e) => window.alert(e));
  }

  return (
    <div>
      <Typography variant="subtitle1" gutterBottom>サイズ設定</Typography>
      <Grid container spacing={2}>
        <Grid item xs={6}>
          <FormControl fullWidth>
            <InputLabel id="depth-label">縦</InputLabel>
            <Select
              labelId="depth-label"
              id="depth-select"
              value={depth}
              onChange={(e) => setDepth(Number(e.target.value))}
              label="縦"
            >
              {
                [1, 2, 3, 4, 5, 6, 7, 8, 9, 10].map((i) => <MenuItem value={i}>{i}</MenuItem>)
              }
            </Select>
          </FormControl>
        </Grid>
        <Grid item xs={6}>
          <FormControl fullWidth>
            <InputLabel id="width-label">横</InputLabel>
            <Select
              labelId="width-label"
              id="width-select"
              value={width}
              onChange={(e) => setWidth(Number(e.target.value))}
              label="横"
            >
              {
                [1, 2, 3, 4, 5, 6, 7, 8, 9, 10].map((i) => <MenuItem value={i + 1}>{i + 1}</MenuItem>)
              }
            </Select>
          </FormControl>
        </Grid>
        <Grid item xs={12}>
          <Button
            fullWidth
            variant="contained"
            color="primary"
            onClick={openEditScreen}
          >
            OK
          </Button>
        </Grid>
      </Grid>
    </div>
  );
}

export default SizeConfig;