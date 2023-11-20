import { RefObject, useEffect } from "react"

export function useClickOutside(
    refs: RefObject<HTMLElement>[],
    callback: () => void,
) {
    useEffect(() => {
        const handleClickOutside = (event: MouseEvent) => {
            const containsNoneOf = refs
                .map(
                    (ref) =>
                        ref.current &&
                        !ref.current.contains(event.target as Node),
                )
                .every((v) => v === true)
            if (containsNoneOf) callback()
        }

        document.addEventListener("mousedown", handleClickOutside)

        return () => {
            document.removeEventListener("mousedown", handleClickOutside)
        }
    }, [refs, callback])
}
