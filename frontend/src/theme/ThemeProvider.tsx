import { createContext, useContext, useEffect, useMemo, useState } from 'react'
import type { ReactNode } from 'react'
import { themeOrder, themes, type ThemeName } from './themes'

type ThemeContextValue = {
  theme: ThemeName
  setTheme: (theme: ThemeName) => void
  cycleTheme: () => void
}

const ThemeContext = createContext<ThemeContextValue | null>(null)

const THEME_STORAGE_KEY = 'rewardio-theme'

export function ThemeProvider({ children }: { children: ReactNode }) {
  const [theme, setTheme] = useState<ThemeName>(() => {
    const savedTheme = localStorage.getItem(THEME_STORAGE_KEY)
    if (savedTheme && savedTheme in themes) {
      return savedTheme as ThemeName
    }
    return 'lavender'
  })

  useEffect(() => {
    localStorage.setItem(THEME_STORAGE_KEY, theme)
    document.documentElement.setAttribute('data-theme', theme)
  }, [theme])

  const value = useMemo(
    () => ({
      theme,
      setTheme,
      cycleTheme: () => {
        const currentIndex = themeOrder.indexOf(theme)
        const nextTheme = themeOrder[(currentIndex + 1) % themeOrder.length]
        setTheme(nextTheme)
      },
    }),
    [theme],
  )

  return <ThemeContext.Provider value={value}>{children}</ThemeContext.Provider>
}

export function useTheme() {
  const context = useContext(ThemeContext)
  if (!context) {
    throw new Error('useTheme must be used within ThemeProvider')
  }
  return context
}
