import React, { ReactElement, useContext, useMemo, useRef, useState } from "react";
import DropdownButton, { Dropdown, Option } from "./DropdownButton.tsx";
import "./nav.css";
import MaterialIcon from "./Symbol.tsx";
import { EditorContext } from "./App.tsx";
import { FILE_EXTENSION, saveFile } from '../index.tsx'

export default function Nav(): ReactElement {
    const [textContent, setTextContent] = useContext(EditorContext)!.textContent;
    const [filename, setFilename] = useContext(EditorContext)!.fileName;
    const [filenameField, setFilenameField] = useState<string>(filename);
    const filenameInput = useRef<HTMLInputElement>(null);

    // Resize input with text
    const filenameFieldWidth = useMemo(() => {
        // Measure text size using temporary span element
        const temp = document.createElement("span");
        temp.style.whiteSpace = "pre";  // Don't trim whitespace
        temp.textContent = filenameField;
        document.body.appendChild(temp);
        const inputWidth = temp.getBoundingClientRect().width;
        document.body.removeChild(temp);
        return inputWidth;
    }, [filenameField]);

    const newButton = () => {
        setTextContent("");
        setFilename("Untitled");
        setFilenameField("Untitled");
    };

    const upload = () => {
        const input = document.createElement('input');
        input.type = 'file';
        input.multiple = false;
        input.accept = `.txt, .${FILE_EXTENSION}`;
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
            <input ref={filenameInput} type="text" style={{ width: filenameFieldWidth }} className="flush" value={filenameField}
                onInput={e => {
                    const text = (e.target as HTMLInputElement).value;
                    if (text === null) return;

                    setFilenameField(text);
                }}
                onKeyDown={e => {
                    const el = (e.target as HTMLInputElement)
                    const text = el.value;
                    if (text === null) return;

                    if (e.key === "Enter") {
                        e.preventDefault();
                        if (text.trim() !== "") {
                            setFilename(text);
                        }
                        setTimeout(() => el.blur());
                    }
                }}
                onBlur={() => setFilenameField(filename)}
            />
            <span onClick={() => {
                const input = filenameInput.current;
                if (input === null) return;

                input.focus()
                input.selectionStart = Number.MAX_SAFE_INTEGER;
                input.selectionEnd = Number.MAX_SAFE_INTEGER;
            }}>.{FILE_EXTENSION}</span>
        </div>
    </nav>
}