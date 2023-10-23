import Input from "../Input"
import styles from "./style.module.scss"
import { AiOutlineSearch } from "react-icons/ai"

interface Props<T> {
    text: string
    data: Array<T>
    cmpKeys: string[]
    onSearch: (output: Array<T>) => void
}

const SearchBar = <T,>({ text, data, cmpKeys, onSearch }: Props<T>) => {
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
                            for (let key of cmpKeys) {
                                if (elm[key].includes(query)) {
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
