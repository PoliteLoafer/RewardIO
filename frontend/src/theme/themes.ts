export type ThemeName = 'lavender' | 'sunset' | 'night'

type ThemeConfig = {
  label: string
  cloudSrc: string
  largeCloudSrc: string
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
    cloudSrc: '/clouds/lavender.svg',
    largeCloudSrc: '/clouds/lavender-large.svg',
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
    cloudSrc: '/clouds/sunset.svg',
    largeCloudSrc: '/clouds/sunset-large.svg',
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
    cloudSrc: '/clouds/night.svg',
    largeCloudSrc: '/clouds/night-large.svg',
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
