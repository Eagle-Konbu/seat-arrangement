import { Box, Stack, Grid } from "@mui/material"
import { Student } from "../types/Student";
import SeatCard from "./SeatCard";

function SeatLayout(props: {seats: (Student | null)[][]}) {

  const width = props.seats[0].length;
  const depth = props.seats.length;

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
  );
}