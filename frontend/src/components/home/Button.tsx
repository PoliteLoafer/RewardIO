import type { ButtonHTMLAttributes, ReactNode } from 'react'

type ButtonVariant = 'primary' | 'outline'

type ButtonProps = {
  children: ReactNode
  variant?: ButtonVariant
} & ButtonHTMLAttributes<HTMLButtonElement>

const buttonBaseClassName =
  'cursor-pointer rounded-xl border-[1.5px] px-5 py-2.5 text-[1.05rem] font-semibold shadow-[0_4px_10px_rgba(30,34,73,0.14)] transition duration-150 ease-out hover:-translate-y-px hover:shadow-[0_5px_12px_rgba(30,34,73,0.16)] active:translate-y-0 active:shadow-[0_3px_8px_rgba(30,34,73,0.12)] motion-reduce:transform-none motion-reduce:transition-none'
const buttonVariantClassNames: Record<ButtonVariant, string> = {
  outline: 'border-[var(--button-border)] bg-[rgba(255,255,255,0.72)] text-[var(--button)]',
  primary: 'border-[var(--button-border)] bg-[var(--button)] text-[var(--button-text)]',
}

export function Button({ children, className = '', variant = 'primary', type = 'button', ...props }: ButtonProps) {
  const classes = [buttonBaseClassName, buttonVariantClassNames[variant], className].filter(Boolean).join(' ')

  return (
    <button type={type} className={classes} {...props}>
      {children}
    </button>
  )
}
