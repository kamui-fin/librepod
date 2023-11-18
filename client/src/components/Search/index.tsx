import Input from "../Input"
import styles from "./style.module.scss"
import { AiOutlineSearch } from "react-icons/ai"

// eslint-disable-next-line @typescript-eslint/no-explicit-any
type SearchableObject = { [key: string]: any };

interface Props<T extends SearchableObject> {
    text: string
    data: Array<T>
    cmpKeys: string[]
    onSearch: (output: Array<T>) => void
}

const SearchBar = <T extends SearchableObject,>({ text, data, cmpKeys, onSearch }: Props<T>) => {
    return (
        <div className={styles.container}>
            <AiOutlineSearch />
            <Input
                className={styles.input}
                placeholder={text}
                onChange={(ev) => {
                    const query = ev.target.value

                    onSearch(
                        data.filter((elm) => {
                            for (const key of cmpKeys) {
                                if (String(elm[key]).includes(query)) {
                                    return true
                                }
                            }
                            return false
                        }),
                    )
                }}
            />
        </div>
    )
}

export default SearchBar
