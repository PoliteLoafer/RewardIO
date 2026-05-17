import type { HTMLAttributes, ReactNode } from 'react'

type GlassPanelProps = {
  children: ReactNode
} & HTMLAttributes<HTMLElement>

const glassPanelBaseClassName =
  'relative z-[3] mx-auto mt-[clamp(5rem,16vh,11rem)] w-full max-w-[760px] overflow-hidden border-[1.5px] border-[rgba(255,255,255,0.35)] bg-[var(--card)] p-[clamp(1.6rem,3.5vw,3rem)] shadow-[0_20px_48px_rgba(30,34,73,0.025),inset_0_1px_0_rgba(255,255,255,0.005)] backdrop-blur-[5px] backdrop-saturate-[10%] before:pointer-events-none before:absolute before:inset-0 before:bg-[rgba(255,255,255,0.003)] [&>*]:relative [&>*]:z-[1]'

export function GlassPanel({ children, className = '', ...props }: GlassPanelProps) {
  const classes = [glassPanelBaseClassName, className].filter(Boolean).join(' ')

  return (
    <section className={classes} {...props}>
      {children}
    </section>
  )
}
