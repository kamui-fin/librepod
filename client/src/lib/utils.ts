import { DateTime } from "luxon"

export const getHumanDate = (date: number): string => {
    const datetime = DateTime.fromJSDate(new Date(date / 1000))
    return datetime.toRelative()!
}
