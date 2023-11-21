import { useEffect } from "react"
import { toast } from "react-toastify"

interface UseErrorToastProps {
    error: Error | null
}

export const useErrorToast = ({ error }: UseErrorToastProps): void => {
    useEffect(() => {
        if (error) {
            toast.error(`Error: ${error.message}`, { toastId: "error" })
        }
    }, [error])
}
