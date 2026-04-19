import { CloudLayer } from './CloudLayer'
import { useTheme } from '../../theme/ThemeProvider'
import { themes } from '../../theme/themes'

export function HomePage() {
  const { theme, cycleTheme } = useTheme()
  const currentTheme = themes[theme]

  return (
    <main className="home-page">
      <CloudLayer theme={theme} />

      <header className="home-header">
        <h1 className="logo">Rewardio</h1>
        <nav className="header-actions">
          <button type="button" className="btn btn-primary">
            Contact
          </button>
          <button type="button" className="btn btn-primary">
            Learn More
          </button>
          <button type="button" className="btn btn-outline" onClick={cycleTheme}>
            Theme: {currentTheme.label}
          </button>
        </nav>
      </header>

      <section className="hero-card">
        <h2>Be brave to learn</h2>
        <p>Go ahead and say just a little more about what you do</p>
        <div className="hero-actions">
          <button type="button" className="btn btn-primary">
            Sign in
          </button>
          <button type="button" className="btn btn-outline">
            Sign up
          </button>
        </div>
      </section>
    </main>
  )
}
