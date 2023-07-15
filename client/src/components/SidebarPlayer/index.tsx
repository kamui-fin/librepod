import { BiSkipNext, BiSkipPrevious } from "react-icons/bi"
import cx from "classnames"
import { Range, getTrackBackground } from "react-range"
import { useState } from "react"
import {
    MdPause,
    MdReplay10,
    MdForward10,
} from "react-icons/md"
import styles from "./style.module.scss"

const SidebarPlayer = () => {
    const [values, setValues] = useState([0])
    return (
        <div className={styles.player}>
            <img src="https://crazy.capital/assets/icon-square-normal-dark-cn@4x.png" />
            <h3>322. 马自达昂克赛拉官降3万...</h3>
            <p> 别人研究车，而我研究你。一个可以当脱口秀听的汽车电台...</p>
            <div className={styles.slider}>
                <span>12:09</span>
                <Range values={values} min={0} max={60 * 40} step={1} onChange={values => setValues(values)} renderTrack={({ props, children }) => (
                    <div
                        {...props}
                        className={styles.sliderTrack}
                        style={{
                            ...props.style,
                            background: getTrackBackground({
                                values,
                                colors: ['#b8c0cc', '#3c4554'],
                                min: 0,
                                max: 60 * 40,
                            }),
                        }}
                    >
                        {children}
                    </div>
                )}
                    renderThumb={({ props }) => (
                        <div
                            {...props}
                            className={styles.sliderThumb}
                        />
                    )}
                />
                <span>-34:58</span>
            </div>
            <div className={styles.controls}>
                <div className={styles.iconOnly}>
                    <MdReplay10 />
                </div>
                <div className={styles.circleBtn}>
                    <BiSkipPrevious />
                </div>
                <div className={cx(styles.circleBtn, styles.pause)}>
                    <MdPause />
                </div>
                <div className={styles.circleBtn}>
                    <BiSkipNext />
                </div>
                <div className={styles.iconOnly}>
                    <MdForward10 />
                </div>
            </div>
        </div>
    )
}

export default SidebarPlayer;
