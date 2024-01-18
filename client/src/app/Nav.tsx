import React, { ReactElement } from "react";
import DropdownButton from "./DropdownButton.tsx";
import "./nav.css";

export default function Nav(): ReactElement {
    return <nav>
        <div className="nav-buttons">
            <DropdownButton name="File">
                {{ name: "Save", onClick: () => { } }}
                {{ name: "Save as", onClick: () => { } }}
            </DropdownButton>
            <DropdownButton name="Edit">
                {{ name: "Erm", onClick: () => { } }}
            </DropdownButton>
            <DropdownButton name="Format">
                {{ name: "Erm", onClick: () => { } }}
            </DropdownButton>
            <DropdownButton name="Help">
                {{ name: "Erm", onClick: () => { } }}
            </DropdownButton>
        </div>
        <div className="file-name">File name</div>
    </nav>
}