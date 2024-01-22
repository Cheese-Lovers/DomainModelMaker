import React, { ReactElement, createContext, useEffect, useState } from "react";
import Nav from "./Nav.tsx";
import "./app.css";
import CodeInput from "./CodeInput.tsx";
import { saveFile } from "../index.tsx";
import ImagePreview from "./ImagePreview.tsx";

type Editor = {
    textContent: [string, (text: string) => void],
    fileName: [string, (text: string) => void]
};

export const EditorContext = createContext<Editor | null>(null);

export default function App(): ReactElement {
    const editor = {
        textContent: useState(""),
        fileName: useState("Filename")
    };

    useEffect(() => {
        const saveListener = (e: KeyboardEvent) => {
            if (e.ctrlKey && e.key === 's') {
                e.preventDefault();
                saveFile(editor.textContent[0], editor.fileName[0]);
            }
        };

        window.addEventListener('keydown', saveListener);
        return () => window.removeEventListener('keydown', saveListener);
    });

    return <EditorContext.Provider value={editor}>
        <Nav />
        <main>
            <CodeInput />
            <ImagePreview />
        </main>
    </EditorContext.Provider>
}