
import { cn } from "@/utils/style";
import { forwardRef } from "react";

export const Card = forwardRef(
  ({ className, ...props }, ref) => (
    <div
      ref={ref}
      className={cn("flex flex-col space-y-3 rounded border bg-background p-6", className)}
      {...props}
    />
  ),
);

Card.displayName = "Card";

export const CardHeader = forwardRef(
  ({ className, ...props }, ref) => (
    <div ref={ref} className={cn("flex flex-col space-y-1", className)} {...props} />
  ),
);

CardHeader.displayName = "CardHeader";

export const CardTitle = forwardRef(
  ({ className, ...props }, ref) => (
    <h3
      ref={ref}
      className={cn("font-semibold leading-normal tracking-tight", className)}
      {...props}
    >
      {props.children}
    </h3>
  ),
);

CardTitle.displayName = "CardTitle";

export const CardDescription = forwardRef(({ className, ...props }, ref) => (
  <div
    ref={ref}
    className={cn("text-xs font-medium leading-relaxed opacity-80", className)}
    {...props}
  />
));

CardDescription.displayName = "CardDescription";

export const CardContent = forwardRef(
  ({ className, ...props }, ref) => <div ref={ref} className={className} {...props} />,
);

CardContent.displayName = "CardContent";

export const CardFooter = forwardRef(
  ({ className, ...props }, ref) => (
    <div ref={ref} className={cn("flex items-center p-6", className)} {...props} />
  ),
);

CardFooter.displayName = "CardFooter";
