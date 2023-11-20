// eslint-disable-next-line @typescript-eslint/no-explicit-any
type SearchableObject = { [key: string]: any }

export const keywordSelect = <T extends SearchableObject>(
    data: T[],
    cmpKeys: string[],
    keyword: string,
) => {
    return data.filter((element) => {
        for (const key of cmpKeys) {
            if (String(element[key]).includes(keyword)) {
                return true
            }
        }
        return false
    })
}
