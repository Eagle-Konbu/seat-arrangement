import { Card, CardActionArea, CardContent, Grid, Typography } from "@mui/material";

function SeatCard(props: { student: Student, onClick: () => void }) {
  return (
    <Grid item xs={1}>
      <Card variant="outlined">
        <CardActionArea onClick={props.onClick}>
          <CardContent sx={{ display: 'flex', justifyContent: 'center', alignItems: 'center' }}>
            <Typography>
              {props.student.id}. {props.student.name}
            </Typography>
          </CardContent>
        </CardActionArea>
      </Card>
    </Grid>
  );
}

export default SeatCard;