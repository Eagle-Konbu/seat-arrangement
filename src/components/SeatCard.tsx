import { Card, CardActionArea, CardContent, Grid, Tooltip, Typography } from "@mui/material";

function SeatCard(props: { student: Student, onClick: () => void }) {
  const studentInfo = (student: Student) => {
    let res = `学力: ${student.academic_ability} 運動能力: ${student.exercise_ability} リーダーシップ: ${student.leadership_ability}`
    if (student.needs_assistance) {
      res += " 要支援"
    }
    return res;
  };
  return (
    <Grid item xs={1}>
      <Tooltip title={studentInfo(props.student)} arrow>
        <Card variant="outlined">
          <CardActionArea onClick={props.onClick}>
            <CardContent sx={{ display: 'flex', justifyContent: 'center', alignItems: 'center' }}>
              <Typography>
                {props.student.id}. {props.student.name}
              </Typography>
            </CardContent>
          </CardActionArea>
        </Card>
      </Tooltip>
    </Grid>
  );
}

export default SeatCard;