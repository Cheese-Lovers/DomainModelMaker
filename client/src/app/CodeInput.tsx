import React, { useRef, useState } from "react";
import { ReactElement } from "react";
import "./codeInput.css"

export default function CodeInput(): ReactElement {
    const [lineCount, setLineCount] = useState<number>(1);
    const ref = useRef<HTMLTextAreaElement>(null);

    // General idea taken from this article:
    // https://medium.com/weekly-webtips/enable-line-numbering-to-any-html-textarea-35e15ea320e2
    // and 'React-ified'.
    return <div className="code-input">
        <textarea ref={ref} className='line-counter' wrap='off' readOnly={true}
            value={Array(lineCount).fill(0).map((_, i) => `${i + 1}.`).join('\n')}>
        </textarea>
        <textarea wrap="off"
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

                setLineCount(codeEditor.value.split('\n').length);
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