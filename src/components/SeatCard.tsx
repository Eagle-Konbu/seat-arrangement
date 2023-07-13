import { Card, CardActionArea, CardContent, Grid, Tooltip, Typography } from "@mui/material";

import type { Student } from "../types/Student";

function SeatCard(props: { student: Student | null, onClick?: () => void }) {
  const studentInfo = (student: Student) => {
    let res = student.gender == "Male" ? "男" : "女";
    res += ` 学力: ${student.academic_ability} 運動能力: ${student.exercise_ability} リーダーシップ: ${student.leadership_ability}`
    if (student.needs_assistance) {
      res += " 要支援"
    }
    return res;
  };

  if (props.student === null) {
    return (
      <Card variant="outlined">
          <CardActionArea onClick={props.onClick}>
            <CardContent sx={{ display: 'flex', justifyContent: 'center', alignItems: 'center' }}>
              <Typography>
                未使用
              </Typography>
            </CardContent>
          </CardActionArea>
        </Card>
    );
  } else {
    return (
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
    );
  }
}

export default SeatCard;