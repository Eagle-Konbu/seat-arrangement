import { Dialog, DialogContent, DialogTitle, Box, Typography, Slider, Stack, DialogActions, Button, Tab, FormControl, InputLabel, Select, MenuItem, SelectChangeEvent } from "@mui/material";
import { TabContext, TabList, TabPanel } from "@mui/lab";

import { useState } from "react";

import type { WeightConfig } from "../types/WeightConfig";

function ConfigDialog(props: { open: boolean, defaultWidth: number, defaultDepth: number, defaultWeightConfig: WeightConfig, onClose?: () => void, onSave?: (width: number, depth: number, weightConfig: WeightConfig) => void }) {
  const [width, setWidth] = useState(props.defaultWidth);
  const [depth, setDepth] = useState(props.defaultDepth);

  const [tabValue, setTabValue] = useState("size");
  const [weightConfig, setWeightConfig] = useState<WeightConfig>({
    academic: 1,
    exercise: 1,
    leadership: 1,
    male_rate: 1
  });

  const [presetId, setPresetId] = useState("0");

  const handleTabChange = (event: React.SyntheticEvent, newValue: string) => {
    setTabValue(newValue);
  };
  const handleWeightPresetChange = (event: SelectChangeEvent) => {
    setPresetId(event.target.value);
    setWeightConfig(weightPresets[Number(event.target.value)].value);
  };

  const weightPresets = [
    {
      name: "デフォルト",
      value: {
        academic: 1,
        exercise: 1,
        leadership: 1,
        male_rate: 1
      }
    },
    {
      name: "学業重視",
      value: {
        academic: 2,
        exercise: 1,
        leadership: 1,
        male_rate: 1
      }
    },
    {
      name: "運動重視",
      value: {
        academic: 1,
        exercise: 2,
        leadership: 1,
        male_rate: 1
      }
    },
    {
      name: "リーダーシップ重視",
      value: {
        academic: 1,
        exercise: 1,
        leadership: 2,
        male_rate: 1
      }
    },
    {
      name: "男女比重視",
      value: {
        academic: 1,
        exercise: 1,
        leadership: 1,
        male_rate: 2
      }
    }
  ];

  return (
    <Dialog
      open={props.open}
      onClose={props.onClose}
      fullWidth={true}
      maxWidth="sm"
    >
      <DialogTitle>設定</DialogTitle>
      <DialogContent>
        <TabContext value={tabValue}>
          <Box sx={{ borderBottom: 1, borderColor: 'divider' }}>
            <TabList onChange={handleTabChange} aria-label="lab API tabs example">
              <Tab label="サイズ" value="size" />
              <Tab label="重視内容" value="weight" />
            </TabList>
          </Box>
          <Box padding={2}>
            <TabPanel value="size">
              <Stack spacing={2} direction="row">
                <Typography variant="subtitle1" gutterBottom>縦</Typography>
                <Slider
                  defaultValue={depth}
                  step={1}
                  min={1}
                  max={10}
                  marks
                  onChange={(_, v) => setDepth(v as number)}
                />
              </Stack>

              <Stack spacing={2} direction="row">
                <Typography variant="subtitle1" gutterBottom>横</Typography>
                <Slider
                  defaultValue={width}
                  step={1}
                  min={1}
                  max={10}
                  marks={Array(10).fill(0).map((_, i) => ({ value: i + 1, label: (i + 1).toString() }))}
                  onChange={(_, v) => setWidth(v as number)}
                />
              </Stack>
            </TabPanel>
            <TabPanel value="weight">
              <FormControl fullWidth>
                <InputLabel id="demo-simple-select-label">重視項目</InputLabel>
                <Select
                  labelId="demo-simple-select-label"
                  id="demo-simple-select"
                  value={presetId}
                  label="重視項目"
                  onChange={handleWeightPresetChange}
                >
                  {
                    weightPresets.map((preset, i) => <MenuItem value={i}>{preset.name}</MenuItem>)
                  }
                </Select>
              </FormControl>
            </TabPanel>
          </Box>
        </TabContext>
      </DialogContent>
      <DialogActions>
        <Stack direction="row" spacing={2}>
          <Button variant="outlined" onClick={props.onClose} sx={{ boxShadow: 0 }}>キャンセル</Button>
          <Button variant="contained" onClick={() => props.onSave?.(width, depth, weightConfig)}>保存</Button>
        </Stack>
      </DialogActions>
    </Dialog>
  )
}

export default ConfigDialog;