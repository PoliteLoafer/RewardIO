import { useId } from 'react'
import type { CloudPalette } from '../../theme/themes'

type CloudGraphicProps = {
  palette: CloudPalette
  size: 'small' | 'large'
  className?: string
}

const SMALL_CLOUD_PATH =
  'M200.295 0C249.338 0 291.193 32.3088 307.605 77.7588C309.624 77.6962 311.653 77.6631 313.694 77.6631C386.212 77.6631 445 117.585 445 166.831C445 216.077 386.212 256 313.694 256C293.499 256 274.37 252.902 257.277 247.371H174.567C157.91 252.861 138.622 256 118.063 256C55.0681 256 4 226.541 4 190.202C4 160.339 38.4878 135.123 85.7412 127.084C85.6276 124.888 85.5684 122.675 85.5684 120.449C85.5684 53.9269 136.933 0 200.295 0Z'

const LARGE_CLOUD_PATH =
  'M481.5 0C569.589 0 641 42.7568 641 95.5C641 109.986 635.612 123.718 625.973 136.019C626.979 136.007 627.988 136 629 136C705.768 136 768 170.474 768 213C768 255.526 705.768 290 629 290C603.426 290 579.465 286.172 558.881 279.497C531.507 300.451 487.998 314 439 314C416.045 314 394.296 311.024 374.845 305.709C344.475 311 309.808 314 273 314C217.279 314 166.463 307.127 128.036 295.832C122.488 296.599 116.798 297 111 297C51.9055 297 4 255.586 4 204.5C4 153.414 51.9055 112 111 112C124.47 112 137.358 114.152 149.23 118.079C153.14 84.559 206.834 58 272.5 58C293.303 58 312.905 60.6652 330.103 65.373C351.174 27.3932 410.993 0 481.5 0Z'

export function CloudGraphic({ palette, size, className = '' }: CloudGraphicProps) {
  const id = useId().replace(/:/g, '')
  const gradientId = `cloud-gradient-${id}`
  const filterId = `cloud-shadow-${id}`

  if (size === 'large') {
    return (
      <svg viewBox="0 0 772 322" xmlns="http://www.w3.org/2000/svg" className={className}>
        <g opacity={palette.largeOpacity} filter={`url(#${filterId})`}>
          <path d={LARGE_CLOUD_PATH} fill={`url(#${gradientId})`} />
        </g>
        <defs>
          <filter id={filterId} x="0" y="0" width="772" height="322" filterUnits="userSpaceOnUse" colorInterpolationFilters="sRGB">
            <feFlood floodOpacity="0" result="BackgroundImageFix" />
            <feColorMatrix
              in="SourceAlpha"
              type="matrix"
              values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0"
              result="hardAlpha"
            />
            <feOffset dy="4" />
            <feGaussianBlur stdDeviation="2" />
            <feComposite in2="hardAlpha" operator="out" />
            <feColorMatrix type="matrix" values={`0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 ${palette.shadowOpacity} 0`} />
            <feBlend mode="normal" in2="BackgroundImageFix" result="effect1_dropShadow" />
            <feBlend mode="normal" in="SourceGraphic" in2="effect1_dropShadow" result="shape" />
          </filter>
          <linearGradient id={gradientId} x1="386" y1="0" x2="386" y2="314" gradientUnits="userSpaceOnUse">
            <stop offset="0.514423" stopColor={palette.start} />
            <stop offset="0.854167" stopColor={palette.end} />
          </linearGradient>
        </defs>
      </svg>
    )
  }

  return (
    <svg viewBox="0 0 449 264" xmlns="http://www.w3.org/2000/svg" className={className}>
      <g opacity={palette.smallOpacity} filter={`url(#${filterId})`}>
        <path d={SMALL_CLOUD_PATH} fill={`url(#${gradientId})`} />
      </g>
      <defs>
        <filter id={filterId} x="0" y="0" width="449" height="264" filterUnits="userSpaceOnUse" colorInterpolationFilters="sRGB">
          <feFlood floodOpacity="0" result="BackgroundImageFix" />
          <feColorMatrix
            in="SourceAlpha"
            type="matrix"
            values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0"
            result="hardAlpha"
          />
          <feOffset dy="4" />
          <feGaussianBlur stdDeviation="2" />
          <feComposite in2="hardAlpha" operator="out" />
          <feColorMatrix type="matrix" values={`0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 ${palette.shadowOpacity} 0`} />
          <feBlend mode="normal" in2="BackgroundImageFix" result="effect1_dropShadow" />
          <feBlend mode="normal" in="SourceGraphic" in2="effect1_dropShadow" result="shape" />
        </filter>
        <linearGradient id={gradientId} x1="224.5" y1="0" x2="224.5" y2="256" gradientUnits="userSpaceOnUse">
          <stop stopColor={palette.start} />
          <stop offset="0.78" stopColor={palette.middle} />
          <stop offset="1" stopColor={palette.end} />
        </linearGradient>
      </defs>
    </svg>
  )
}
