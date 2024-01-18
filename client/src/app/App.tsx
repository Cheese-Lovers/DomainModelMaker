import React, { ReactElement } from "react";
import Nav from "./Nav.tsx";
import "./app.css";

export default function App(): ReactElement {
    return <>
        <Nav />
        <main>
            <div id="left-panel">
                <textarea></textarea>
            </div>
            <div></div>
        </main>
    </>
}