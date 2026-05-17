import { CloudLayer } from './CloudLayer'
import { Button } from './Button'
import { GlassPanel } from './GlassPanel'
import { useTheme } from '../../theme/ThemeProvider'
import { themes } from '../../theme/themes'

const pageClassName = 'relative min-h-screen overflow-hidden px-12 py-8 max-lg:px-6 max-lg:pb-10 max-lg:pt-6 max-md:px-4 max-md:py-4'
const headerClassName = 'relative z-[3] flex items-center justify-between gap-4 max-lg:flex-col max-lg:items-start'
const headerActionsClassName = 'flex flex-wrap items-center gap-3 max-md:gap-2'
const heroPanelClassName = 'text-center max-lg:mt-[4.6rem] max-md:mt-[3.6rem]'
const heroActionsClassName = 'flex flex-wrap items-center justify-center gap-3'

export function HomePage() {
  const { theme, cycleTheme } = useTheme()
  const currentTheme = themes[theme]

  return (
    <main className={pageClassName}>
      <CloudLayer theme={theme} />

      <header className={headerClassName}>
        <h1 className="home-logo">RewardIO</h1>
        <nav className={headerActionsClassName}>
          <Button>Contact</Button>
          <Button>Learn More</Button>
          <Button variant="outline" onClick={cycleTheme}>
            Theme: {currentTheme.label}
          </Button>
        </nav>
      </header>

      <GlassPanel className={heroPanelClassName}>
        <h2 className="hero-title">Be brave to learn</h2>
        <p className="hero-text">
          Go ahead and say just a little more about what you do
        </p>
        <div className={heroActionsClassName}>
          <Button>Sign in</Button>
          <Button variant="outline">Sign up</Button>
        </div>
      </GlassPanel>
    </main>
  )
}
