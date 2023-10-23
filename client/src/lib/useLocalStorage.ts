import { useState } from "react"

export const useLocalStorage = (key: string, defaultValue: object) => {
    const [storedValue, setStoredValue] = useState(() => {
        try {
            const value = window.localStorage.getItem(key)
            if (value) {
                return JSON.parse(value)
            } else {
                window.localStorage.setItem(key, JSON.stringify(defaultValue))
                return defaultValue
            }
        } catch (err) {
            return defaultValue
        }
    })
    const setValue = (newValue: object) => {
        try {
            window.localStorage.setItem(key, JSON.stringify(newValue))
        } catch (err) {}
        setStoredValue(newValue)
    }
    return [storedValue, setValue]
}
