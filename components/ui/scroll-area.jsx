import { cn } from "@/utils/style";
import * as ScrollAreaPrimitive from "@radix-ui/react-scroll-area";

import { forwardRef } from "react";

export const ScrollArea = forwardRef(
  (
    {
      type = "scroll",
      orientation = "vertical",
      hideScrollbar = false,
      className,
      children,
      ...props
    },
    ref,
  ) => (
    <ScrollAreaPrimitive.Root
      ref={ref}
      type={type}
      className={cn("relative overflow-hidden", className)}
      {...props}
    >
      <ScrollAreaPrimitive.Viewport className="size-full rounded-[inherit]">
        {children}
      </ScrollAreaPrimitive.Viewport>
      <ScrollBar orientation={orientation} className={cn(hideScrollbar && "opacity-0")} />
      <ScrollAreaPrimitive.Corner />
    </ScrollAreaPrimitive.Root>
  ),
);

ScrollArea.displayName = ScrollAreaPrimitive.Root.displayName;

export const ScrollBar = forwardRef(({ className, orientation, ...props }, ref) => (
  <ScrollAreaPrimitive.ScrollAreaScrollbar
    ref={ref}
    orientation={orientation}
    className={cn(
      "flex touch-none select-none transition-colors",
      orientation === "vertical" && "h-full w-2.5 border-l border-l-transparent p-px",
      orientation === "horizontal" && "h-2.5 border-t border-t-transparent p-px",
      className,
    )}
    {...props}
  >
    <ScrollAreaPrimitive.ScrollAreaThumb className="relative flex-1 rounded-full bg-border" />
  </ScrollAreaPrimitive.ScrollAreaScrollbar>
));

ScrollBar.displayName = ScrollAreaPrimitive.ScrollAreaScrollbar.displayName;
