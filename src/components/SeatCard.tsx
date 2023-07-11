import { Card, CardContent, Grid, Typography } from "@mui/material";

function SeatCard(props: { student: Student }) {
  return (
    <Grid item xs={1}>
      <Card variant="outlined">
        <CardContent sx={{ display: 'flex', justifyContent: 'center', alignItems: 'center' }}>
          <Typography>
            {props.student.id}. {props.student.name}
          </Typography>
        </CardContent>
      </Card>
    </Grid>
  );
}

export default SeatCard;