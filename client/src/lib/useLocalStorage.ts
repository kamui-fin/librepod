import { useState } from "react"

export const useLocalStorage = <T>(key: string, defaultValue: T) => {
    const [storedValue, setStoredValue] = useState<T>(() => {
        try {
            const value = window.localStorage.getItem(key)
            if (value) {
                return JSON.parse(value) as T
            } else {
                window.localStorage.setItem(key, JSON.stringify(defaultValue))
                return defaultValue
            }
        } catch (err) {
            return defaultValue
        }
    })
    const setValue = (newValue: T) => {
        try {
            window.localStorage.setItem(key, JSON.stringify(newValue))
        } catch (err) {
            console.error(err);
        }
        setStoredValue(newValue)
    }
    return [storedValue, setValue] as const
}
