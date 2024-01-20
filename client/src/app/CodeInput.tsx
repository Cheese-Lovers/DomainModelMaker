import React, { useContext, useMemo, useRef } from "react";
import { ReactElement } from "react";
import "./codeInput.css"
import { EditorContext } from "./App.tsx";


export default function CodeInput(): ReactElement {
    const [textContent, setTextContent] = useContext(EditorContext)!.textContent;
    const ref = useRef<HTMLTextAreaElement>(null);

    const lineCount = useMemo(() => {
        return textContent.split('\n').length;
    }, [textContent]);

    // General idea taken from this article:
    // https://medium.com/weekly-webtips/enable-line-numbering-to-any-html-textarea-35e15ea320e2
    // and 'React-ified'.
    return <div className="code-input">
        <textarea ref={ref} className='flush line-counter' wrap='off' readOnly={true}
            value={Array(lineCount).fill(0).map((_, i) => `${i + 1}.`).join('\n')}>
        </textarea>
        <textarea className="flush" wrap="off" value={textContent}
            onInput={(e: React.KeyboardEvent<HTMLTextAreaElement>) => {
                // Handle tab input and update line count
                const codeEditor = e.target as HTMLTextAreaElement;
                let { value, selectionStart, selectionEnd } = codeEditor;
                if (e.key === "Tab") {
                    e.preventDefault();
                    // Input two spaces in place of tab
                    codeEditor.value = value.slice(0, selectionStart) + "  " + value.slice(selectionEnd);
                    codeEditor.setSelectionRange(selectionStart + 2, selectionStart + 2)
                }

                setTextContent(value);
            }}
            onScroll={e => {
                // Match scrolling between the line counter element and the code editor
                const codeEditor = e.target as HTMLTextAreaElement;
                if (ref.current !== null) {
                    ref.current.scrollTop = codeEditor.scrollTop;
                    ref.current.scrollLeft = codeEditor.scrollLeft;
                }
            }}
        ></textarea>
    </div>
}