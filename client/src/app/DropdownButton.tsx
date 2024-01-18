import React, { ReactElement, useState } from "react";
import "./dropdownButton.css";

type DropdownOption = {
    name: string,
    onClick?: () => void,
}

export default function DropdownButton(props: { name: string, children: DropdownOption | DropdownOption[] }): ReactElement {
    const [open, setOpen] = useState<boolean>();
    const children = Array.isArray(props.children) ? props.children : [props.children];

    return <>
        <button className="flush dropdown-button" onMouseLeave={() => setOpen(false)} onClick={() => setOpen(!open)}>
            {props.name}
            <div className="button-dropdown">
                <ul hidden={!open}>
                    {children.map(opt => {
                        const onClick = opt.onClick ?? (() => { });
                        return <li>
                            <button className="flush" onClick={() => onClick()}>
                                {opt.name}
                            </button>
                        </li>
                    })}
                </ul>
            </div>
        </button >
    </>
}