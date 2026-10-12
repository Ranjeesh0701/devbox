import type { ButtonHTMLAttributes, ReactNode } from "react";

type Props = ButtonHTMLAttributes<HTMLButtonElement> & {
  variant?: "primary" | "secondary" | "ghost";
  icon?: ReactNode;
};

export function Button({
  variant = "secondary",
  icon,
  children,
  className = "",
  ...props
}: Props) {
  return (
    <button className={`button button-${variant} ${className}`} {...props}>
      {icon}
      {children}
    </button>
  );
}
