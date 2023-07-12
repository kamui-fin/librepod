import Input from "../Input"
import styles from "./style.module.scss"
import { AiOutlineSearch } from "react-icons/ai"

interface Props {
    text: string
}

const SearchBar = ({ text }: Props) => {
    return (
        <div className={styles.container}>
            <AiOutlineSearch />
            <Input className={styles.input} placeholder={text} />
        </div>
    )
}

export default SearchBar
