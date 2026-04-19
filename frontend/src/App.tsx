import { HomePage } from './components/home/HomePage'
import { ThemeProvider } from './theme/ThemeProvider'

function App() {
  return (
    <ThemeProvider>
      <HomePage />
    </ThemeProvider>
  )
}

export default App
