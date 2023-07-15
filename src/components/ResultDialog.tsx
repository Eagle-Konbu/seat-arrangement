import { Box, Stack, Grid, Dialog, DialogContent, DialogTitle, DialogActions, Button } from "@mui/material"
import { Student } from "../types/Student";
import SeatCard from "./SeatCard";

function ResultDialog(props: { seats: (Student | null)[][], open: boolean, onClose?: () => void, onCloseClick?: () => void }) {

  const width = props.seats[0].length;
  const depth = props.seats.length;

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
      </DialogActions>
    </Dialog>

  );
}

export default ResultDialog;