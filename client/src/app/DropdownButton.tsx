import React, { MouseEventHandler, ReactElement, ReactNode, createContext, useContext, useState } from "react";
import "./dropdownButton.css";

const OpenContext = createContext<boolean>(false);

export default function DropdownButton(props: {
    children: [ReactNode, ReactElement<DropdownProps>]
}): ReactElement {
    const [open, setOpen] = useState<boolean>(false);

    return <OpenContext.Provider value={open}>
        <button className="flush dropdown-button" onMouseLeave={() => setOpen(false)} onClick={() => setOpen(!open)}>
            {props.children}
        </button >
    </OpenContext.Provider>
}

type DropdownProps = {
    children: ReactElement<OptionProps> | ReactElement<OptionProps>[]
}

export function Dropdown(props: DropdownProps): ReactElement {
    const children = Array.isArray(props.children) ? props.children : [props.children];
    const open = useContext(OpenContext);

    return <div className="button-dropdown" hidden={!open}>
        <ul>
            {children.map((el, key) => <li key={key}>
                {el}
            </li>)}
        </ul>
    </div>
}

type OptionProps = {
    children: ReactNode,
    onClick?: MouseEventHandler<HTMLButtonElement>
};

export function Option(props: OptionProps): ReactElement {
    const onClick = props.onClick ?? (() => { });
    return <button className="flush" onClick={e => onClick(e)}>
        {props.children}
    </button>
}