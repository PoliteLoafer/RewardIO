export type ThemeName = 'lavender' | 'sunset' | 'night'

export type CloudPalette = {
  start: string
  middle: string
  end: string
  smallOpacity: number
  largeOpacity: number
  shadowOpacity: number
}

type ThemeConfig = {
  label: string
  cloud: CloudPalette
  colors: {
    background: string
    heading: string
    accent: string
    muted: string
    card: string
    navButton: string
    navButtonText: string
    navButtonBorder: string
  }
}

export const themes: Record<ThemeName, ThemeConfig> = {
  lavender: {
    label: 'Lavender Sky',
    cloud: {
      start: '#7F75E9',
      middle: '#D9EDF6',
      end: '#FFFFFF',
      smallOpacity: 0.78,
      largeOpacity: 0.5,
      shadowOpacity: 0.25,
    },
    colors: {
      background: '#f3f3f7',
      heading: '#6d24e8',
      accent: '#8d84ff',
      muted: '#e8aaaa',
      card: 'rgba(246, 246, 251, 0.86)',
      navButton: '#7f1ff5',
      navButtonText: '#ffffff',
      navButtonBorder: '#7f1ff5',
    },
  },
  sunset: {
    label: 'Sunset Peach',
    cloud: {
      start: '#FF8A5B',
      middle: '#FFE2C7',
      end: '#FFF6EE',
      smallOpacity: 0.78,
      largeOpacity: 0.5,
      shadowOpacity: 0.25,
    },
    colors: {
      background: '#fff4eb',
      heading: '#cf4f1e',
      accent: '#ff915c',
      muted: '#b87e5d',
      card: 'rgba(255, 249, 245, 0.9)',
      navButton: '#e55e2d',
      navButtonText: '#ffffff',
      navButtonBorder: '#e55e2d',
    },
  },
  night: {
    label: 'Blue Night',
    cloud: {
      start: '#4B5AC9',
      middle: '#8EB8DD',
      end: '#D7E8F4',
      smallOpacity: 0.85,
      largeOpacity: 0.52,
      shadowOpacity: 0.4,
    },
    colors: {
      background: '#eaf0fa',
      heading: '#2e3f93',
      accent: '#556ccf',
      muted: '#6f83a5',
      card: 'rgba(238, 244, 253, 0.86)',
      navButton: '#3950b4',
      navButtonText: '#ffffff',
      navButtonBorder: '#3950b4',
    },
  },
}

export const themeOrder: ThemeName[] = ['lavender', 'sunset', 'night']
