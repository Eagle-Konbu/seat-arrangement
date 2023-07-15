import { Dialog, DialogContent, DialogTitle, Box, Typography, Slider, Stack, DialogActions, Button } from "@mui/material";
import { useState } from "react";

function SizeConfigDialog(props: { open: boolean, defaultWidth: number, defaultDepth: number, onClose?: () => void, onSave?: (width: number, depth: number) => void }) {
  const [width, setWidth] = useState(props.defaultWidth);
  const [depth, setDepth] = useState(props.defaultDepth);

  return (
    <Dialog
      open={props.open}
      onClose={props.onClose}
    >
      <DialogTitle>サイズ設定</DialogTitle>
      <DialogContent>
        <Box padding={2}>
          <Stack spacing={2} direction="row">
            <Typography variant="subtitle1" gutterBottom>縦</Typography>
            <Slider
              defaultValue={depth}
              step={1}
              min={1}
              max={10}
              marks
              valueLabelDisplay="auto"
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
              marks
              valueLabelDisplay="auto"
              onChange={(_, v) => setWidth(v as number)}
            />
          </Stack>
        </Box>
      </DialogContent>
      <DialogActions>
        <Stack direction="row" spacing={2}>
          <Button variant="outlined" onClick={props.onClose} sx={{ boxShadow: 0 }}>キャンセル</Button>
          <Button variant="contained" onClick={() => props.onSave?.(width, depth)}>保存</Button>
        </Stack>
      </DialogActions>
    </Dialog>
  )
}

export default SizeConfigDialog;