import React, { ReactElement, useContext, useState } from "react";
import DropdownButton, { Dropdown, Option } from "./DropdownButton.tsx";
import "./nav.css";
import MaterialIcon from "./Symbol.tsx";
import { EditorContext } from "./App.tsx";
import { saveFile } from '../index.tsx'

export default function Nav(): ReactElement {
    const [textContent, setTextContent] = useContext(EditorContext)!.textContent;
    const [filename, setFilename] = useContext(EditorContext)!.fileName;
    const [filenameField, setFilenameField] = useState<string>(filename);

    const newButton = () => {
        setTextContent("");
        setFilename("Untitled");
        setFilenameField("Untitled");
    };

    const upload = () => {
        const input = document.createElement('input');
        input.type = 'file';
        input.multiple = false;
        input.accept = ".txt";
        input.hidden = true;

        const fileListener = () => {
            if (input.files !== null && input.files.length === 1) {
                input.files[0].text()
                    .then(text => setTextContent(text));

                setFilename(input.files[0].name);
                setFilenameField(input.files[0].name);
            }

            input.removeEventListener('change', fileListener);
            document.body.removeChild(input);
        };

        input.addEventListener('change', fileListener)
        document.body.appendChild(input);

        input.click();
    };

    return <nav>
        <div className="nav-buttons">
            <DropdownButton>
                File
                <Dropdown>
                    <Option onClick={newButton}><MaterialIcon icon="add" />New</Option>
                    <Option onClick={() => saveFile(textContent, filename)}><MaterialIcon icon="save" />Save</Option>
                    <Option onClick={upload}><MaterialIcon icon="upload" />Import</Option>
                    <Option><MaterialIcon icon="download" />Export</Option>
                </Dropdown>
            </DropdownButton>
            <DropdownButton>
                Edit
                <Dropdown>
                    <Option>Erm</Option>
                </Dropdown>
            </DropdownButton>
            <DropdownButton>
                Format
                <Dropdown>
                    <Option>Erm</Option>
                </Dropdown>
            </DropdownButton>
            <DropdownButton>
                Help
                <Dropdown>
                    <Option>Erm</Option>
                </Dropdown>
            </DropdownButton>
        </div>
        <div className="file-name">
            <input className="flush" value={filenameField}
                onInput={e => {
                    const text = (e.target as HTMLInputElement).value;
                    if (text === null) return;

                    setFilenameField(text);
                }}
                onKeyDown={e => {
                    const text = (e.target as HTMLInputElement).value;
                    if (text === null) return;

                    if (e.key === "Enter") {
                        e.preventDefault();
                        if (text.trim() === "") {
                            const defaultName = "Untitled";
                            setFilenameField(defaultName);
                            setFilename(defaultName);
                        } else {
                            setFilename(text);
                        }
                    }
                }}
            />
        </div>
    </nav>
}