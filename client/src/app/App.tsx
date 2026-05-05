import React, { ReactElement, createContext, useEffect, useMemo, useState } from "react";
import Nav from "./Nav.tsx";
import "./app.css";
import CodeInput from "./CodeInput.tsx";
import { saveFile } from "../index.tsx";
import ImagePreview from "./ImagePreview.tsx";
import { defaultGraph, defaultPlacements, Graph, GridPlacements } from "./graph.ts";
import { generate_graph as generateGraph } from "../generator/server";

type Editor = {
    textContent: [string, (text: string) => void],
    fileName: [string, (text: string) => void],
    hideUI: [boolean, (hide: boolean) => void]
};

export const EditorContext = createContext<Editor | null>(null);
export const GraphContext = createContext<Graph | null>(null);
export const PlacementsContext = createContext<GridPlacements | null>(null);

export default function App(): ReactElement {
    const [textContent, setTextContent] = useState("");
    const [hideUI, setHideUI] = useState(false);
    const [fileName, setFileName] = useState("Untitled");
    const [graph, setGraph] = useState(defaultGraph());
    const [placements, setPlacements] = useState(defaultPlacements());

    const editor = useMemo(() => {
        return {
            textContent: [textContent, setTextContent],
            fileName: [fileName, setFileName],
            hideUI: [hideUI, setHideUI]
        } as Editor;
    }, [textContent, graph, fileName, hideUI]);

    useEffect(() => {
        const saveListener = (e: KeyboardEvent) => {
            if (e.ctrlKey && e.key === 's') {
                e.preventDefault();
                saveFile(textContent, fileName);
            }
        };

        globalThis.addEventListener('keydown', saveListener);
        return () => globalThis.removeEventListener('keydown', saveListener);
    }, [textContent, fileName]);

    useEffect(() => {
        const graphResult = generateGraph(textContent);
        console.log(JSON.stringify(graphResult));

        if ("Ok" in graphResult && graphResult.Ok) {
            setGraph(graphResult.Ok[0]);
            setPlacements(graphResult.Ok[1]);
        } else {
            // TODO: Popup (or something else?)
        }
    }, [textContent])

    return <EditorContext.Provider value={editor}>
        <PlacementsContext.Provider value={placements}>
            <GraphContext.Provider value={graph} >
                <Nav />
                <main>
                    <CodeInput />
                    <ImagePreview />
                </main>
            </GraphContext.Provider>
        </PlacementsContext.Provider>
    </EditorContext.Provider>
}