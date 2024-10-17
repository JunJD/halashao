import { Slot } from "@radix-ui/react-slot";

import { forwardRef } from "react";
import { buttonVariants } from "./variants/button";
import { cn } from "@/utils/style";

export const Button = forwardRef(
    ({ className, variant, size, asChild = false, ...props }, ref) => {
        const Component = asChild ? Slot : "button";

        return (
            <Component
                ref={ref}
                className={cn(buttonVariants({ variant, size, className }))}
                {...props}
            />
        );
    },
);

Button.displayName = "Button";
