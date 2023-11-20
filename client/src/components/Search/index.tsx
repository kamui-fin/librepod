import Input from "../Input"
import styles from "./style.module.scss"
import { AiOutlineSearch } from "react-icons/ai"

interface Props {
    text: string
    onSearch: (query: string) => void
}

const SearchBar = ({ text, onSearch }: Props) => {
    return (
        <div className={styles.container}>
            <AiOutlineSearch />
            <Input
                className={styles.input}
                placeholder={text}
                onChange={(ev) => {
                    const query = ev.target.value
                    onSearch(query)
                }}
            />
        </div>
    )
}

export default SearchBar
