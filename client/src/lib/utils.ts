import { DateTime } from "luxon"
import React from "react";

export const getHumanDate = (date: number): string => {
    const datetime = DateTime.fromJSDate(new Date(date / 1000))
    return datetime.toRelative()!
}

export function createCtx<ContextType>() {
    const ctx = React.createContext<ContextType | undefined>(undefined);
    const useContext = () => {
        const c = React.useContext(ctx);
        if (!c)
            throw new Error(
                "useCtx must be inside a Provider with a value"
            );
        return c;
    }
    return [useContext, ctx.Provider] as const;
}
