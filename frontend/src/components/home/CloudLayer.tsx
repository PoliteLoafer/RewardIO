import type { ThemeName } from '../../theme/themes'
import { themes } from '../../theme/themes'
import { CloudGraphic } from './CloudGraphic'

type CloudLayerProps = {
  theme: ThemeName
}

type CloudItem = {
  top: string
  width: string
  opacity: number
  size: 'small' | 'large'
  duration?: string
  delay?: string
  left?: string
}

const cloudLayerClassName = 'pointer-events-none absolute inset-0 z-[1]'
const cloudClassName = 'cloud absolute left-[-38vw] h-auto max-w-[62vw]'
const cloudGraphicClassName = 'block h-auto w-full'

const cloudItems: CloudItem[] = [
  { top: '8%', width: 'clamp(15rem, 22vw, 19rem)', size: 'small', duration: '58s', delay: '-2s', opacity: 0.8 },
  { top: '30%', width: 'clamp(18rem, 26vw, 22.5rem)', size: 'small', duration: '62s', delay: '-44s', opacity: 0.6 },
  {
    top: '52%',
    left: '-42%',
    width: 'clamp(32rem, 54vw, 47.5rem)',
    size: 'large',
    opacity: 0.8,
    duration: '78s',
    delay: '-17s',
  },
  { top: '78%', width: 'clamp(14rem, 20vw, 17.5rem)', size: 'small', duration: '52s', delay: '-12s', opacity: 0.95 },
]

export function CloudLayer({ theme }: CloudLayerProps) {
  const palette = themes[theme].cloud

  return (
    <div className={cloudLayerClassName} aria-hidden="true">
      {cloudItems.map((cloud, index) => (
        <div
          key={`${theme}-${index}`}
          className={cloudClassName}
          style={{
            top: cloud.top,
            left: cloud.left,
            width: cloud.width,
            animationDuration: cloud.duration,
            animationDelay: cloud.delay,
            opacity: cloud.opacity,
          }}
        >
          <CloudGraphic palette={palette} size={cloud.size} className={cloudGraphicClassName} />
        </div>
      ))}
    </div>
  )
}
