import { ThemeOptions, ThemeProvider, createTheme } from "@mui/material";
import EditLayout from "./EditLayout";

function App() {
  const theme = createTheme({
    palette: {
      mode: 'light',
      primary: {
        main: '#1f9e92',
      },
      secondary: {
        main: '#dc5481',
      },
    },
  });

  return (
    <ThemeProvider theme={theme}>
      <EditLayout />
    </ThemeProvider>
  );
}

export default App;
