import { Box, Stack, Grid, Dialog, DialogContent, DialogTitle, DialogActions, Button, Menu, MenuItem } from "@mui/material"
import { Student } from "../types/Student";
import SeatCard from "./SeatCard";
import React from "react";

function ResultDialog(props: { seats: (Student | null)[][], open: boolean, onClose?: () => void, onCloseClick?: () => void, onSave?: () => void, onPdfSave?: () => void, onCsvSave?: () => void }) {

  const width = props.seats[0].length;
  const depth = props.seats.length;

  const [menuAnchorEl, setMenuAnchorEl] = React.useState<null | HTMLElement>(null);
  const menuOpen = Boolean(menuAnchorEl);

  return (
    <Dialog
      open={props.open}
      fullWidth={true}
      onClose={props.onClose}
      maxWidth="xl"
    >
      <DialogTitle>
        結果
      </DialogTitle>
      <DialogContent>
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
                        student={props.seats[y][x]}
                      />
                    </Grid>
                  );
                }
                return elements;
              })()}
            </Grid>
          </Stack>
        </Box>
      </DialogContent>
      <DialogActions>
        <Button onClick={props.onCloseClick} sx={{ boxShadow: 0 }}>閉じる</Button>
        <Button onClick={(event: React.MouseEvent<HTMLButtonElement>) => setMenuAnchorEl(event.currentTarget)} sx={{ boxShadow: 0 }}>保存</Button>
        <Menu
          id="file-menu"
          anchorEl={menuAnchorEl}
          open={menuOpen}
          onClose={() => setMenuAnchorEl(null)}
          MenuListProps={{
            'aria-labelledby': 'file-menu',
          }}
        >
          <MenuItem onClick={props.onSave}>作業ファイル</MenuItem>
          <MenuItem onClick={props.onCsvSave}>CSV</MenuItem>
          <MenuItem onClick={props.onPdfSave}>PDF</MenuItem>
        </Menu>
      </DialogActions>
    </Dialog>

  );
}

export default ResultDialog;