import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useSearchParams } from "react-router-dom";
import { Card, CardContent, Grid, Typography } from "@mui/material";

function EditLayout() {

  const [searchParams, _] = useSearchParams();

  const width = Number(searchParams.get("width"));
  const depth = Number(searchParams.get("depth"));

  return (
    <div>
      <Grid container spacing={2} columns={width}>
        {(() => {
          const elements = [];
          for (let i = 0; i < width * depth; i++) {
            elements.push(
              <Grid item xs={1}>
                <Card variant="outlined">
                  <CardContent sx={{ display: 'flex', justifyContent: 'center', alignItems: 'center' }}>
                    <Typography>
                      1. 田中 太郎
                    </Typography>
                  </CardContent>
                </Card>
              </Grid>
            );
          }
          return elements;
        })()}
      </Grid>
    </div>
  );
}

export default EditLayout;