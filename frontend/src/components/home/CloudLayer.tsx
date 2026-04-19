import { themes, type ThemeName } from '../../theme/themes'

type CloudLayerProps = {
  theme: ThemeName
}

type CloudItem = {
  top: string
  width: string
  opacity: number
  duration?: string
  delay?: string
  left?: string
  isLarge?: boolean
}

const cloudItems: CloudItem[] = [
  { top: '8%', width: '300px', duration: '58s', delay: '-2s', opacity: 0.8 },
  { top: '30%', width: '360px', duration: '62s', delay: '-44s', opacity: 0.6 },
  {
    top: '52%',
    left: '-46vw',
    width: '760px',
    opacity: 0.8,
    duration: '78s',
    delay: '-17s',
    isLarge: true,
  },
  { top: '78%', width: '280px', duration: '52s', delay: '-12s', opacity: 0.95 },
]

export function CloudLayer({ theme }: CloudLayerProps) {
  const currentTheme = themes[theme]

  return (
    <div className="cloud-layer" aria-hidden="true">
      {cloudItems.map((cloud, index) => (
        <img
          key={`${theme}-${index}`}
          className="cloud"
          src={cloud.isLarge ? currentTheme.largeCloudSrc : currentTheme.cloudSrc}
          alt=""
          style={{
            top: cloud.top,
            left: cloud.left,
            width: cloud.width,
            animationDuration: cloud.duration,
            animationDelay: cloud.delay,
            opacity: cloud.opacity,
          }}
        />
      ))}
    </div>
  )
}
