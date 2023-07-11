import Axios from "axios"

export const axios = Axios.create({
    baseURL: import.meta.env.VITE_API_URL,
})

axios.interceptors.response.use(
    function(response) {
        return response
    },
    function(error) {
        if (error.status === 401) {
            localStorage.removeItem("user")
        }
        return Promise.reject(error)
    }
)
