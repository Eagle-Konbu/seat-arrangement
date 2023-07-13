import { useEffect, useState } from "react";
import { Box, Grid, Stack } from "@mui/material";
import { useSearchParams } from "react-router-dom";
import SeatCard from "./components/SeatCard";

import type { Student } from "./types/Student";
import type { ExecutionResult } from "./types/ExecutionResult";

function Result() {
  const [searchParams, _] = useSearchParams();
  const result = searchParams.get("result");

  const [width, setWidth] = useState(0);
  const [depth, setDepth] = useState(0);

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

  useEffect(() => {
    const resultJson = JSON.parse(result!) as ExecutionResult;
    setWidth(resultJson.new_seat_assignment[0].length);
    setDepth(resultJson.new_seat_assignment.length);
    setSeats(resultJson.new_seat_assignment);
  }, [result]);

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

export default Result;
